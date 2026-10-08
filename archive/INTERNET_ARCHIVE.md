# Reconstruction archive preparation

[`archive_upload.py`](../scripts/archive_upload.py) prepares a public, redacted
reconstruction copy from selected **current files**, historical source snapshots
and individual preservation members. It includes uncommitted and ignored project
data. Original sources stay untouched. Preparation and publication are separate.

A real-data packaging test was stopped at the owner’s request during verification
to reclaim disk space. Its generated archive was deleted. No restoration,
rendering, upload or credential configuration was performed.
This is a downloadable collection, not a Wayback capture.

## Selection and contents

[`archive_sources.py`](../scripts/archive_sources.py) creates a private JSON scope
plan. It discovers the project, sibling `stillwet` and `stillwet-data`, current
`paint-studio-*` directories, older project checkouts, every round under
`~/tmp/gallery-fcf9c110` and preservation scopes/inventories. It also reads
`manifest.json`, `studios.json` and `*_sittings.json` references. Project-specific
Pi session directories are selected in full, including continuations and
nongallery work; viewer excerpts do not replace the JSONL records.

The selection includes studios' sources, briefs, notes, research, references,
images, outputs, saved state, checkpoints and `bin/`; runner outcomes and
`.painted` files; website inputs and local `dist` assets; viewer code and vendor
assets; the relevant Caddy configuration source; painter extensions and the
installed Pi package under its specific local npm directory, when present.
Installed Pi package dependencies are retained. Other `node_modules`, build
caches and virtual environments are excluded. Explicit `target/release/easel`
files survive the general `target` exclusion.

Only project-related session directories and named painter extensions/packages
are allowed from the global Pi tree. Auth files, credential filenames, `.env`
files, raw `.git` internals and opaque `.preservation` archives are excluded.
Filename rules are not a general secret detector. Scope review is necessary:
new project locations outside the allowlist require an explicit code change,
not a broader scan of personal directories.

Preservation members are checked against their recorded SHA-256 values before
transformation. Missing current files are recovered into `home/`; divergent
older files go under `history/<preservation-id>/home/`. Current files take
precedence. Preservation Git databases are never published. Historical source
refs and checkout heads are exported under `sources/<commit>/`, with receipts
retaining requested refs and resolved commits. These exports use `git archive`
and therefore honor Git export attributes; they do not include Git history.

The earlier preservation inventory's counts are historical claims, not a new
completeness check. The generated provenance lists actual selected files,
missing inputs, exclusions, redactions, replacements and unresolved components.
A source changing during its individual copy aborts preparation. This does not
provide a cross-file atomic snapshot of active painters; a quiescent source is
needed for that consistency.

## Local workflow

Python 3.11+ and `uv` with a virtual environment are required. Preparation uses
the standard library and local Git. No subcommand except `upload` contacts
Internet Archive. The script does not execute engines, build the website or
configure credentials.

| Subcommand | Required arguments | Effect |
|---|---|---|
| `plan` | New private plan filename, `--project`; optional `--home` | Discovers sources, writes mode-600 JSON and prints its SHA-256 |
| `prepare` | Plan filename, new candidate directory, `--scope-sha256` | Requires the reviewed plan digest; writes the candidate outside selected source roots |
| `verify` | Candidate directory | Rechecks archive inventory, checksums and raw identity scan |
| `restore` | Candidate directory, new restoration directory | Verifies then restores into an isolated tree |
| `upload` | Candidate directory, `--identifier`, `--reviewed-sha256`, `--confirm-reviewed-content`, `--confirm-public-upload` | Public publication after all gates pass |

The private plan remains local. Its `roots`, `source_refs` and `replacements`
are reviewable. Any edit changes the required scope digest. `replacements` maps
an exact original source path, or a preservation/Git `source` label reported in
`sources.json`, to a separate reviewed replacement file. Source and replacement
hashes are both recorded. A fresh preparation is required after changes.

The candidate contains `reconstruction.tar.gz`, `sources.json` and
`manifest.json`. The manifest binds the archive, per-member inventory and public
provenance. The provenance includes redacted source paths, original hashes,
public hashes, scope decisions, engine refs and runtime dependency receipts.
Safe internal symlinks are retained; unsafe or unresolved links block upload.
Failed preparation deletes the partial payload and manifest but retains the
candidate directory and any diagnostic provenance already written.

## Privacy and publication

Text and names receive the approved identity/path redaction. JSON values are
decoded before transformation; every valid JSONL entry, including compaction
records, is retained. Whitespace and JSON serialization can change. Identity
redaction applies even inside quoted source text and can affect code identifiers;
this derivative is not byte-identical to private originals.

