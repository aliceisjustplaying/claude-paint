#!/usr/bin/env python3
"""Plan and prepare a redacted reconstruction copy; upload only after explicit review."""
import argparse
import base64
import binascii
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import posixpath
import re
import shutil
import subprocess
import tarfile
import tempfile

from archive_sources import discover, digest, excluded, git, index_entries, selected_path, walk

PRIVATE = re.compile(r's[a]rah', re.I)
PRIVATE_BYTES = re.compile(rb's[a]rah', re.I)
PRIVATE_WIDE = re.compile(rb's\x00a\x00r\x00a\x00h|\x00s\x00a\x00r\x00a\x00h', re.I)
HOME_TOKEN = '/ARCHIVE_HOME'
ENCODED = re.compile(r'data:[^\s;,]+;base64,[A-Za-z0-9+/]{4,}={0,2}')
ENCODED_VALUE = re.compile(r'[A-Za-z0-9+/]{64,}={0,2}')
TEXT_SUFFIXES = {'.json', '.jsonl', '.md', '.txt', '.py', '.sh', '.rs', '.toml', '.lock',
                 '.lua', '.ts', '.js', '.html', '.css', '.svg', '.nix', '.yaml', '.yml', '.csv'}
OPAQUE_SUFFIXES = {'.gz', '.zip', '.xz', '.bz2', '.7z', '.zst', '.pdf', '.png', '.jpg',
                   '.jpeg', '.webp', '.mp4', '.mov', '.woff', '.woff2'}


def json_bytes(value):
    return (json.dumps(value, ensure_ascii=False, indent=2) + '\n').encode()


def safe_name(name):
    p = PurePosixPath(name)
    if not name or p.is_absolute() or '..' in p.parts or str(p) != name or '\\' in name:
        raise ValueError('Unsafe archive path')
    if PRIVATE.search(name) or '.git' in p.parts or any(ord(c) < 32 for c in name):
        raise ValueError('Private or unsupported archive path')
    return name


def link_destination(name, target):
    if target.startswith('/') or '\\' in target:
        raise ValueError('Absolute or unsupported archive link')
    return safe_name(posixpath.normpath(posixpath.join(posixpath.dirname(name), target)))


def strings(value, transform):
    if isinstance(value, str):
        return transform(value)
    if isinstance(value, list):
        return [strings(x, transform) for x in value]
    if isinstance(value, dict):
        result = {}
        for key, item in value.items():
            new = transform(key)
            if new in result:
                raise ValueError('Redaction creates duplicate JSON keys')
            encoded = (key in {'signature', 'thinkingSignature', 'encrypted_content'} or
                       (key == 'data' and (value.get('type') in {'image', 'base64'} or
                                          'mimeType' in value or 'media_type' in value)))
            if encoded and isinstance(item, str) and hasattr(transform, 'encoded'):
                result[new] = transform.encoded(item)
            else:
                result[new] = strings(item, transform)
        return result
    return value


class Redactor:
    def __init__(self, home):
        self.homes = sorted({str(home), '/Users/alice', '/home/alice'}, key=len, reverse=True)

    def encoded(self, value):
        try:
            decoded = base64.b64decode(re.sub(r'\s+', '', value.split(';base64,', 1)[-1]),
                                       altchars=b'-_', validate=True)
        except (ValueError, binascii.Error) as error:
            if PRIVATE.search(value):
                raise ValueError('Identity-like bytes in opaque payload require separate review') from error
            return value  # Opaque signatures are retained unchanged for manual review.
        if PRIVATE_BYTES.search(decoded) or PRIVATE_WIDE.search(decoded):
            raise ValueError('Protected identity in decoded base64; separate reviewed replacement needed')
        return value

    def __call__(self, text):
        protected = {}

        def protect(match):
            value = match.group()
            encoded = value.split(';base64,', 1)[-1]
            try:
                decoded = base64.b64decode(encoded, validate=True)
            except (ValueError, binascii.Error):
                return value
            if PRIVATE_BYTES.search(decoded) or PRIVATE_WIDE.search(decoded):
                raise ValueError('Protected identity in decoded base64; separate reviewed replacement needed')
            # Never rewrite encoded bytes, including accidental identity-like base64 characters.
            marker = '\x00ARCHIVE_BINARY_' + str(len(protected)) + '\x00'
            while marker in text:
                marker += '_'
            protected[marker] = value
            return marker

        whole = ENCODED_VALUE.fullmatch(text)
        text = protect(whole) if whole and not text.startswith(tuple(self.homes)) else ENCODED.sub(protect, text)
        for home in self.homes:
            text = text.replace(home, HOME_TOKEN)
            text = text.replace(home.strip('/').replace('/', '-'), 'ARCHIVE_HOME')
        text = PRIVATE.sub('alice', text)
        for marker, value in protected.items():
            text = text.replace(marker, value)
        return text

    def member(self, path):
        return safe_name(self(str(path)))


