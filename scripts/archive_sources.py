"""Local reconstruction scope discovery. Never contacts a service or reads credentials."""
import json
import os
from pathlib import Path
import re
import subprocess

# Build caches are excluded, but dist, bin, images, state and checkpoints are data.
EXCLUDED = {'.git', '.preservation', '.venv', 'venv', 'node_modules', 'target',
            '__pycache__', '.pytest_cache', '.mypy_cache', '.ruff_cache', '.cache',
            '.DS_Store'}
SECRET_NAMES = {'auth.json', 'credentials.json', '.env', '.netrc', '.npmrc',
                '.pypirc', 'id_rsa', 'id_ed25519'}
PROJECT = re.compile(r'(?:claude-paint|paint-studio-|paint-r\d|paint-overnight-|'
                     r'paint-review-|gallery-fcf9c110|stillwet|r18-open-)')


def digest(path):
    import hashlib
    with Path(path).open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args], stderr=subprocess.DEVNULL)


def excluded(path, *, engine=False, runtime=False):
    parts = Path(path).parts
    names = EXCLUDED - {'target'} if engine else EXCLUDED
    if runtime:
        names = names - {'node_modules'}
    return any(p in names or p in SECRET_NAMES or p.startswith('.env.') for p in parts)


def selected_path(path, home):
    """Reject broad home/config/session roots even in an edited private plan."""
    try:
        rel = Path(path).absolute().relative_to(home)
    except ValueError:
        return False
    parts = rel.parts
    if '..' in parts or not parts:
        return False
    if parts[:3] == ('.pi', 'agent', 'sessions'):
        return len(parts) >= 4 and bool(PROJECT.search(parts[3]))
    if parts[:3] == ('.pi', 'agent', 'git'):
        return len(parts) >= 6 and parts[3:5] == ('github.com', 'aliceisjustplaying') and parts[5] in {
            'pi-black', 'pi-anthropic-compat', 'pi-codex-compaction', 'pi-batch-order'}
    if parts[:3] == ('.pi', 'agent', 'extensions'):
        return len(parts) == 4 and parts[3] in {
            'persistent-temp.ts', 'pi-anthropic-compat.ts', 'pi-codex-compaction.ts', 'pi-batch-order.ts'}
    if parts[:3] == ('.local', 'lib', 'node_modules'):
        return len(parts) >= 5 and parts[3] in {'@earendil-works', '@mariozechner'} and parts[4] == 'pi-coding-agent'
    if parts == ('.local', 'bin', 'pi'):
        return True
    if parts == ('.config', 'nix-darwin', 'modules', 'stillwet.nix'):
        return True
    if parts[:2] == ('src', 'a'):
        return len(parts) >= 3 and bool(PROJECT.match(parts[2]))
    return len(parts) >= 2 and parts[0] == 'tmp' and bool(PROJECT.match(parts[1]))


def walk(root, *, runtime=False):
    """Yield files/links without following directory links or enumerating caches."""
    root = Path(root)
    if root.is_symlink() or not root.is_dir():
        yield root
        return
    for base, dirs, files in os.walk(root, followlinks=False):
        for name in sorted(dirs):
            path = Path(base) / name
            if excluded(Path(name), runtime=runtime) or path.is_symlink():
                dirs.remove(name)
                yield path
        dirs.sort()
        for name in sorted(files):
            yield Path(base) / name


def index_entries(path):
    data = json.loads(Path(path).read_text())
    if isinstance(data, list):
        return {x['path']: x['sha256'] for x in data}
    return {name: value['sha256'] if isinstance(value, dict) else value
            for name, value in data.items()}


