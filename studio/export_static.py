# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///
"""Export the studio viewer as static files, painters only and scrubbed as --public is.

    uv run studio/export_static.py <out-dir> [--skip paint-studio-xxxxxx ...] [--only-atelier | --skip-atelier]

Writes <out>/index.html (the viewer, reading files instead of the live API), <out>/stream.css (its livestream
layout, loaded with ?stream=1), <out>/data/sessions.json and,
per-painter share/<painter>.html (the same viewer with crawler-visible metadata, served by Caddy for ?p= links),
per painter, data/<painter>/events.json (with the image extensions), data/<painter>/img/<i>.<ext> and
data/<painter>/file/<the painting's source> (from stillwet-paintings/other/claude-paint/sources/ once the painter's folder is gone). Every text response is scrubbed like the public server's
(home folder -> ~, account name -> user); the export stops if a scrubbed file still names either.

Each look also gets two small JPEG copies for the web, made once (again only when the look itself changes):
data/<painter>/t/<i>.jpg for the look-strip and data/<painter>/v/<i>.jpg for the main view; the original stays for
the zoom. events.json's "web" lists which looks have them. Without Pillow there are none and the viewer uses the originals.

The export owns <out>: it keeps a list of the files it wrote in <out>/.studio-export and, on the next run,
deletes only the listed files it didn't write again. It won't write into a folder that has files and no list,
except an export from before the list (index.html with the static switch, data/sessions.json, nothing else
at the top), which it adopts, owning all of data/ as the old exporter did.

The atelier's paintings are the folders its works name (~/src/a/stillwet-runs/atelier/works/*/run/studios.json and
manifest.json). --skip-atelier leaves them out (the house export at /studio/). --only-atelier exports only them, as the
viewer at /atelier/easel/: its links point there, and every script is a same-origin file (js/<hash>.js), because the
atelier's pages allow no inline script.
"""
import argparse, glob, hashlib, html, io, json, os, re, shutil, subprocess, sys, time
import platform
from concurrent.futures import ThreadPoolExecutor
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import studio as S
from urllib.parse import quote
try:
    from PIL import Image
    from palette_board import palette_board
except ImportError:  # the web copies are an optimization: without Pillow the viewer shows the originals
    Image = None

EXT = {"image/png": "png", "image/jpeg": "jpg", "image/webp": "webp", "image/gif": "gif"}
PLAUSIBLE = (b'<script async src="/v/app.js"></script><script>window.plausible=window.plausible||function(){(plausible.q=plausible.q||[]).push(arguments)},'
             b'plausible.init=plausible.init||function(i){plausible.o=i||{}};plausible.init({endpoint:"/v/e"})</script>')
THUMB = (168, 120)  # the look-strip shows 81x58: twice that, for sharp screens
VIEW = 1600         # the main view's copy, long side
SITE = "https://stillwet.art"
DEFAULT_IMAGE = {"url": SITE + "/img/og-card.jpg", "width": 1200, "height": 630}
ATELIER = os.path.expanduser("~/src/a/stillwet-runs/atelier")
# Keep the returned header's model names consistent with the interactive viewer.
MODEL_NAMES = {
    "claude-opus-5-5": "Claude Opus 5.5", "claude-fable-5-1": "Claude Fable 5.1",
    "claude-sonnet-5": "Claude Sonnet 5", "claude-sonnet-5-5": "Claude Sonnet 5.5",
    "gpt-6-astra": "GPT-6 Astra", "gpt-6-luna": "GPT-6 Luna", "gpt-6.1-sol": "GPT-6.1 Sol",
    "gemini-3.8-flash": "Gemini 3.8 Flash", "muse-spark-1.3": "Muse Spark 1.3", "muse-wonder-full-1.0": "Muse Wonder Full",
    "glm-5.3-flash": "GLM-5.3 Flash", "mimo-v2.6-pro": "MiMo v2.6 Pro",
    "deepseek-v4.1-flash": "DeepSeek V4.1 Flash", "kimi-k3": "Kimi K3", "space-bunny-free": "Space Bunny",
}


