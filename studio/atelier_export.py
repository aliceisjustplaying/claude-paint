# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///
"""The atelier's data for stillwet.art/atelier: atelier.json and its web images, from ~/src/a/stillwet-runs/atelier.

    uv run studio/atelier_export.py <out>      (out: the site's atelier/data/ folder)

For each studio in the atelier's studios.json it writes
  <slug>/<n>/{gallery,color,relief}.jpg, <slug>/<n>/journal.md   each painting on its walls
  <slug>/<n>/look.jpg                                            the newest whole look of the painting on the easel
  <slug>/notebook/<n>.md                                         the notebook as painting n left it
and atelier.json in the shape of the atelier spec's section 7, its paths relative to /atelier/ (data/<slug>/...).
A painting is on the walls once its card (studios/<id>/cards/<nnn>.json) exists. The newest work without one is on
the easel: live by the studio export's rule (a running sitting, a painter at work in its folder and activity within
30 minutes), else paused. Text is scrubbed as the studio export's is, and the export stops if a file still names the
home folder or the account. It owns <out> as export_static.py owns its folder.
"""
import argparse, difflib, glob, io, json, os, re, sys, time
from datetime import datetime, timezone
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import export_static as E
import studio as S
from PIL import Image

WEB = 1600  # a wall image's long side
LIGHTS = ("gallery", "color", "relief")


def load(path):
    try:
        with open(path) as fh:
            return json.load(fh)
    except (OSError, ValueError):
        return None


def read(path):
    try:
        with open(path, errors="replace") as fh:
            return fh.read()
    except OSError:
        return None


def jpeg(src, dest):
    """A picture (a file or its bytes) as a JPEG at dest, at most WEB px on its long side; a file's copy is remade
    only when the file is newer than it."""
    E.WRITTEN.add(os.path.abspath(dest))
    if isinstance(src, str) and os.path.isfile(dest) and os.path.getmtime(dest) >= os.path.getmtime(src):
        return
    with Image.open(src if isinstance(src, str) else io.BytesIO(src)) as im:
        im = im.convert("RGB")
        im.thumbnail((WEB, WEB), Image.LANCZOS)
        buf = io.BytesIO()
        im.save(buf, "JPEG", quality=85, optimize=True, progressive=True)
    E.write(dest, buf.getvalue())


def added(before, after):
    """What a painting added to the notebook: the lines of after that aren't in before, each run of them a paragraph."""
    a, b = before.splitlines(), after.splitlines()
    return "\n\n".join("\n".join(b[j1:j2]).strip() for tag, _, _, j1, j2 in
                       difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes()
                       if tag in ("insert", "replace") and "".join(b[j1:j2]).strip())


def easel(slug, n, work, sessions, out):
    """The painting on the easel: its state, times and newest whole look, in the gallery light when there is one."""
    manifest = load(os.path.join(work, "manifest.json")) or {}
    folder = next(iter((load(os.path.join(work, "run", "studios.json")) or {}).values()), None)
    folder = folder or os.path.basename((manifest.get("folder") or "").rstrip("/")) or None
    session = next((s for s in sessions if s["p"] == folder), None)
    look = last = None
    if session:
        st = S.stream("p:" + folder, S.painter_files(folder))
        for e in st["events"]:
            if e["kind"] == "paint" and not e.get("scratch"):
                last = e["ts"]
            if e["kind"] == "image" and not e.get("display_pending") and not e.get("ref") and S.is_whole(e.get("look")):
                look = e
        if look:
            im = (look.get("gallery") and S.image_bytes(st, look["img"], light="gallery")) or S.image_bytes(st, look["img"])
            look = None
            if im:
                jpeg(im[1], E.inside(out, slug, str(n), "look.jpg"))
                look = f"data/{slug}/{n}/look.jpg"
    return {"n": n, "folder": folder, "state": "live" if session and session["active"] else "paused",
            "title": S.runner_title(folder) if folder else None, "theme": manifest.get("theme"),
            "started": manifest.get("started"), "last": last, "look": look}


