# /// script
# requires-python = ">=3.11"
# ///
"""Export the studio viewer as static files, painters only and scrubbed as --public is.

    uv run studio/export_static.py <out-dir> [--skip paint-studio-xxxxxx ...]

Writes <out>/index.html (the viewer, reading files instead of the live API), <out>/data/sessions.json and,
per painter, data/<painter>/events.json (with the image extensions), data/<painter>/img/<i>.<ext> and
data/<painter>/file/<the painting's source>. Every text response is scrubbed like the public server's
(home folder -> ~, account name -> user); the export stops if a scrubbed file still names either.
"""
import argparse, base64, json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import studio as S

EXT = {"image/png": "png", "image/jpeg": "jpg", "image/webp": "webp", "image/gif": "gif"}


WRITTEN = set()


def write(path, data):
    """Write a file only if its contents changed (so a deploy's rsync sends only what's new)."""
    os.makedirs(os.path.dirname(path), exist_ok=True)
    if isinstance(data, str):
        data = data.encode()
    WRITTEN.add(os.path.abspath(path))
    if os.path.isfile(path) and os.path.getsize(path) == len(data):
        with open(path, "rb") as fh:
            if fh.read() == data:
                return
    with open(path, "wb") as fh:
        fh.write(data)


def text(path, data):
    body = S.scrub(data if isinstance(data, bytes) else data.encode())
    if S.HOME.encode() in body or re.search(re.escape(S.USER.encode()), body, re.I):
        sys.exit(f"export: {path} still names the home folder or account after scrubbing")
    write(path, body)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("out")
    ap.add_argument("--skip", nargs="*", default=[], help="painter folders to leave out (test runs)")
    a = ap.parse_args()
    S.PUBLIC = True
    out = os.path.abspath(a.out)
    ss = [{k: v for k, v in s.items() if k != "files"} for s in S.list_sessions() if s.get("painter") and s["p"] not in a.skip]
    text(os.path.join(out, "data", "sessions.json"), json.dumps(ss))
    n_img = 0
    for s in ss:
        p = s["p"]
        files = S.painter_files(p)
        st = S.stream("p:" + p, files)
        ev = st["events"]
        n = sum(len(im) for _, _, im in [(f, 0, S._cache[f]["images"]) for f in files])
        exts = []
        for i in range(n):
            im = S.image(st, i)
            ext = EXT.get(im[0], "bin") if im else "bin"
            exts.append(ext)
            if im:
                write(os.path.join(out, "data", p, "img", f"{i}.{ext}"), base64.b64decode(im[1]))
        n_img += n
        text(os.path.join(out, "data", p, "events.json"),
             json.dumps({"events": ev, "total": len(ev), "epoch": st["epoch"], "sittings": len(files), "imgext": exts}))
        # the painting's source, as the live server's /api/file gives it
        cwd, rels = S.painting_sources(ev)
        for rel in sorted(rels):
            src = os.path.realpath(os.path.join(cwd, rel))
            if cwd and src.startswith(os.path.realpath(cwd) + os.sep) and os.path.isfile(src):
                with open(src, "rb") as fh:
                    text(os.path.join(out, "data", p, "file", rel), fh.read())
        print(f"{p}: {len(ev)} events, {n} images", flush=True)
    with open(os.path.join(S.HERE, "index.html"), "rb") as fh:
        page = fh.read().replace(b'<label id="allwrap"', b'<label id="allwrap" hidden')
    page = page.replace(b"<script>\nconst $ =", b"<script>window.STUDIO_STATIC = true;</script>\n<script>\nconst $ =", 1)
    assert b"STUDIO_STATIC = true" in page, "index.html changed: the static switch didn't go in"
    text(os.path.join(out, "index.html"), page)
    stale = 0
    for root, _, fs in os.walk(out, topdown=False):
        for f in fs:
            if os.path.abspath(os.path.join(root, f)) not in WRITTEN:
                os.remove(os.path.join(root, f)); stale += 1
        if root != out and not os.listdir(root):
            os.rmdir(root)
    print(f"exported {len(ss)} painters, {n_img} images to {out} ({stale} stale files removed)")


if __name__ == "__main__":
    main()