def atelier_folders():
    """The painting folders of the atelier's works, by name: each work's run/studios.json, written at its start, and
    its manifest's folder."""
    folders = set()
    for path in glob.glob(os.path.join(ATELIER, "works", "*", "run", "studios.json")):
        try:
            with open(path) as fh:
                folders.update(v for v in json.load(fh).values() if isinstance(v, str))
        except (OSError, ValueError, AttributeError):
            continue
    for path in glob.glob(os.path.join(ATELIER, "works", "*", "manifest.json")):
        try:
            with open(path) as fh:
                folder = json.load(fh).get("folder")
        except (OSError, ValueError, AttributeError):
            continue
        if isinstance(folder, str):
            folders.add(os.path.basename(folder.rstrip("/")))
    return folders


def working_studios():
    """Actual painter processes, not persistent easel servers or saved running records: a Pi's working directory,
    or the --studio of an agy painter's easel tool server (the studio's bin/mcp/server.py --agy)."""
    try:
        ps = subprocess.run(["ps", "-Ao", "pid=,comm="], capture_output=True, text=True, check=True, timeout=10)
        pids = [parts[0] for row in ps.stdout.splitlines()
                if len(parts := row.split()) == 2 and os.path.basename(parts[1]) == "pi"]
        args = subprocess.run(["ps", "-wwAo", "args="], capture_output=True, text=True, check=True, timeout=10)
        agy = {os.path.basename(m.group(1)) for row in map(str.strip, args.stdout.splitlines())
               if re.search(r"(?:^| )--agy(?: |$)", row)
               and (m := re.search(r"/bin/mcp/server\.py .*?--studio (.+?)(?: --|$)", row))}
        if not pids:
            return agy
        if platform.system() == "Linux":
            working = set(agy)
            for pid in pids:
                try:
                    working.add(os.path.basename(os.readlink(f"/proc/{pid}/cwd")))
                except OSError:
                    pass
            return working
        lsof = shutil.which("lsof") or "/usr/sbin/lsof"
        r = subprocess.run([lsof, "-a", "-p", ",".join(pids), "-d", "cwd", "-Fn"],
                           capture_output=True, text=True, timeout=10)
        return agy | {os.path.basename(row[1:]) for row in r.stdout.splitlines() if row.startswith("n")}
    except (OSError, subprocess.SubprocessError):
        return set()  # no execution evidence: never claim live


def runner_states(sessions, working, now):
    """Require a runner or native MCP sitting, a live painter in that studio and recent session activity.
    Completion comes only from the runner outcome, never a title, quiet session or exported render."""
    records = {}
    for path in S.runner_paths():
        try:
            with open(path) as fh:
                names = json.load(fh)
            for key, painter in names.items():
                lane, number = S.runner_key(path, key)
                rd = os.path.join(os.path.dirname(path), lane)
                stem = os.path.join(rd, "p" + (number or "0"))
                running = False
                try:
                    with open(stem + "_sittings.json") as fh:
                        sittings = json.load(fh)
                    running = bool(sittings) and sittings[-1].get("status") == "running"
                except (OSError, ValueError, AttributeError, TypeError, KeyError):
                    pass
                completed = next((os.path.getmtime(f) for f in (stem + ".painted", stem + "_outcome.json")
                                  if os.path.isfile(f)), 0)
                records[painter] = (running, completed)
        except (OSError, ValueError, AttributeError, TypeError):
            continue
    for s in sessions:
        running, completed = records.get(s["p"], (False, 0))
        if s["p"] not in records and s["p"] in working:
            try:
                with open(os.path.join(os.path.expanduser("~/src/a"), s["p"], "out/easel/sittings.json")) as fh:
                    running = json.load(fh).get("complete") is False
            except (OSError, ValueError, AttributeError, TypeError):
                pass
        s["active"] = bool(not s.get("outcome") and running and s["p"] in working and 0 <= now - s["mtime"] < 1800)
        s["completed_at"] = (completed or s["mtime"]) if (s.get("outcome") or {}).get("status") == "finished" else None


def featured(sessions):
    active = [s for s in sessions if s.get("active")]
    if active:
        return max(active, key=lambda s: s["mtime"])
    completed = [s for s in sessions if s.get("completed_at") is not None]
    return max(completed, key=lambda s: s["completed_at"]) if completed else None