def transform_stream(source, destination, name, transform):
    """JSON values are decoded before redaction; complete JSONL records are retained."""
    suffix = Path(name).suffix.lower()
    source.seek(0)
    sample = source.read(8192)
    source.seek(0)
    try:
        sample.decode('utf-8')
        likely_text = b'\x00' not in sample and suffix not in OPAQUE_SUFFIXES
    except UnicodeError:
        likely_text = False
    if suffix in TEXT_SUFFIXES or likely_text:
        try:
            wrapper = io.TextIOWrapper(source, encoding='utf-8', newline='')
            try:
                if suffix == '.json':
                    serialized = json.dumps(strings(json.load(wrapper), transform), ensure_ascii=False, indent=2)
                    # Escaping the first character preserves JSON/base64 values without a raw-name leak.
                    serialized = PRIVATE.sub(lambda m: '\\u%04x' % ord(m[0][0]) + m[0][1:], serialized)
                    destination.write((serialized + '\n').encode())
                elif suffix == '.jsonl':
                    for line in wrapper:
                        if line.strip():
                            serialized = json.dumps(strings(json.loads(line), transform), ensure_ascii=False)
                            serialized = PRIVATE.sub(lambda m: '\\u%04x' % ord(m[0][0]) + m[0][1:], serialized)
                            destination.write((serialized + '\n').encode())
                        else:
                            destination.write(line.encode())
                else:
                    for line in wrapper:
                        destination.write(transform(line).encode())
            finally:
                wrapper.detach()
            return 'text'
        except (UnicodeError, ValueError) as error:
            # Malformed logs must not silently lose entries or evade decoded checks.
            if suffix in {'.json', '.jsonl'} or not isinstance(error, UnicodeError):
                raise ValueError('Content transformation blocked: ' + str(error)) from error
            destination.seek(0)
            destination.truncate()
            source.seek(0)
    shutil.copyfileobj(source, destination)
    return 'binary'


def scan(stream):
    checksum = hashlib.sha256()
    tail = b''
    size = 0
    while chunk := stream.read(1024 * 1024):
        if PRIVATE_BYTES.search(tail + chunk) or PRIVATE_WIDE.search(tail + chunk):
            raise ValueError('Protected identity in binary/raw bytes; rebuild or supply a reviewed replacement')
        checksum.update(chunk)
        size += len(chunk)
        tail = chunk[-32:]
    return checksum.hexdigest(), size