Recognized base64 payloads are checked after decoding and retained without
changing their decoded bytes. Accidental identity-like characters in the base64
spelling are escaped at the JSON serialization level. Opaque signatures remain
unchanged; unknown encodings still need manual review.

Binary bytes are never redacted by substitution. A protected identity detected
in a binary excludes that member and creates an upload blocker. A rebuild from
the matching source with sanitized build paths, or a separately reviewed
replacement, is needed before a new candidate can pass. Original and replacement
provenance remain distinct. Invalid JSON/JSONL also creates a blocker.

The raw scan is not an automated privacy certification. Content review includes
all outgoing files and metadata, decoded/encoded data, nested archives, embedded
session images, image text, video, secrets, third-party notices, omissions and
runtime dependencies. The two upload confirmation flags attest to that review
and to explicit publication authorization; the exact manifest digest binds the
review to this candidate. A digest alone does not unlock upload.

Upload requires `internetarchive==5.11.1` and separately configured credentials.
It rechecks the candidate, rejects unresolved blockers and refuses any existing
item identifier, including a partially uploaded item. Only the three candidate
files are sent, with `verify=True` and `checksum=True`. Successful requests do
not establish finished processing or functioning public downloads. Partial
uploads need remote inspection before any recovery decision.
[Client API](https://archive.org/developers/internetarchive/api.html) ·
[Authentication](https://archive.org/developers/internetarchive/cli.html) ·
[Pinned client](https://github.com/jjjake/internetarchive/blob/v5.11.1/internetarchive/item.py)

## Reconstruction and limits

The archive contains its preparation/restoration tools under `tools/`.
Restoration preserves the home-relative directory layout under
`<destination>/home`, including `src/a/`, `tmp/` and selected `.pi/` data. Absolute
home references use `/ARCHIVE_HOME` in the public copy. Restoration rewrites that
marker and encoded session-directory names to the new home in text/JSON,
creates safe relative symlinks last and never overwrites an existing destination.
Project tools need that restored directory as `HOME`. Original sources and the
public candidate remain unchanged; restored text has new hashes because its
paths changed. Other absolute paths and platform-specific binary paths remain
recorded dependencies, not silently claimed portable.

The restored `stillwet/dist` is the available **local** static snapshot. It has
not been verified equal to `/var/lib/stillwet` on the deployed host. Serving its
root also exposes its copied `/studio/`, images, data and media paths when those
assets were present. Missing local assets remain gaps; no remote acquisition
has occurred. Rebuilding uses the adjacent `stillwet-data` and the restored
`claude-paint` exporter, following the captured `stillwet/DEPLOY.md`.
`build.py` declares Python >=3.11, Pillow and markdown-it-py through PEP 723.
Caddy's query routing, redirects and analytics are deployment configuration,
not behavior supplied by a basic static file server.

Replay uses each studio's pinned executable when compatible, or a build from its
recorded source snapshot and Cargo lockfile. `otool -L`/`readelf -d` inspect
available executable dependencies without running them. Available non-system
absolute libraries are copied under `runtime/`; OS libraries are recorded but
not redistributed. Loader search paths can remain unresolved. Copied libraries
may need loader relocation; rebuilding from source is the cross-platform route.
The installed Pi package records its current version and dependencies, not a
claim that this was every historical sitting's harness version. Captured
extension sources and runner instructions supply further provenance; missing
old harness versions remain reviewable gaps. Credentials are never restored.

For r22.1, original source `0e00118` remains unavailable. The approved substitute
is `f334aef11d0c069b083371d2cb8ceb4865664e83`; missing original source is nonfatal.
The existing [media-backlog receipt](../notes/media-backlog/INDEX.md) reports a
matching final PNG, not identical intermediate states. Both that provenance and
other receipt/checkout mismatches remain in the copy. No new replay measurement
is claimed here.

Validation covers four synthetic CLI tests: complete selected sessions and
portable restoration, current/preserved file precedence, binary publication
blocking, scope/content integrity and unsafe preservation input rejection.
All four passed under Python 3.13.14; static syntax parsing also passed.
The interrupted real-data test wrote approximately 70 GiB of compressed data
with 258,340 members and 171 publication blockers, mostly protected identity
strings in binary files. Its manifest listed 84,600,270,815 restored bytes.
Verification did not finish; this is not a successful reconstruction result.
Complete restoration, historical builds, website behavior, platform dependencies
and publication remain unverified. The implementation has no progress reporting
or disk-space guard; a future run needs sufficient headroom and monitoring.