def card_page(page, painter, homepage=False, root="/studio/"):
    """Metadata and the selected painter's first view in the returned HTML, escaped and scrubbed."""
    url = SITE + root + ("?p=" + quote(painter["p"], safe="") if painter and not homepage else "")
    image = (painter or {}).get("og_image") or DEFAULT_IMAGE
    title, description = "The studio · stillwet", "AI models painting in a simulation of oil paint. Watch every brushstroke and how the picture grows."
    if painter:
        state = "Live" if painter["active"] else {
            "finished": "Finished", "cap_reached": "Sitting limit reached", "crash_limit_reached": "Stopped after errors"
        }.get((painter.get("outcome") or {}).get("status"), "Not currently painting")
        name = painter.get("title") or "Untitled"
        model = painter.get("model") or "AI painter"
        subject = painter.get("artist") or painter.get("subject")
        subject = (" after " + subject) if subject and subject not in ("free", "self-portrait") else ""
        title = f"{name} · {model} · {state} · stillwet studio"
        description = f"{state}: {model} painting{subject} in a simulation of oil paint. Latest canvas snapshot; watch every brushstroke in the studio."
    esc = lambda value: html.escape(str(value), quote=True)
    tags = [f"<title>{esc(title)}</title>", f'<base href="{esc(root)}">',
            f'<link rel="canonical" href="{esc(url)}">']
    for key, value in (("description", description), ("og:title", title), ("og:description", description),
                       ("og:type", "website"), ("og:url", url), ("og:image", image["url"]),
                       ("og:image:width", image["width"]), ("og:image:height", image["height"]),
                       ("og:image:type", "image/jpeg"), ("og:image:alt", title),
                       ("twitter:card", "summary_large_image"), ("twitter:title", title),
                       ("twitter:description", description), ("twitter:image", image["url"]), ("twitter:image:alt", title)):
        attr = "property" if key.startswith("og:") else "name"
        tags.append(f'<meta {attr}="{key}" content="{esc(value)}">')
    # The default selection also opens the painting on direct access to a generated page.
    initial = {k: v for k, v in painter.items() if k in (
        "p", "painter", "folder", "mtime", "model", "title", "subject", "artist", "reference_artist",
        "active", "outcome", "said", "look", "og_image",
    )} if painter else None
    # JSON in a script must not be able to close its element, even through a painting title.
    script_json = lambda value: json.dumps(value).replace("<", "\\u003c")
    tags.append('<script>window.STUDIO_FEATURED=' + script_json(painter["p"] if painter else None)
                + ';window.STUDIO_INITIAL=' + script_json(initial) + ';'
                + "(()=>{const u=new URLSearchParams(location.search),p=u.get('p');"
                  "if(window.STUDIO_INITIAL&&((p&&p!==window.STUDIO_FEATURED)||(!p&&u.get('s')))){"
                  "document.documentElement.classList.add('studio-route-pending');document.title='The studio · stillwet'}})();</script>")
    start, end = b"<!-- STUDIO_META_START -->", b"<!-- STUDIO_META_END -->"
    before, rest = page.split(start, 1)
    _, after = rest.split(end, 1)
    page = before + start + ("\n" + "\n".join(tags) + "\n").encode() + end + after
    page = page.replace(b'<body class="nocode">', b'<body class="nocode static">', 1)
    page = page.replace(b'<div id="clockbox">', b'<div id="clockbox" hidden>', 1)
    if painter:
        active = painter.get("active") is True and time.time() - painter["mtime"] < 1800
        name = painter.get("title") or ("Untitled, in progress" if active else "Untitled")
        model = " → ".join(MODEL_NAMES.get(m, m) for m in (painter.get("model") or "").split(" → "))
        subject = painter.get("subject")
        subject = {"Friedrich": "after Friedrich", "free": "free subject", "self-portrait": "self-portrait"}.get(
            subject, "after " + (painter.get("artist") or subject)) if subject else ""
        header = f'{name} · {model}' + (f' · {subject}' if subject else '') + ': choose another painter'
        page = page.replace(b'title="choose another painter"', f'title="{esc(header)}"'.encode(), 1)
        page = page.replace(b'<span class="t">Choose a painter</span><span class="m"></span>',
                            f'<span class="t">{esc(name)}</span><span class="m">{esc(model)}</span>'.encode(), 1)
        badge = "LIVE" if active else {
            "finished": "Finished", "cap_reached": "Sitting limit reached", "crash_limit_reached": "Stopped after errors",
        }.get((painter.get("outcome") or {}).get("status"), "Paused")
        page = page.replace(b'<span id="badge"></span>',
                            f'<span id="badge" class="{"live" if active else ""}">{badge}</span>'.encode(), 1)
        if painter.get("og_image"):
            pic = painter["og_image"]
            page = page.replace(b'<img id="img"',
                                f'<img id="img" src="{esc(pic["url"].removeprefix(SITE).split("?", 1)[0])}" fetchpriority="high"'.encode(), 1)
            if pic["width"] < pic["height"] * 1.15:
                page = page.replace(b'<div id="view">', b'<div id="view" class="tall">', 1)
        else:
            page = page.replace(b'<p id="nopic" hidden>', b'<p id="nopic">', 1)
    return page


