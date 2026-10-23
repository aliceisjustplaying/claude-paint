"""Tiny synthetic CLI contracts; never reads the real project or uploads."""
import hashlib
import base64
import io
import json
import os
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import unittest

SCRIPT = Path(__file__).with_name('archive_upload.py')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


class ReconstructionCLI(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='archive-fixture-')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve()
        self.identity = 's' + 'arah'
        self.home = self.root / self.identity
        self.project = self.home / 'src/a/claude-paint'
        self.project.mkdir(parents=True)
        self.plan = self.root / 'scope.json'
        self.candidate = self.root / 'candidate'

    def write(self, relative, data):
        path = self.home / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data if isinstance(data, bytes) else data.encode())
        return path

    def cli(self, *args, success=True):
        result = subprocess.run([sys.executable, str(SCRIPT), *map(str, args)],
                                capture_output=True, text=True)
        self.assertEqual(result.returncode == 0, success, result.stdout + result.stderr)
        return result

    def plan_scope(self):
        self.cli('plan', self.plan, '--home', self.home, '--project', self.project)
        data = json.loads(self.plan.read_text())
        # The fixture has no historical Git repository. These are ordinary editable plan fields.
        data['source_refs'] = {}
        data['repositories'] = []
        self.plan.write_text(json.dumps(data))
        return data

    def prepare(self):
        self.cli('prepare', self.plan, self.candidate, '--scope-sha256', sha(self.plan))
        return json.loads((self.candidate / 'manifest.json').read_text())

    def test_painting_repositories_and_source_refs_are_discovered_outside_the_engine(self):
        refs = {'historical-engine': '1234567890abcdef'}
        self.write('src/a/stillwet-paintings/other/claude-paint/source-refs.json', json.dumps(refs))
        self.write('src/a/stillwet-paintings-public/README.md', 'Public painting records')
        self.cli('plan', self.plan, '--home', self.home, '--project', self.project)
        data = json.loads(self.plan.read_text())
        self.assertEqual(data['source_refs']['historical-engine'], refs['historical-engine'])
        roots = {entry['path'] for entry in data['roots']}
        self.assertIn(str(self.home / 'src/a/stillwet-paintings'), roots)
        self.assertIn(str(self.home / 'src/a/stillwet-paintings-public'), roots)

    def test_committed_typescript_config_is_ready_and_restores(self):
        relative = 'experiments/watercolor/lab-app/tsconfig.json'
        committed = SCRIPT.parent.parent / relative
        original = committed.read_bytes()
        config = self.write('src/a/claude-paint/' + relative, original)
        self.plan_scope()
        manifest = self.prepare()
        self.assertEqual(manifest['blockers'], [])
        self.cli('verify', self.candidate)
        self.cli('restore', self.candidate, self.root / 'restored')
        restored = self.root / 'restored/home/src/a/claude-paint' / relative
        data = json.loads(restored.read_text())
        self.assertIs(data['compilerOptions']['noUncheckedIndexedAccess'], False)
        self.assertEqual(data['include'], ['src'])
        self.assertEqual(config.read_bytes(), original)

    def test_jsonc_configs_redact_decoded_strings_and_preserve_string_syntax(self):
        config = self.write('src/a/claude-paint/tsconfig.build.json',
                            '{/* config */ "compilerOptions": {"baseUrl": ' +
                            json.dumps(str(self.home)) + ',}, "name": "\\u0073' +
                            'arah", "literal": "https://example.test/*text*/,}",}\n')
        self.write('src/a/claude-paint/settings.jsonc', '{// config\n"values": [1, 2,],}\n')
        original = config.read_bytes()
        self.plan_scope()
        self.assertEqual(self.prepare()['blockers'], [])
        self.cli('restore', self.candidate, self.root / 'restored')
        restored = self.root / 'restored/home'
        data = json.loads((restored / 'src/a/claude-paint/tsconfig.build.json').read_text())
        self.assertEqual(data['compilerOptions']['baseUrl'], str(restored))
        self.assertEqual(data['name'], 'alice')
        self.assertEqual(data['literal'], 'https://example.test/*text*/,}')
        self.assertEqual(json.loads((restored / 'src/a/claude-paint/settings.jsonc').read_text()),
                         {'values': [1, 2]})
        self.assertEqual(config.read_bytes(), original)

    def test_jsonc_support_does_not_relax_malformed_logs_or_encoded_privacy(self):
        rejected = {
            'ordinary.json': '{// not a config\n"a": 1}',
            'session.jsonl': '{"a": 1,}\n',
            'tsconfig.json': '{/* unclosed comment',
            'invalid.jsonc': '{"a":,}',
            'empty-object.jsonc': '{,}',
            'empty-array.jsonc': '[,]',
            'double-comma.jsonc': '[1,,]',
            'tsconfig.private.json': '{// valid config, private payload\n"encrypted_content": "' +
                                     base64.b64encode(self.identity.encode()).decode() + '"}',
        }
        for name, text in rejected.items():
            self.write('src/a/claude-paint/' + name, text)
        self.plan_scope()
        manifest = self.prepare()
        self.assertEqual({Path(x['path']).name for x in manifest['blockers']}, set(rejected))
        self.assertTrue(any('decoded base64' in x['reason'] for x in manifest['blockers']))

    def test_standalone_runner_and_all_finished_painting_records_restore(self):
        run = self.home / 'src/a/stillwet-runs/dryrun/run'
        studio = self.home / 'tmp/paint-studio-finished'
        sessions = self.home / '.pi/agent/sessions/--paint-studio-finished--'
        config = self.write('src/a/stillwet-runs/dryrun/runner/run-config.json', json.dumps({
            'studio': str(studio), 'source': str(self.project), 'run': str(run), 'critic': True}))
        self.write('src/a/stillwet-runs/dryrun/run/studios.json', json.dumps({'OPUS1': str(studio)}))
        self.write('src/a/stillwet-runs/dryrun/run/OPUS/p1_outcome.json', '{"status":"finished"}')
        self.write('src/a/stillwet-runs/dryrun/run/OPUS/p1_sittings.json', json.dumps([
            {'sitting': 1, 'status': 'completed', 'sessions': [str(sessions / 'first.jsonl')]},
            {'sitting': 2, 'part': 2, 'status': 'completed', 'sessions': [str(sessions / 'continuation.jsonl')]},
        ]))
        expected = {
            config.relative_to(self.home): None,
            Path('src/a/stillwet-runs/dryrun/run/studios.json'): None,
            Path('src/a/stillwet-runs/dryrun/run/OPUS/p1_outcome.json'): None,
            Path('src/a/stillwet-runs/dryrun/run/OPUS/p1_sittings.json'): None,
            Path('tmp/paint-studio-finished/notes/critic.md'): 'Latest critique',
            Path('tmp/paint-studio-finished/notes/journal.md'): 'Painting journal',
            Path('src/a/stillwet-runs/dryrun/run/OPUS/critic/first-sitting.md'): 'First critique',
            Path('src/a/stillwet-runs/dryrun/run/OPUS/critic/second-sitting.md'): 'Second critique',
            Path('src/a/stillwet-runs/dryrun/run/OPUS/critic/sessions/first/visit.jsonl'): '{"type":"message"}\n',
            (sessions / 'first.jsonl').relative_to(self.home): '{"type":"message","content":"first sitting"}\n',
            (sessions / 'continuation.jsonl').relative_to(self.home): '{"type":"compaction","summary":"continued"}\n',
        }
        for path, text in expected.items():
            if text is not None:
                self.write(path, text)
        private = self.home / 'private/notes.txt'
        private.parent.mkdir()
        private.write_text('UNRELATED PRIVATE DATA')
        self.write('src/a/stillwet-runs/dryrun/run/private.json', json.dumps({'path': str(private)}))
        self.write('src/a/stillwet-runs/dryrun/run/.env', 'SECRET ENV')
        data = self.plan_scope()
        self.assertTrue(any(run.is_relative_to(Path(x['path'])) for x in data['roots']))
        manifest = self.prepare()
        self.assertEqual(manifest['blockers'], [])
        self.cli('restore', self.candidate, self.root / 'restored')
        restored = self.root / 'restored/home'
        for path, text in expected.items():
            self.assertTrue((restored / path).is_file(), str(path))
            if text is not None and path.suffix != '.jsonl':
                self.assertEqual((restored / path).read_text(), text)
        restored_config = json.loads((restored / config.relative_to(self.home)).read_text())
        self.assertEqual(restored_config['run'], str(restored / run.relative_to(self.home)))
        self.assertEqual(restored_config['studio'], str(restored / studio.relative_to(self.home)))
        sittings = json.loads((restored / 'src/a/stillwet-runs/dryrun/run/OPUS/p1_sittings.json').read_text())
        self.assertEqual(len(sittings), 2)
        for sitting in sittings:
            for session in sitting['sessions']:
                self.assertTrue(Path(session).is_file())
                self.assertEqual(len(Path(session).read_text().splitlines()), 1)
        self.assertFalse((restored / 'private').exists())
        self.assertFalse((restored / 'src/a/stillwet-runs/dryrun/run/.env').exists())

    def test_run_config_references_follow_newly_selected_indexes_only_within_scope(self):
        run = self.home / 'tmp/stillwet-isolated-run'
        studio = self.home / 'tmp/paint-studio-indexed'
        self.write('src/a/claude-paint/runner/run-config.json', json.dumps({'run': str(run)}))
        self.write('tmp/stillwet-isolated-run/studios.json', json.dumps({'OPUS1': str(studio)}))
        self.write('tmp/paint-studio-indexed/notes/journal.md', 'Selected through two indexes')
        unrelated = self.home / 'private/run-config.json'
        unrelated.parent.mkdir()
        unrelated.write_text(json.dumps({'run': str(self.home / 'tmp/stillwet-private')}))
        (self.project / 'run-config.json').symlink_to(unrelated)
        self.write('tmp/stillwet-isolated-run/OPUS/p1_sittings.json', json.dumps([
            {'sessions': [str(self.home / '.pi/agent/sessions/--unrelated-private--/private.jsonl')]}]))
        data = self.plan_scope()
        roots = {x['path'] for x in data['roots']}
        self.assertIn(str(run), roots)
        self.assertIn(str(studio), roots)
        self.assertNotIn(str(self.home / 'tmp/stillwet-private'), roots)
        self.assertNotIn(str(self.home / '.pi/agent/sessions/--unrelated-private--'), roots)

    def test_discovery_rejects_symlinked_ancestors_even_with_allowlisted_targets(self):
        for target in ('private', 'tmp/stillwet-target'):
            with self.subTest(target=target):
                label = Path(target).name
                selected = self.home / ('tmp/stillwet-unread-' + label)
                index = self.write(target + '/records/studios.json', json.dumps({'studio': str(selected)}))
                self.write(str(selected.relative_to(self.home) / 'notes.txt'), 'Not selected')
                link = self.home / ('tmp/stillwet-link-' + label)
                link.symlink_to(index.parent.parent, target_is_directory=True)
                references = [link / 'records', link / 'records/studios.json']
                self.write('src/a/claude-paint/run-config.json', json.dumps({'paths': list(map(str, references))}))
                self.plan = self.root / ('scope-' + label + '.json')
                data = self.plan_scope()
                roots = {x['path'] for x in data['roots']}
                for path in references + [selected]:
                    self.assertNotIn(str(path), roots)

    def test_prepare_rejects_symlinked_ancestors_in_reviewed_roots(self):
        index = self.write('private/records/studios.json', '{"private": "Do not archive"}')
        link = self.home / 'tmp/stillwet-link'
        link.parent.mkdir(parents=True)
        link.symlink_to(index.parent.parent, target_is_directory=True)
        for relative in ('records', 'records/studios.json'):
            with self.subTest(relative=relative):
                self.plan = self.root / ('scope-' + Path(relative).name)
                self.candidate = self.root / ('candidate-' + Path(relative).name)
                data = self.plan_scope()
                data['roots'].append({'path': str(link / relative), 'reasons': ['Reviewed reference'], 'engine': False})
                self.plan.write_text(json.dumps(data))
                result = self.cli('prepare', self.plan, self.candidate,
                                  '--scope-sha256', sha(self.plan), success=False)
                self.assertIn('Scope contains a broad or unrelated private root', result.stderr)
                self.assertFalse(self.candidate.exists())

    def test_current_full_sessions_preservation_and_portable_restore(self):
        """Uncommitted files, every sitting/entry and vanished data survive in a usable layout."""
        current = self.write('src/a/claude-paint/uncommitted.txt', str(self.home) + '/src/a/paint-studio-new\n')
        self.write('src/a/paint-studio-new/paintings/checkpoint.bin', b'checkpoint\x00data')
        session_dir = '.pi/agent/sessions/--' + str(self.home).strip('/').replace('/', '-') + '-src-a-paint-studio-new--'
        entries = [{'type': 'session', 'cwd': str(self.home / 'src/a/paint-studio-new')},
                   {'type': 'compaction', 'summary': 'keep the full summary'},
                   {'type': 'message', 'content': 'complete message ' + self.identity}]
        encoded_image = 'A' * 28 + self.identity + 'A' * 31
        entries[-1]['image'] = {'type': 'image', 'mimeType': 'image/png', 'data': encoded_image}
        session = self.write(session_dir + '/first.jsonl', ''.join(json.dumps(x) + '\n' for x in entries))
        self.write(session_dir + '/continuation.jsonl', '{"type":"message","content":"second sitting"}\n')
        self.write('.pi/agent/sessions/--unrelated-private--/private.jsonl', 'SECRET GLOBAL SESSION')
        self.write('.pi/agent/auth.json', 'SECRET AUTH')
        self.write('src/a/claude-paint/.git/config', 'SECRET GIT CONFIG')
        self.write('src/a/claude-paint/.env', 'SECRET ENV')
        self.write('src/a/stillwet/dist/index.html', '<img src="images/final.png">')
        self.write('src/a/stillwet/dist/images/final.png', b'fake PNG\x00image')
        self.write('src/a/claude-paint/alias', b'temporary')
        (self.project / 'alias').unlink()
        (self.project / 'alias').symlink_to('uncommitted.txt')
        preserve = self.project / '.preservation/old'
        preserve.mkdir(parents=True)
        (preserve / 'scope.json').write_text(json.dumps({'roots': [str(self.home / 'src/a/paint-studio-gone')]}))
        old = {'src/a/paint-studio-gone/painting.lua': b'paint(old)',
               'src/a/claude-paint/uncommitted.txt': b'previous version'}
        with tarfile.open(preserve / 'records.tar.gz', 'w:gz') as archive:
            for name, body in old.items():
                info = tarfile.TarInfo(name)
                info.size = len(body)
                archive.addfile(info, io.BytesIO(body))
        (preserve / 'files.json').write_text(json.dumps([
            {'path': name, 'sha256': hashlib.sha256(body).hexdigest()} for name, body in old.items()]))
        original = (current.read_bytes(), session.read_bytes())
        self.plan_scope()
        manifest = self.prepare()
        self.assertEqual(manifest['blockers'], [])
        self.cli('restore', self.candidate, self.root / 'restored')
        restored = self.root / 'restored/home'
        self.assertEqual((restored / 'src/a/claude-paint/uncommitted.txt').read_text(),
                         str(restored) + '/src/a/paint-studio-new\n')
        self.assertEqual((restored / 'src/a/claude-paint/alias').read_text(),
                         str(restored) + '/src/a/paint-studio-new\n')
        self.assertEqual((restored / 'src/a/paint-studio-gone/painting.lua').read_bytes(), b'paint(old)')
        self.assertEqual((self.root / 'restored/history/old-records/home/src/a/claude-paint/uncommitted.txt').read_bytes(), b'previous version')
        sessions = list((restored / '.pi/agent/sessions').glob('*/*.jsonl'))
        self.assertEqual(len(sessions), 2)
        first = next(x for x in sessions if x.name == 'first.jsonl')
        records = [json.loads(line) for line in first.read_text().splitlines()]
        self.assertEqual([r['type'] for r in records], ['session', 'compaction', 'message'])
        self.assertEqual(records[0]['cwd'], str(restored / 'src/a/paint-studio-new'))
        self.assertEqual(records[2]['content'], 'complete message alice')
        self.assertEqual(base64.b64decode(records[2]['image']['data']), base64.b64decode(encoded_image))
        self.assertEqual(original, (current.read_bytes(), session.read_bytes()))
        for p in (restored / '.pi/agent/auth.json', restored / 'src/a/claude-paint/.git',
                  restored / 'src/a/claude-paint/.env'):
            self.assertFalse(p.exists())
        self.assertTrue((restored / 'src/a/stillwet/dist/images/final.png').is_file())

    def test_binary_identity_blocks_publication_without_corrupting_source(self):
        binary = self.write('src/a/paint-studio-new/bin/easel', b'\x00EXEC' + self.identity.encode())
        image_log = self.write('src/a/paint-studio-new/image.jsonl', json.dumps({
            'type': 'image', 'mimeType': 'image/png', 'data': base64.b64encode(self.identity.encode()).decode()}) + '\n')
        original = binary.read_bytes()
        self.plan_scope()
        manifest = self.prepare()
        self.assertTrue(manifest['blockers'])
        self.assertTrue(any('decoded base64' in x['reason'] for x in manifest['blockers']))
        result = self.cli('upload', self.candidate, '--identifier', 'fixture-never-upload',
                         '--reviewed-sha256', sha(self.candidate / 'manifest.json'),
                         '--confirm-reviewed-content', '--confirm-public-upload', success=False)
        self.assertIn('unresolved', result.stderr)
        self.assertEqual(binary.read_bytes(), original)
        replacement = self.root / 'rebuilt-easel'
        replacement.write_bytes(b'\x00EXEC-public-rebuild')
        plan = json.loads(self.plan.read_text())
        plan['replacements'][str(binary)] = str(replacement)
        image_replacement = self.root / 'reviewed-image.jsonl'
        image_replacement.write_text('{"type":"image","mimeType":"image/png","data":"AAAA"}\n')
        plan['replacements'][str(image_log)] = str(image_replacement)
        self.plan.write_text(json.dumps(plan))
        self.candidate = self.root / 'replacement-candidate'
        self.assertEqual(self.prepare()['blockers'], [])
        sources = json.loads((self.candidate / 'sources.json').read_text())
        receipt = next(x for x in sources['files'] if x['path'].endswith('/bin/easel'))
        self.assertEqual(receipt['original_sha256'], hashlib.sha256(original).hexdigest())
        self.assertEqual(receipt['replacement_sha256'], sha(replacement))
        self.assertEqual(binary.read_bytes(), original)

    def test_scope_digest_and_publication_content_integrity(self):
        self.write('src/a/claude-paint/record.json', '{"name":"\\u0073' + 'arah","value":42}')
        self.plan_scope()
        self.cli('prepare', self.plan, self.candidate, '--scope-sha256', '0' * 64, success=False)
        self.assertFalse(self.candidate.exists())
        self.prepare()
        self.cli('upload', self.candidate, '--identifier', 'fixture-never-upload',
                 '--reviewed-sha256', sha(self.candidate / 'manifest.json'), success=False)
        sources = self.candidate / 'sources.json'
        sources.write_text(sources.read_text() + ' ')
        result = self.cli('verify', self.candidate, success=False)
        self.assertIn('Provenance changed', result.stderr)

    def test_unsafe_preservation_member_and_checksum_are_rejected(self):
        for unsafe in (True, False):
            with self.subTest(unsafe=unsafe):
                preserve = self.project / '.preservation' / str(unsafe)
                preserve.mkdir(parents=True)
                (preserve / 'scope.json').write_text('{"roots":[]}')
                name = '../escape' if unsafe else 'src/a/paint-studio-gone/painting.lua'
                with tarfile.open(preserve / 'records.tar.gz', 'w:gz') as archive:
                    info = tarfile.TarInfo(name)
                    info.size = 3
                    archive.addfile(info, io.BytesIO(b'abc'))
                (preserve / 'files.json').write_text(json.dumps({name: '0' * 64}))
                self.plan = self.root / ('scope-' + str(unsafe) + '.json')
                self.candidate = self.root / ('candidate-' + str(unsafe))
                self.plan_scope()
                result = self.cli('prepare', self.plan, self.candidate,
                                  '--scope-sha256', sha(self.plan), success=False)
                self.assertIn('Unsafe preservation member' if unsafe else 'checksum mismatch', result.stderr)
                self.assertFalse((self.candidate / 'manifest.json').exists())
                # Remove only this fixture's preservation folder before the second case.
                import shutil
                shutil.rmtree(preserve)


if __name__ == '__main__':
    unittest.main()