def studio(sid, info, sessions, out):
    home, slug = os.path.join(E.ATELIER, "studios", sid), info.get("slug") or sid
    cards = sorted((c for c in map(load, glob.glob(os.path.join(home, "cards", "*.json")))
                    if isinstance(c, dict) and isinstance(c.get("number"), int)), key=lambda c: c["number"])
    walls = []
    for card in cards:
        n, work = card["number"], os.path.join(E.ATELIER, "works", f"{sid}-{card['number']:03d}")
        manifest = load(os.path.join(work, "manifest.json")) or {}
        wall = {"n": n, "title": card.get("title"), "words": card.get("words"), "folder": card.get("folder"),
                "finished": card.get("finished"), "theme": manifest.get("theme"), "sittings": manifest.get("sittings"),
                "img": {}}
        for light in LIGHTS:
            src = os.path.join(work, "final", light + ".png")
            if os.path.isfile(src):
                jpeg(src, E.inside(out, slug, str(n), light + ".jpg"))
                wall["img"][light] = f"data/{slug}/{n}/{light}.jpg"
        if manifest.get("canvas"):
            wall["canvas"] = manifest["canvas"]
        journal = read(os.path.join(work, "journal.md"))
        if journal is not None:
            E.text(E.inside(out, slug, str(n), "journal.md"), journal)
            wall["journal"] = f"data/{slug}/{n}/journal.md"
        walls.append(wall)
    hung = {c["number"] for c in cards}
    works = [int(m.group(1)) for w in glob.glob(os.path.join(E.ATELIER, "works", glob.escape(sid) + "-*"))
             if (m := re.fullmatch(re.escape(sid) + r"-(\d+)", os.path.basename(w)))]
    on = max((n for n in works if n not in hung), default=None)
    versions = []
    for path in sorted(glob.glob(os.path.join(home, "notebook", "*-after.md"))):
        m = re.fullmatch(r"(\d+)-after\.md", os.path.basename(path))
        if not m:
            continue
        n, after = int(m.group(1)), read(path) or ""
        E.text(E.inside(out, slug, "notebook", f"{n}.md"), after)
        versions.append({"after": n, "added": added(read(os.path.join(home, "notebook", f"{m.group(1)}-before.md")) or "", after),
                         "file": f"data/{slug}/notebook/{n}.md"})
    versions.sort(key=lambda v: v["after"])
    return {"slug": slug, "name": info.get("name") or E.MODEL_NAMES.get(info.get("model"), info.get("model")),
            "since": info.get("since"),
            "easel": easel(slug, on, os.path.join(E.ATELIER, "works", f"{sid}-{on:03d}"), sessions, out) if on is not None else
            {"n": None, "folder": None, "state": "empty", "title": None, "theme": None, "started": None, "last": None, "look": None},
            "walls": walls,
            "notebook": {"text": read(os.path.join(home, "notebook.md")) or "",
                         "after": versions[-1]["after"] if versions else None, "versions": versions}}


def export(out):
    S.PUBLIC = True
    E.WRITTEN.clear()
    out = os.path.abspath(out)
    before = E.owned(out)
    sessions = [s for s in S.list_sessions() if s.get("painter")]
    E.runner_states(sessions, E.working_studios(), time.time())
    studios = [studio(sid, info, sessions, out) for sid, info in (load(os.path.join(E.ATELIER, "studios.json")) or {}).items()
               if isinstance(info, dict)]
    E.text(os.path.join(out, "atelier.json"), json.dumps(
        {"updated": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"), "studios": studios}, ensure_ascii=False))
    stale = E.prune(out, before)
    print(f"exported {len(studios)} studios, {sum(len(s['walls']) for s in studios)} paintings on their walls to {out} "
          f"({stale} stale files removed)")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("out", help="the site's atelier/data/ folder")
    export(ap.parse_args().out)


if __name__ == "__main__":
    main()
