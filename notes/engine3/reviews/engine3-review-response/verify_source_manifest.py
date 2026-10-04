"""Check the reviewed source files against the original packet archive."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from zipfile import ZipFile, BadZipFile


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('archive', type=Path)
    args = parser.parse_args()
    expected = json.loads(Path(__file__).with_name('source_manifest.json').read_text())
    failures = []
    try:
        with ZipFile(args.archive) as z:
            for rel, wanted in expected.items():
                name = 'engine3-rag-review/' + rel
                try:
                    got = hashlib.sha256(z.read(name)).hexdigest()
                except KeyError:
                    failures.append(f'MISSING {rel}')
                    continue
                if got != wanted:
                    failures.append(f'CHANGED {rel}: expected {wanted}, got {got}')
    except (OSError, ValueError, BadZipFile) as exc:
        parser.exit(2, f'Cannot inspect archive: {exc}\n')
    if failures:
        print('\n'.join(failures))
        return 1
    print(f'All {len(expected)} reviewed source hashes match the archive.')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