def discover(home, project):
    home, project = home.resolve(), project.resolve()
    roots = {}
    missing = []

    def add(path, reason):
        path = Path(path).absolute()
        if selected_path(path, home):
            roots.setdefault(str(path), set()).add(reason)
        else:
            missing.append({'path': str(path), 'reason': 'Outside project allowlist: ' + reason})

    add(project, 'Current project, including uncommitted and ignored meaningful files')
    for name in ('stillwet', 'stillwet-data'):
        add(project.parent / name, 'Website source, data and local static assets')
    for pattern in ('paint-studio-*', 'paint-r*', 'claude-paint-*'):
        for path in sorted(project.parent.glob(pattern)):
            if path.is_dir():
                add(path, 'Project studio or historical working checkout')
    add(home / 'tmp/gallery-fcf9c110', 'All runner rounds, outcomes, briefs and artifacts')
    sessions = home / '.pi/agent/sessions'
    if sessions.is_dir():
        for path in sorted(sessions.iterdir()):
            if path.is_dir() and PROJECT.search(path.name):
                add(path, 'Complete project session directory, including continuations')
    preservation = []
    preserved_links = {}
    for folder in sorted((project / '.preservation').glob('*')):
        scope = folder / 'scope.json'
        if not scope.is_file():
            continue
        for root in json.loads(scope.read_text()).get('roots', []):
            add(root, 'Preservation scope ' + folder.name)
        links = folder / 'symlinks.json'
        if links.is_file():
            preserved_links.update(json.loads(links.read_text()))
        for archive, inventory in [('records.tar.gz', 'files.json'),
                                   ('gallery-history.tar.gz', 'gallery-history-files.json'),
                                   ('additional-originals.tar.gz', 'additional-originals.json')]:
            tar, index = folder / archive, folder / inventory
            if tar.is_file() and index.is_file():
                preservation.append({'archive': str(tar), 'index': str(index),
                                     'index_sha256': digest(index)})
                for member in index_entries(index):
                    p = home / member
                    if selected_path(p, home) and not excluded(member, engine=member.endswith('/target/release/easel')):
                        # Catalog can select vanished external briefs/studios omitted from old scope.
                        add(p, 'Preservation inventory ' + folder.name + '/' + inventory)
    for name in ('pi-black', 'pi-anthropic-compat', 'pi-codex-compaction', 'pi-batch-order'):
        add(home / '.pi/agent/git/github.com/aliceisjustplaying' / name, 'Painter harness dependency source')
    for name in ('persistent-temp.ts', 'pi-anthropic-compat.ts', 'pi-codex-compaction.ts', 'pi-batch-order.ts'):
        add(home / '.pi/agent/extensions' / name, 'Explicit painter extension, no global configuration')
    add(home / '.config/nix-darwin/modules/stillwet.nix', 'Deployment routing source, not deployed state')
    for namespace in ('@earendil-works', '@mariozechner'):
        package = home / '.local/lib/node_modules' / namespace / 'pi-coding-agent'
        if package.is_dir():
            add(package, 'Installed Pi package and its nested dependencies, with package versions')
    if (home / '.local/bin/pi').is_symlink():
        add(home / '.local/bin/pi', 'Pi launcher link')

    # Read only project manifests and runner indexes, not arbitrary global session contents.
    candidates = set()
    for root in list(roots):
        path = Path(root)
        if path.is_file() and (path.name in {'manifest.json', 'studios.json'} or path.name.endswith('_sittings.json')):
            candidates.add(path)
        elif path.is_dir() and '.pi' not in path.parts:
            candidates.update(p for p in walk(path) if p.name in {'manifest.json', 'studios.json'} or p.name.endswith('_sittings.json'))
    for path in sorted(candidates):
        try:
            data = json.loads(path.read_text())
        except (ValueError, UnicodeError):
            missing.append({'path': str(path), 'reason': 'Could not parse selection index'})
            continue
        todo = [data]
        while todo:
            value = todo.pop()
            if isinstance(value, dict):
                todo.extend(value.values())
            elif isinstance(value, list):
                todo.extend(value)
            elif isinstance(value, str):
                for match in re.finditer(r'(?:~/|/Users/[^/\s]+/|/home/[^/\s]+/)[^\s\"\'<>;,]+', value):
                    ref = match.group().rstrip(').:')
                    ref = re.sub(r'^(?:~|/Users/[^/]+|/home/[^/]+)', str(home), ref)
                    ref = ref.replace('--Users-alice-', '--' + str(home).strip('/').replace('/', '-') + '-')
                    p = Path(ref)
                    if selected_path(p, home):
                        add(p, 'Reference in ' + str(path.relative_to(home)))
                        if p.suffix == '.jsonl':
                            add(p.parent, 'All sittings adjacent to explicit session reference')

    # Explicit pinned engine survives exclusion of build caches.
    for root in list(roots):
        p = Path(root)
        if p.is_dir() and (p / 'Cargo.toml').is_file():
            add(p / 'target/release/easel', 'Pinned executable; runtime dependencies require review')
    # Compact overlapping roots without dropping selection evidence.
    compact = []
    for root, reasons in sorted(roots.items(), key=lambda x: (len(Path(x[0]).parts), x[0])):
        p = Path(root)
        engine = p.parts[-3:] == ('target', 'release', 'easel')
        if not engine and any(p.is_relative_to(Path(x['path'])) for x in compact if not x['engine']):
            continue
        compact.append({'path': root, 'reasons': sorted(reasons), 'engine': engine})
    refs = {}
    refs_file = project / 'archive/collection/source-refs.json'
    if refs_file.is_file():
        refs.update(json.loads(refs_file.read_text()))
    refs.update({'r22.1-original-unavailable': '0e00118',
                 'r22.1-approved-substitute': 'f334aef11d0c069b083371d2cb8ceb4865664e83',
                 'rounds16-18': 'archive/r17-base', 'round19': 'round-19', 'round20': 'round-20'})
    repos = [x['path'] for x in compact if Path(x['path']).is_dir() and (Path(x['path']) / '.git').exists()]
    for repo in repos:
        try:
            refs['checkout/' + Path(repo).name] = git(repo, 'rev-parse', 'HEAD').decode().strip()
            if Path(repo) == project:
                for line in git(repo, 'for-each-ref', '--format=%(refname)',
                                'refs/tags/round-*', 'refs/tags/archive/*', 'refs/heads/preservation/*').decode().splitlines():
                    refs[line] = line
        except subprocess.CalledProcessError:
            pass
    return {'schema': 1, 'home': str(home), 'project': str(project), 'roots': compact,
            'preservation': preservation, 'source_refs': refs, 'repositories': repos,
            'preserved_links': preserved_links,
            'replacements': {}, 'discovery_notes': missing,
            'site': 'Local working source/data and local dist, not a verified deployed or Wayback capture',
            'engine_policy': 'r22.1: original 0e00118 unavailable; approved substitute f334aef. Final-output receipt does not establish intermediate-state identity.'}