def external_scripts(page, out):
    """Each inline script of page as a same-origin file, js/<hash of its code>.js, for a page whose policy allows
    scripts from its own site only."""
    def external(m):
        name = "js/" + hashlib.sha256(m.group(1)).hexdigest()[:16] + ".js"
        text(os.path.join(out, name), m.group(1))
        return b'<script src="' + name.encode() + b'"></script>'
    return re.sub(rb"<script>(.*?)</script>", external, page, flags=re.S)


def palette_chips(data, names):
    """Match look.rs chart glyphs before browser color processing or canvas protections."""
    im = Image.open(io.BytesIO(data)).convert("RGB")
    if im.width == 1000:
        return palette_board(data)
    if im.width != 1124:
        return None
    px, rows = im.load(), []
    for y in range(32, im.height):
        white = [x for x in range(8, 532) if px[x, y] == (255, 255, 255)]
        if not white:
            continue
        if rows and y - rows[-1][1] <= 16:
            rows[-1][1] = y
            rows[-1][2] = max(rows[-1][2], white[-1])
        else:
            rows.append([y, y, white[-1]])
    glyphs = dict(zip("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_", (
        "75557 26227 71747 71317 55711 74717 74757 71122 75757 75717 "
        "25755 65656 34443 65556 74647 74644 34553 55755 72227 11152 55655 44447 "
        "57755 65555 25552 65644 25563 65655 34216 72222 55557 55552 55775 55255 55222 71247 00007"
    ).split()))
    signatures = {}
    for name in names:
        if len(name) > 66:
            continue
        points = [(x, y) for y in range(5) for x in range(len(name) * 4)
                  if x % 4 < 3 and int(glyphs[name[x // 4].upper()][y]) & (4 >> (x % 4))]
        top = min(y for _, y in points)
        key = tuple((x, y - top) for x, y in points)
        signatures[key] = name if key not in signatures else None
    chips = []
    for i, (top, bottom, right) in enumerate(rows):
        key = tuple(((x - 10) // 2, (y - top) // 2)
                    for y in range(top, bottom + 1, 2) for x in range(10, right + 1, 2)
                    if px[x, y] == (255, 255, 255))
        name = signatures.get(key)
        if name is None:
            return None
        end = rows[i + 1][0] - 3 if i + 1 < len(rows) else im.height
        y = (top - 3 + end) // 2
        chips.append({"name": name, "thick": "#%02x%02x%02x" % px[608, y],
                      "thin": "#%02x%02x%02x" % px[752, y]})
    return chips or None


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
EXPORTED = {}  # (out, painter, image) -> (its sources, ext, web copies made, files written): see image_sources


def image_sources(st, i):
    """What image i of a stream is made of: its look and display companions, each with the file's state when it
    is a picture on disk, or the session's parse generation for a look inside it (appends leave a look alone; a
    replaced session is parsed again). A later export in the same process skips an image whose sources match the
    last one's. None if a file can't be read."""
    for f, _, ni in st["parts"]:
        if i < ni:
            c = S._cache[f]
            srcs = [c["images"][i] if i < len(c["images"]) else None]
            srcs += [c["lights"].get(light, {}).get(i) for light in S.DISPLAY_LIGHTS]
            try:
                return tuple(src and (src, c.get("generation", 0) if src[1] is not None else os.stat(src[0]).st_mtime_ns)
                             for src in srcs)
            except OSError:
                return None
        i -= ni
    return None
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
    if S.HOME.encode() in body or (S.USER != "user" and re.search(re.escape(S.USER.encode()), body, re.I)):
        sys.exit(f"export: {path} still names the home folder or account after scrubbing")
    write(path, body)


def export(out, skip=(), only=None, live=False, atelier=None, elsewhere=()):
    """atelier: None for every painter, "only" for the atelier's alone (the viewer at /atelier/easel/), "skip" for
    every painter but the atelier's. elsewhere: painters working on another machine, live while their session is recent."""
    S.PUBLIC = True
    WRITTEN.clear()
    out = os.path.abspath(out)
    before = owned(out)
    folders = atelier_folders() if atelier else set()
    root = "/atelier/easel/" if atelier == "only" else "/studio/"
    ss = [{k: v for k, v in s.items() if k != "files"} for s in S.list_sessions()
          if s.get("painter") and s["p"] not in skip and (only is None or s["p"] == only)
          and (atelier is None or (s["p"] in folders) == (atelier == "only"))]
    if only is not None and len(ss) != 1:
        raise ValueError("the selected painter has no viewer session yet")
    runner_states(ss, working_studios(), time.time())
    for s in ss:
        if live or s["p"] in elsewhere:
            s["active"] = not s.get("outcome") and 0 <= time.time() - s["mtime"] < 1800
    n_img = n_web = 0
    pool = ThreadPoolExecutor(os.cpu_count() or 4) if Image else None
    for s in ss:
        p = s["p"]
        files = S.painter_files(p)
        st = S.stream("p:" + p, files)
        ev = st["events"]
        names = set()
        if Image:
            for e in ev:
                names.update(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", e.get("code", "")))
                if e["kind"] == "image" and re.search(r"palette (?!False)", e.get("look", "")):
                    im = S.image_bytes(st, e["img"])
                    chips = palette_chips(im[1], names) if im else None
                    if chips:
                        e["palette"] = chips
        n = sum(len(im) for _, _, im in [(f, 0, S._cache[f]["images"]) for f in files])
        exts, web, jobs = [], [0] * n, {}
        for i in range(n):
            srcs = image_sources(st, i)
            done = EXPORTED.get((out, p, i))
            if srcs and done and done[0] == srcs and all(map(os.path.isfile, done[3])):
                exts.append(done[1])
                web[i] = done[2]
                WRITTEN.update(done[3])
                continue
            im = S.image_bytes(st, i)
            ext = EXT.get(im[0], "bin") if im else "bin"
            exts.append(ext)
            mine = []
            if im:
                data = im[1]
                mine.append(os.path.join(out, "data", p, "img", f"{i}.{ext}"))
                changed = write(mine[-1], data)
                for light in S.DISPLAY_LIGHTS:
                    lit = S.image_bytes(st, i, light=light)
                    if lit:
                        mine.append(os.path.join(out, "data", p, light, f"{i}.png"))
                        write(mine[-1], lit[1])
                if Image:
                    t, v = (os.path.join(out, "data", p, d, f"{i}.jpg") for d in ("t", "v"))
                    if changed or not (os.path.isfile(t) and os.path.isfile(v)):
                        jobs[i] = pool.submit(web_copies, data, t, v)
                    else:
                        web[i] = 1
                    mine += [t, v]
                    WRITTEN.update((os.path.abspath(t), os.path.abspath(v)))
            EXPORTED[(out, p, i)] = (srcs, ext, web[i], [os.path.abspath(f) for f in mine])
        for i, job in jobs.items():
            web[i] = int(job.result())
            n_web += 1
            srcs, ext, _, mine = EXPORTED[(out, p, i)]
            EXPORTED[(out, p, i)] = (srcs, ext, web[i], mine)
        n_img += n
        # the newest whole look with a web copy (no crop, no mode), for a glimpse of the canvas elsewhere (the gallery's
        # index, the studio's picker)
        s["look"] = None
        for e in ev:
            if e["kind"] == "image" and not e.get("display_pending") and S.is_whole(e.get("look")) and 0 <= e["img"] < n and web[e["img"]]:
                s["look"] = e["img"]
        # the painting's title, if the painter's last words begin with one
        # (the closing words of each sitting: its last words before the next sitting starts, newest first)
        closings = [e["text"] for k, e in enumerate(ev) if e["kind"] == "say"
                    and all(x["kind"] != "say" for x in ev[k + 1:next((j for j in range(k + 1, len(ev)) if ev[j]["kind"] == "start"), len(ev))])]
        s["title"] = S.title_of_closings(closings) or S.runner_title(p)
        # its last event is its own words: it ended a sitting (or the painting), not cut off mid-step; the website lists a
        # quiet run as in progress only then
        s["said"] = bool(ev) and ev[-1]["kind"] == "say"
        if s["look"] is None:  # painters from before the look tool read their renders as files: the last whole one
            # they saw (studio.py mark_whole), else the last (a reference picture, read from the studio's reference/,
            # is never the painter's picture)
            mine = [e for e in reversed(ev) if e["kind"] == "image" and "look" not in e and not e.get("ref")
                    and 0 <= e["img"] < n and web[e["img"]]]
            s["look"] = next((e["img"] for e in mine if e.get("whole") is not False), mine[0]["img"] if mine else None)
        # Reuse the latest whole snapshot's web copy, re-encoded without source metadata. The content
        # version changes even if a session rewrites the same image index.
        s["og_image"] = None
        if Image and s["look"] is not None:
            with open(os.path.join(out, "data", p, "v", f'{s["look"]}.jpg'), "rb") as fh:
                data = fh.read()
            rel = f'data/{p}/v/{s["look"]}.jpg?v=' + hashlib.sha256(data).hexdigest()
            with Image.open(io.BytesIO(data)) as im:
                s["og_image"] = {"url": SITE + root + rel, "width": im.width, "height": im.height}
        text(os.path.join(out, "data", p, "events.json"),
             json.dumps({"events": ev, "total": len(ev), "epoch": st["epoch"], "sittings": s["sittings"], "imgext": exts, "web": web}))
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
    if live:
        page = page.replace(b"window.STUDIO_STATIC = true;", b"window.STUDIO_STATIC = true;window.STUDIO_LIVE = true;")
        text(os.path.join(out, "index.html"), page)
    else:
        pages = {"index.html": card_page(page, featured(ss), homepage=True, root=root)}
        for s in ss:
            pages[os.path.join("share", s["p"] + ".html")] = card_page(page, s, root=root)
        for rel, body in pages.items():
            text(inside(out, rel), external_scripts(body, out) if atelier == "only" else body)
    for asset in S.VIEWER_ASSETS:
        with open(os.path.join(S.HERE, asset), "rb") as fh:
            write(os.path.join(out, asset), fh.read())
    stale = prune(out, before)
    print(f"exported {len(ss)} painters, {n_img} images ({n_web} new web copies) to {out} ({stale} stale files removed)")


def prune(out, before):
    """Removes the files the last export listed and this one didn't write, lists this one's; returns how many it removed."""
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
    return stale


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("out")
    ap.add_argument("--skip", nargs="*", default=[], help="painter folders to leave out (test runs)")
    which = ap.add_mutually_exclusive_group()
    which.add_argument("--only-atelier", action="store_const", const="only", dest="atelier",
                       help="only the atelier's paintings, as the viewer at /atelier/easel/")
    which.add_argument("--skip-atelier", action="store_const", const="skip", dest="atelier",
                       help="every painter but the atelier's")
    ap.add_argument("--elsewhere", nargs="*", default=[],
                    help="painters working on another machine (their sessions copied here): live while recently active")
    a = ap.parse_args()
    export(a.out, skip=a.skip, atelier=a.atelier, elsewhere=a.elsewhere)


if __name__ == "__main__":
    main()