class Candidate:
    def __init__(self, archive, plan):
        self.archive, self.plan = archive, plan
        self.home = Path(plan['home'])
        self.redact = Redactor(self.home)
        self.files = {}
        self.origins = {}
        self.raw_origins = {}
        self.issues = []
        self.exclusions = []
        self.refs = []
        self.runtime = []
        self.runtime_seen = set()

    def runtime_dependencies(self, path, logical_path=None):
        """Inspect load commands, never execute archived programs or use ldd."""
        resolved = path.resolve()
        logical = logical_path or resolved
        if str(resolved) in self.runtime_seen:
            return
        self.runtime_seen.add(str(resolved))
        with resolved.open('rb') as stream:
            magic = stream.read(4)
        macho = magic in {b'\xcf\xfa\xed\xfe', b'\xce\xfa\xed\xfe', b'\xca\xfe\xba\xbe', b'\xfe\xed\xfa\xcf'}
        elf = magic == b'\x7fELF'
        if not (macho or elf):
            return
        tool = shutil.which('otool' if macho else 'readelf')
        receipt = {'path': self.redact(str(logical)), 'format': 'Mach-O' if macho else 'ELF',
                   'dependencies': [], 'note': 'Platform-specific binary; rebuild from matching source for another OS/architecture. System libraries are not redistributed.'}
        self.runtime.append(receipt)
        if not tool:
            receipt['status'] = 'Static dependency inspector unavailable; manual review required'
            return
        result = subprocess.run([tool, '-L' if macho else '-d', str(resolved)], capture_output=True, text=True)
        receipt['inspection'] = self.redact((result.stdout + result.stderr).replace(str(resolved), str(logical)))
        if result.returncode:
            receipt['status'] = 'Dependency inspection failed; manual review required'
            return
        dependencies = ([line.strip().split(' (', 1)[0] for line in result.stdout.splitlines()[1:] if line.strip()]
                        if macho else re.findall(r'\(NEEDED\).*?\[(.*?)\]', result.stdout))
        for dep in dependencies:
            record = {'reference': self.redact(dep)}
            receipt['dependencies'].append(record)
            if dep.startswith(('/usr/lib/', '/System/Library/')):
                record['status'] = 'OS-provided'
                continue
            concrete = dep.replace('@loader_path', str(logical.parent)).replace('@executable_path', str(logical.parent))
            library = Path(concrete)
            if (library.is_absolute() and library.is_file() and
                (str(library).startswith(('/nix/store/', '/opt/homebrew/', '/usr/local/')) or selected_path(library, self.home)) and
                (library.suffix == '.dylib' or '.so' in library.name)):
                name = 'runtime/' + str(library).lstrip('/')
                with library.open('rb') as stream:
                    self.add(stream, name, str(library), library.stat().st_mode)
                record.update(status='copied; loader relocation may be required', path=self.redact(name))
                self.runtime_dependencies(library)
            else:
                record['status'] = 'Unresolved loader search path; install dependency or rebuild before replay'

    def link(self, name, target, origin):
        name, target = self.redact.member(name), self.redact.member(target)
        relative = posixpath.relpath(target, posixpath.dirname(name))
        link_destination(name, relative)
        if name in self.files:
            if self.files[name].get('target') != relative or self.raw_origins[name] != origin:
                raise ValueError('Link path collision: ' + name)
            return
        info = tarfile.TarInfo(name)
        info.type, info.linkname, info.mode = tarfile.SYMTYPE, relative, 0o755
        self.archive.addfile(info)
        checksum = hashlib.sha256(relative.encode()).hexdigest()
        self.files[name] = {'path': name, 'type': 'symlink', 'target': relative, 'mode': 0o755}
        self.raw_origins[name] = origin
        self.origins[name] = {'path': name, 'source': self.redact(origin),
                              'original_sha256': checksum, 'kind': 'symlink', 'target': relative}

    def issue(self, path, reason, blocking=False):
        self.issues.append({'path': self.redact(str(path)), 'reason': reason, 'blocking': blocking})

    def add(self, stream, name, origin, mode=0o644, expected=None, replacement=None, original_source_hash=None):
        name = self.redact.member(name)
        # Spooling keeps large images/videos off the Python heap. Temp files are deleted.
        with tempfile.TemporaryFile() as raw, tempfile.TemporaryFile() as public:
            checksum = hashlib.sha256()
            while chunk := stream.read(1024 * 1024):
                raw.write(chunk)
                checksum.update(chunk)
            original = checksum.hexdigest()
            if expected and original != expected:
                raise ValueError('Preservation inventory checksum mismatch')
            # Recovered or historical binaries can use the same explicit replacement policy.
            alternative = self.plan.get('replacements', {}).get(origin) if replacement is None else None
            if alternative:
                substitute = Path(alternative)
                if not substitute.is_file() or substitute.is_symlink():
                    raise ValueError('Replacement must be an explicit regular file')
                original_source_hash, replacement = original, str(substitute)
                raw.seek(0)
                raw.truncate()
                with substitute.open('rb') as replacement_stream:
                    shutil.copyfileobj(replacement_stream, raw)
                original = digest(substitute)
            if name in self.files:
                if (self.origins[name]['original_sha256'] != (original_source_hash or original)
                    or self.raw_origins[name] != origin):
                    raise ValueError('Two source paths map to the same public path: ' + name)
                return
            try:
                kind = transform_stream(raw, public, name, self.redact)
                public.seek(0)
                checksum_public, size = scan(public)
            except ValueError as error:
                self.issue(name, str(error), blocking=True)
                return
            mode = 0o755 if mode & 0o111 else 0o644
            info = tarfile.TarInfo(name)
            info.size, info.mode = size, mode
            public.seek(0)
            self.archive.addfile(info, public)
            self.files[name] = {'path': name, 'type': 'file', 'bytes': size, 'sha256': checksum_public, 'mode': mode}
            self.raw_origins[name] = origin
            self.origins[name] = {'path': name, 'source': self.redact(origin), 'original_sha256': original_source_hash or original,
                                  'public_sha256': checksum_public, 'kind': kind,
                                  'redacted_or_reserialized': original != checksum_public,
                                  'replacement': self.redact(replacement) if replacement else None}
            if replacement:
                self.origins[name]['replacement_sha256'] = original
            if kind == 'binary' or Path(name).suffix.lower() in OPAQUE_SUFFIXES:
                self.origins[name]['manual_review'] = 'Media, binary or encoded content; raw byte scan is not a full privacy audit'

    def add_local(self, path, engine=False, runtime=False):
        rel = path.relative_to(self.home)
        if excluded(rel, engine=engine, runtime=runtime):
            self.exclusions.append({'path': self.redact(str(rel)), 'reason': 'Cache, Git internals or credential filename'})
            return
        name = 'home/' + str(rel)
        public_name = self.redact.member(name)
        if not path.exists() and not path.is_symlink():
            self.issue(name, 'Missing current source; preservation recovery attempted later')
            return
        if path.is_symlink():
            target = path.resolve()
            # Never follow a link into global credentials/config or outside scoped sources.
            if not target.exists() or not selected_path(target, self.home) or not any(
                target.is_relative_to(Path(x['path'])) for x in self.plan['roots']
            ) or excluded(target.relative_to(self.home), engine=engine, runtime=runtime or rel == Path('.local/bin/pi')):
                self.issue(name, 'Unresolved or out-of-scope symlink; explicit source/replacement needed', True)
                return
            self.link(name, 'home/' + str(target.relative_to(self.home)), str(path))
            return
        if not path.is_file():
            self.issue(name, 'Special file or directory link cannot be preserved', True)
            return
        source = Path(self.plan.get('replacements', {}).get(str(path), str(path)))
        if source != path and (not source.is_file() or source.is_symlink()):
            raise ValueError('Replacement must be an explicit regular file')
        before = path.stat()
        with source.open('rb') as stream:
            self.add(stream, name, str(path), before.st_mode,
                     replacement=str(source) if source != path else None,
                     original_source_hash=digest(path) if source != path else None)
        after = path.stat()
        if (before.st_size, before.st_mtime_ns, before.st_ino) != (after.st_size, after.st_mtime_ns, after.st_ino):
            raise ValueError('Source changed during preparation; retry with painters quiescent')
        if public_name in self.files and self.origins[public_name].get('kind') == 'binary' and (
            engine or path.parent.name == 'bin' or before.st_mode & 0o111
        ):
            self.runtime_dependencies(source)

    def recover(self, entry):
        index = Path(entry['index'])
        if digest(index) != entry['index_sha256']:
            raise ValueError('Preservation index changed since scope planning')
        inventory = index_entries(index)
        seen = set()
        label = Path(entry['archive']).parent.name + '-' + Path(entry['archive']).name.removesuffix('.tar.gz')
        with tarfile.open(entry['archive'], 'r|gz') as old:
            for member in old:
                rel = PurePosixPath(member.name)
                if rel.is_absolute() or '..' in rel.parts:
                    raise ValueError('Unsafe preservation member')
                name = str(rel)
                engine = name.endswith('/target/release/easel')
                if name not in inventory or not selected_path(self.home / name, self.home) or excluded(name, engine=engine):
                    self.exclusions.append({'path': self.redact(name), 'reason': 'Preservation member outside selected project data or excluded internals'})
                    continue
                seen.add(name)
                destination = self.redact.member('home/' + name)
                if destination in self.files:
                    if self.origins[destination]['original_sha256'] == inventory[name]:
                        continue
                    destination = 'history/' + label + '/' + destination
                if not member.isfile():
                    self.issue(destination, 'Preservation link/special member needs explicit recovery', True)
                    continue
                with old.extractfile(member) as stream:
                    if engine or member.mode & 0o111 or PurePosixPath(name).parent.name == 'bin':
                        with tempfile.NamedTemporaryFile() as executable:
                            shutil.copyfileobj(stream, executable)
                            executable.flush()
                            executable.seek(0)
                            self.add(executable, destination, 'preservation/' + label + '/' + name,
                                     member.mode, expected=inventory[name])
                            self.runtime_dependencies(Path(executable.name), self.home / name)
                    else:
                        self.add(stream, destination, 'preservation/' + label + '/' + name,
                                 member.mode, expected=inventory[name])
        for name in inventory.keys() - seen:
            if selected_path(self.home / name, self.home) and not excluded(name, engine=name.endswith('/target/release/easel')):
                self.issue(name, 'Inventory entry absent from preservation archive', True)

    def snapshots(self):
        exported = set()
        for label, ref in self.plan['source_refs'].items():
            receipt = {'label': self.redact(label), 'requested_ref': ref, 'status': 'unavailable'}
            for repo in self.plan['repositories']:
                try:
                    commit = git(repo, 'rev-parse', '--verify', '--end-of-options', ref + '^{commit}').decode().strip()
                except subprocess.CalledProcessError:
                    continue
                receipt.update(commit=commit, repository=self.redact(repo), status='exported',
                               path='sources/' + commit)
                if commit not in exported:
                    # Temporary stream avoids creating another working tree or retaining .git.
                    with tempfile.TemporaryFile() as archive:
                        subprocess.run(['git', '-C', repo, 'archive', '--format=tar', commit],
                                       stdout=archive, stderr=subprocess.DEVNULL, check=True)
                        archive.seek(0)
                        with tarfile.open(fileobj=archive, mode='r|') as snapshot:
                            for member in snapshot:
                                if excluded(member.name):
                                    continue
                                if member.isfile():
                                    with snapshot.extractfile(member) as stream:
                                        self.add(stream, 'sources/' + commit + '/' + member.name,
                                                 'git/' + commit + '/' + member.name, member.mode)
                                elif member.issym():
                                    target = link_destination(member.name, member.linkname)
                                    self.link('sources/' + commit + '/' + member.name,
                                              'sources/' + commit + '/' + target, 'git/' + commit + '/' + member.name)
                                elif not member.isdir():
                                    self.issue('sources/' + commit + '/' + member.name,
                                               'Git snapshot special member requires manual restoration', True)
                    exported.add(commit)
                break
            if receipt['status'] == 'unavailable':
                self.issue('source-ref/' + label, 'Unavailable historical source ' + ref + '; see engine provenance')
            self.refs.append(receipt)


