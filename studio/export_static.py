# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///
"""Export the studio viewer as static files, painters only and scrubbed as --public is.

    uv run studio/export_static.py <out-dir> [--skip paint-studio-xxxxxx ...]

Writes <out>/index.html (the viewer, reading files instead of the live API), <out>/stream.css (its livestream
layout, loaded with ?stream=1), <out>/data/sessions.json and,
per painter, data/<painter>/events.json (with the image extensions), data/<painter>/img/<i>.<ext> and
data/<painter>/file/<the painting's source> (from archive/sources/ once the painter's folder is gone). Every text response is scrubbed like the public server's
(home folder -> ~, account name -> user); the export stops if a scrubbed file still names either.

Each look also gets two small JPEG copies for the web, made once (again only when the look itself changes):
data/<painter>/t/<i>.jpg for the look-strip and data/<painter>/v/<i>.jpg for the main view; the original stays for
the zoom. events.json's "web" lists which looks have them. Without Pillow there are none and the viewer uses the originals.

The export owns <out>: it keeps a list of the files it wrote in <out>/.studio-export and, on the next run,
deletes only the listed files it didn't write again. It won't write into a folder that has files and no list,
except an export from before the list (index.html with the static switch, data/sessions.json, nothing else
at the top), which it adopts, owning all of data/ as the old exporter did.
"""
import argparse, base64, io, json, os, re, sys
from concurrent.futures import ThreadPoolExecutor
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import studio as S
try:
    from PIL import Image
except ImportError:  # the web copies are an optimization: without Pillow the viewer shows the originals
    Image = None

EXT = {"image/png": "png", "image/jpeg": "jpg", "image/webp": "webp", "image/gif": "gif"}
PLAUSIBLE = (b'<script async src="/v/app.js"></script><script>window.plausible=window.plausible||function(){(plausible.q=plausible.q||[]).push(arguments)},'
             b'plausible.init=plausible.init||function(i){plausible.o=i||{}};plausible.init({endpoint:"/v/e"})</script>')
THUMB = (168, 120)  # the look-strip shows 81x58: twice that, for sharp screens
VIEW = 1600         # the main view's copy, long side


def web_copies(data, thumb, view):
    """The strip's thumbnail and the view's copy of one look, as JPEGs. False if the image can't be read."""
    try:
        im = Image.open(io.BytesIO(data))
        im.load()
    except Exception:
        return False
    im = im.convert("RGB")
    for path, box, q in ((view, (VIEW, VIEW), 85), (thumb, THUMB, 72)):
        c = im.copy()
        c.thumbnail(box, Image.LANCZOS)
        buf = io.BytesIO()
        c.save(buf, "JPEG", quality=q, optimize=True, progressive=True)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "wb") as fh:
            fh.write(buf.getvalue())
    return True


WRITTEN = set()
MARKER = ".studio-export"  # the files the last export wrote, one path (relative to out) per line


def inside(out, *parts):
    """out/parts..., refusing a relative part that's absolute or has a `..`, or a path that lands outside out."""
    for r in parts:
        if os.path.isabs(r) or ".." in r.replace("\\", "/").split("/"):
            sys.exit(f"export: refusing the path {r!r}")
    dest = os.path.join(out, *parts)
    if not os.path.realpath(dest).startswith(os.path.realpath(out) + os.sep):
        sys.exit(f"export: {dest} is outside {out}")
    return dest


def old_export(out):
    """out as an export from before the list: just index.html (the static page) and data/ with sessions.json."""
    if sorted(os.listdir(out)) != ["data", "index.html"] or not os.path.isfile(os.path.join(out, "data", "sessions.json")):
        return False
    with open(os.path.join(out, "index.html"), "rb") as fh:
        return b"STUDIO_STATIC = true" in fh.read()


def owned(out):
    """The files the last export into out wrote (relative paths); exits if out has files but no list."""
    marker = os.path.join(out, MARKER)
    if os.path.isfile(marker):
        with open(marker) as fh:
            return [l for l in fh.read().splitlines() if l]
    if os.path.isdir(out) and os.listdir(out) and old_export(out):
        # adopt it: everything under data/ was the old exporter's, as it pruned all it didn't write
        return ["index.html"] + [os.path.relpath(os.path.join(root, f), out)
                                 for root, _, fs in os.walk(os.path.join(out, "data")) for f in fs]
    if os.path.isdir(out) and os.listdir(out):
        sys.exit(f"export: {out} is not empty and has no {MARKER}: not an earlier export, so nothing written "
                 f"(pick an empty or new folder; to adopt an old export, create an empty {MARKER} in it)")
    return []