def plan_command(args):
    plan = discover(args.home, args.project)
    # Plan is private: exact paths are needed for reading sources later.
    with os.fdopen(os.open(args.plan, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600), 'w', encoding='utf-8') as out:
        out.write(json_bytes(plan).decode())
    print('Private scope plan written; no data packaged and no network operation performed.')
    print('Scope SHA-256: ' + digest(args.plan))


def prepare(args):
    if digest(args.plan) != args.scope_sha256:
        raise ValueError('Reviewed scope digest does not match')
    plan = json.loads(args.plan.read_text())
    home = Path(plan['home']).resolve()
    if plan['schema'] != 1 or str(home) != plan['home']:
        raise ValueError('Unsupported or noncanonical scope plan')
    for entry in plan['roots']:
        if not selected_path(Path(entry['path']), home):
            raise ValueError('Scope contains a broad or unrelated private root')
    for repo in plan['repositories']:
        if not selected_path(Path(repo), home):
            raise ValueError('Repository outside project scope')
    for entry in plan['preservation']:
        for key in ('archive', 'index'):
            if not Path(entry[key]).resolve().is_relative_to(Path(plan['project']) / '.preservation'):
                raise ValueError('Preservation input outside project preservation directory')
    dest = args.directory.resolve()
    if any(dest.is_relative_to(Path(x['path'])) for x in plan['roots']):
        raise ValueError('Candidate directory must be outside all selected roots')
    dest.mkdir(mode=0o700, parents=True, exist_ok=False)
    payload = dest / 'reconstruction.tar.gz'
    try:
        with tarfile.open(payload, 'w:gz') as archive:
            candidate = Candidate(archive, plan)
            for entry in plan['roots']:
                runtime = Path(entry['path']).relative_to(home).parts[:3] == ('.local', 'lib', 'node_modules')
                for path in walk(entry['path'], runtime=runtime):
                    candidate.add_local(path, entry['engine'], runtime=runtime)
            for entry in reversed(plan['preservation']):
                candidate.recover(entry)
            for name, target in plan.get('preserved_links', {}).items():
                original = home / name
                destination = candidate.redact.member('home/' + name)
                if destination in candidate.files or original.exists() or excluded(name):
                    continue
                resolved = Path(os.path.normpath(str(original.parent / target)))
                if selected_path(original, home) and selected_path(resolved, home):
                    candidate.link('home/' + name, 'home/' + str(resolved.relative_to(home)), 'preserved-link/' + name)
            candidate.snapshots()
            for name in ('archive_sources.py', 'archive_upload.py'):
                with Path(__file__).with_name(name).open('rb') as stream:
                    candidate.add(stream, 'tools/' + name, 'preparation-tool/' + name)
            guide = Path(__file__).resolve().parents[1] / 'archive/INTERNET_ARCHIVE.md'
            if guide.is_file():
                with guide.open('rb') as stream:
                    candidate.add(stream, 'tools/INTERNET_ARCHIVE.md', 'preparation-guide')
        # A recovered missing file resolves the missing-current notice, without hiding its provenance.
        for issue in candidate.issues:
            if issue['reason'].startswith('Missing current'):
                path = issue['path']
                if path in candidate.files or any(x.startswith(path + '/') for x in candidate.files):
                    issue['reason'] = 'Current source absent; recovered from preservation'
        sources = {'schema': 1, 'scope_sha256': args.scope_sha256,
                   'selection': strings(plan, candidate.redact), 'files': list(candidate.origins.values()),
                   'source_refs': candidate.refs, 'runtime': candidate.runtime,
                   'issues': candidate.issues, 'exclusions': candidate.exclusions,
                   'portability': 'home/ restores under an isolated HOME; /ARCHIVE_HOME and encoded session names are rewritten. Binary runtime dependencies and embedded paths need review.'}
        # Exact replacement and preservation source paths are sanitized above; no private plan uploaded.
        raw = json_bytes(sources)
        if PRIVATE_BYTES.search(raw):
            raise ValueError('Identity remains in public provenance')
        (dest / 'sources.json').write_bytes(raw)
        metadata = {'title': 'Still Wet / claude-paint reconstruction collection',
                    'mediatype': 'software', 'collection': 'open_source_software',
                    'creator': 'aliceisjustplaying',
                    'description': 'Selected redacted reconstruction derivative: current website source/data and local static assets, complete selected painter sessions, studios, runners, historical sources and preserved artifacts. Completeness, exclusions, substitutions and review limits are recorded in sources.json. Local static assets are not a verified deployed capture or Wayback capture. Original project code: MIT; project paintings, logs and texts: CC BY 4.0. Third-party components retain their own notices.',
                    'source': 'https://github.com/aliceisjustplaying/claude-paint'}
        manifest = {'schema': 1, 'metadata': metadata, 'archive_sha256': digest(payload),
                    'sources_sha256': digest(dest / 'sources.json'),
                    'blockers': [x for x in candidate.issues if x['blocking']],
                    'files': sorted(candidate.files.values(), key=lambda x: x['path'])}
        (dest / 'manifest.json').write_bytes(json_bytes(manifest))
        verify(dest)
        print(f'Prepared {len(candidate.files)} files; {len(manifest["blockers"])} upload blockers.')
        print('Manifest review SHA-256: ' + digest(dest / 'manifest.json'))
        print('Nothing uploaded. Review provenance, omissions, binary dependencies, secrets, media and encoded content.')
    except Exception:
        payload.unlink(missing_ok=True)
        (dest / 'manifest.json').unlink(missing_ok=True)
        raise


def inspect(path):
    entries = []
    seen = set()
    with tarfile.open(path, 'r|gz') as archive:
        for member in archive:
            name = safe_name(member.name)
            if name in seen or not (member.isfile() or member.issym()):
                raise ValueError('Duplicate or unsupported archive member')
            seen.add(name)
            if member.issym():
                link_destination(name, member.linkname)
                entries.append({'path': name, 'type': 'symlink', 'target': member.linkname, 'mode': member.mode})
                continue
            with archive.extractfile(member) as stream:
                checksum, size = scan(stream)
            if member.mode not in (0o644, 0o755):
                raise ValueError('Unexpected archive permissions')
            entries.append({'path': name, 'type': 'file', 'bytes': size, 'sha256': checksum, 'mode': member.mode})
    for entry in entries:
        if any(str(parent) in seen for parent in PurePosixPath(entry['path']).parents if str(parent) != '.'):
            raise ValueError('File or link is an ancestor of another member')
        if entry['type'] == 'symlink':
            target = link_destination(entry['path'], entry['target'])
            if target == entry['path'] or entry['path'].startswith(target + '/'):
                raise ValueError('Recursive archive link')
    links = {x['path']: x['target'] for x in entries if x['type'] == 'symlink'}
    directories = {str(parent) for name in seen for parent in PurePosixPath(name).parents}
    for name, relative in links.items():
        target = link_destination(name, relative)
        visited = {name}
        while True:
            ancestors = [target, *(str(p) for p in PurePosixPath(target).parents)]
            link = next((p for p in ancestors if p in links), None)
            if link is None:
                break
            if link in visited:
                raise ValueError('Cyclic archive links')
            visited.add(link)
            target = link_destination(link, links[link]) + target[len(link):]
        if target not in seen and target not in directories:
            raise ValueError('Archive link target is missing: ' + name)
    return sorted(entries, key=lambda x: x['path'])