def write(path, data):
    """Write a file only if its contents changed (so a deploy's rsync sends only what's new)."""
    os.makedirs(os.path.dirname(path), exist_ok=True)
    if isinstance(data, str):
        data = data.encode()
    WRITTEN.add(os.path.abspath(path))
    if os.path.isfile(path) and os.path.getsize(path) == len(data):
        with open(path, "rb") as fh:
            if fh.read() == data:
                return False
    with open(path, "wb") as fh:
        fh.write(data)
    return True


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
    before = owned(out)
    ss = [{k: v for k, v in s.items() if k != "files"} for s in S.list_sessions() if s.get("painter") and s["p"] not in a.skip]
    n_img = n_web = 0
    pool = ThreadPoolExecutor(os.cpu_count() or 4) if Image else None
    for s in ss:
        p = s["p"]
        files = S.painter_files(p)
        st = S.stream("p:" + p, files)
        ev = st["events"]
        n = sum(len(im) for _, _, im in [(f, 0, S._cache[f]["images"]) for f in files])
        exts, web, jobs = [], [0] * n, {}
        for i in range(n):
            im = S.image(st, i)
            ext = EXT.get(im[0], "bin") if im else "bin"
            exts.append(ext)
            if im:
                data = base64.b64decode(im[1])
                changed = write(os.path.join(out, "data", p, "img", f"{i}.{ext}"), data)
                if Image:
                    t, v = (os.path.join(out, "data", p, d, f"{i}.jpg") for d in ("t", "v"))
                    if changed or not (os.path.isfile(t) and os.path.isfile(v)):
                        jobs[i] = pool.submit(web_copies, data, t, v)
                    else:
                        web[i] = 1
                    WRITTEN.update((os.path.abspath(t), os.path.abspath(v)))
        for i, job in jobs.items():
            web[i] = int(job.result())
            n_web += 1
        n_img += n
        # the newest whole look with a web copy (no crop, no mode), for a glimpse of the canvas elsewhere (the gallery's
        # index, the studio's picker)
        s["look"] = None
        for e in ev:
            if e["kind"] == "image" and S.is_whole(e.get("look")) and 0 <= e["img"] < n and web[e["img"]]:
                s["look"] = e["img"]
        # the painting's title, if the painter's last words begin with one
        s["title"] = S.title_of(next((e["text"] for e in reversed(ev) if e["kind"] == "say"), ""))
        # its last event is its own words: it ended a sitting (or the painting), not cut off mid-step; the website lists a
        # quiet run as in progress only then
        s["said"] = bool(ev) and ev[-1]["kind"] == "say"
        if s["look"] is None:  # painters from before the look tool read their renders as files: the last picture they
            # saw (a reference picture, read from the studio's reference/, is never the painter's picture)
            refs = {e["img"] for e in ev if e["kind"] == "image" and e.get("ref")}
            s["look"] = next((i for i in range(n - 1, -1, -1) if web[i] and i not in refs), None)
        text(os.path.join(out, "data", p, "events.json"),
             json.dumps({"events": ev, "total": len(ev), "epoch": st["epoch"], "sittings": len(files), "imgext": exts, "web": web}))
        # the painting's source, as the live server's /api/file gives it
        cwd, rels = S.painting_sources(ev)
        for rel in sorted(rels):
            src = os.path.realpath(os.path.join(cwd, rel))
            if cwd and src.startswith(os.path.realpath(cwd) + os.sep) and os.path.isfile(src):
                with open(src, "rb") as fh:
                    text(inside(out, "data", p, "file", rel), fh.read())
        print(f"{p}: {len(ev)} events, {n} images", flush=True)
    text(os.path.join(out, "data", "sessions.json"), json.dumps(ss))
    with open(os.path.join(S.HERE, "index.html"), "rb") as fh:
        page = fh.read().replace(b'<label id="allwrap"', b'<label id="allwrap" hidden')
    page = page.replace(b"<script>\nconst $ =", b"<script>window.STUDIO_STATIC = true;</script>\n<script>\nconst $ =", 1)
    assert b"STUDIO_STATIC = true" in page, "index.html changed: the static switch didn't go in"
    # the gallery's page-view counter (Plausible, served from stillwet.art/v/), on the public copy only
    page = page.replace(b"</head>", PLAUSIBLE + b"</head>", 1)
    text(os.path.join(out, "index.html"), page)
    with open(os.path.join(S.HERE, "stream.css"), "rb") as fh:  # the livestream's layout, for ?stream=1
        text(os.path.join(out, "stream.css"), fh.read())
    now = sorted(os.path.relpath(f, out) for f in WRITTEN)
    stale = 0
    for rel in set(before) - set(now):
        if os.path.isabs(rel) or ".." in rel.split("/") or rel == MARKER:
            continue
        f = os.path.join(out, rel)
        if os.path.isfile(f):
            os.remove(f); stale += 1
        d = os.path.dirname(f)
        while d != out and os.path.isdir(d) and not os.listdir(d):
            os.rmdir(d); d = os.path.dirname(d)
    with open(os.path.join(out, MARKER), "w") as fh:
        fh.write("".join(r + "\n" for r in now))
    print(f"exported {len(ss)} painters, {n_img} images ({n_web} new web copies) to {out} ({stale} stale files removed)")


if __name__ == "__main__":
    main()