def verify(directory, reviewed=None):
    manifest_path = directory / 'manifest.json'
    if reviewed is not None and digest(manifest_path) != reviewed:
        raise ValueError('Reviewed manifest digest does not match')
    raw = manifest_path.read_bytes()
    manifest = json.loads(raw)
    sources_raw = (directory / 'sources.json').read_bytes()
    # Decode escaped JSON strings as well as scanning literal bytes.
    for data in (manifest, json.loads(sources_raw)):
        if PRIVATE.search(json.dumps(data, ensure_ascii=False)):
            raise ValueError('Identity in public metadata/provenance')
    if manifest['schema'] != 1 or digest(directory / 'reconstruction.tar.gz') != manifest['archive_sha256']:
        raise ValueError('Archive changed since preparation')
    if digest(directory / 'sources.json') != manifest['sources_sha256']:
        raise ValueError('Provenance changed since preparation')
    if inspect(directory / 'reconstruction.tar.gz') != manifest['files']:
        raise ValueError('Archive inventory does not match manifest')
    return manifest


def restore(args):
    verify(args.directory)
    dest = args.destination.resolve()
    dest.mkdir(mode=0o700, parents=True, exist_ok=False)
    restored_home = dest / 'home'
    encoded_home = str(restored_home).strip('/').replace('/', '-')

    def relocate(text):
        return text.replace(HOME_TOKEN, str(restored_home)).replace('ARCHIVE_HOME', encoded_home)

    relocate.encoded = lambda value: value

    try:
        links = []
        with tarfile.open(args.directory / 'reconstruction.tar.gz', 'r|gz') as archive:
            for member in archive:
                # Links are created last, so no write can follow an extracted symlink.
                name = safe_name(member.name).replace('ARCHIVE_HOME', encoded_home)
                path = dest / name
                if not path.is_relative_to(dest) or path.exists():
                    raise ValueError('Restoration path collision')
                path.parent.mkdir(parents=True, exist_ok=True)
                if member.issym():
                    links.append((path, member.linkname.replace('ARCHIVE_HOME', encoded_home)))
                    continue
                with archive.extractfile(member) as stream, tempfile.TemporaryFile() as raw:
                    shutil.copyfileobj(stream, raw)
                    with path.open('xb') as out:
                        if name.startswith('tools/'):
                            raw.seek(0)
                            shutil.copyfileobj(raw, out)
                        else:
                            transform_stream(raw, out, name, relocate)
                path.chmod(member.mode)
        for path, target in links:
            path.symlink_to(target)
        shutil.copy2(args.directory / 'sources.json', dest / 'sources.json')
        shutil.copy2(args.directory / 'manifest.json', dest / 'manifest.json')
        print('Restored into a new isolated tree. HOME for project tools: ' + str(restored_home))
        print('Restored text differs from the public checksums by path relocation; binaries are unchanged.')
    except Exception:
        print('Restoration incomplete; the new destination is retained for inspection.')
        raise


def upload(args):
    if not args.confirm_reviewed_content or not args.confirm_public_upload:
        raise ValueError('Both content review and public publication require explicit confirmation flags')
    data = verify(args.directory, args.reviewed_sha256)
    if data['blockers']:
        raise ValueError('Candidate has unresolved binary/privacy/recovery blockers')
    if not re.fullmatch(r'[a-zA-Z0-9][a-zA-Z0-9._-]{4,99}', args.identifier) or PRIVATE.search(args.identifier):
        raise ValueError('Invalid or private public identifier')
    import importlib.metadata
    if importlib.metadata.version('internetarchive') != '5.11.1':
        raise ValueError('Upload requires internetarchive==5.11.1')
    import internetarchive
    session = internetarchive.get_session()
    if not session.access_key or not session.secret_key:
        raise ValueError('Internet Archive credentials are not configured')
    item = session.get_item(args.identifier)
    if item.exists:
        raise ValueError('Item already exists; refusing to modify it, including partial uploads')
    responses = item.upload(files={name: str(args.directory / name) for name in
                                  ('reconstruction.tar.gz', 'manifest.json', 'sources.json')},
                            metadata=data['metadata'], verify=True, checksum=True)
    for response in responses:
        response.raise_for_status()
    print('Upload requests succeeded: https://archive.org/details/' + args.identifier)
    print('Processing and public download availability still need separate verification.')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='action', required=True)
    planning = sub.add_parser('plan', help='Discover selected local sources; write a PRIVATE scope file')
    planning.add_argument('plan', type=Path)
    planning.add_argument('--home', type=Path, default=Path.home())
    planning.add_argument('--project', type=Path, required=True)
    prep = sub.add_parser('prepare', help='Prepare a redacted local candidate from the reviewed scope')
    prep.add_argument('plan', type=Path)
    prep.add_argument('directory', type=Path)
    prep.add_argument('--scope-sha256', required=True)
    check = sub.add_parser('verify', help='Check local checksums and raw identity scan, without network access')
    check.add_argument('directory', type=Path)
    recover = sub.add_parser('restore', help='Restore into a NEW isolated directory, never the original sources')
    recover.add_argument('directory', type=Path)
    recover.add_argument('destination', type=Path)
    send = sub.add_parser('upload', help='PUBLIC upload after explicit content review')
    send.add_argument('directory', type=Path)
    send.add_argument('--identifier', required=True)
    send.add_argument('--reviewed-sha256', required=True)
    send.add_argument('--confirm-reviewed-content', action='store_true')
    send.add_argument('--confirm-public-upload', action='store_true')
    args = parser.parse_args()
    try:
        {'plan': plan_command, 'prepare': prepare, 'verify': lambda a: verify(a.directory),
         'restore': restore, 'upload': upload}[args.action](args)
    except Exception as error:
        parser.exit(1, 'Archive operation failed: ' + str(error) + '\n')


if __name__ == '__main__':
    main()
