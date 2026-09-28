# /// script
# dependencies = ["openpyxl"]
# ///
import json, datetime
from openpyxl import Workbook
from openpyxl.styles import Font, PatternFill, Alignment
from openpyxl.utils import get_column_letter
D = "model-costs"
OUT = "notes/costs/vision_models_2026-09-28.xlsx"
models = json.load(open(f"{D}/or_models.json"))["data"]
runs = json.load(open(f"{D}/runs.json"))
OURS = {"anthropic/claude-opus-5.5": "r10-r17 (Opus 5.5)", "anthropic/claude-fable-5.1": "r11", "openai/gpt-6-astra": "r11, audits",
        "google/gemini-3.8-flash": "r11, r16, r18, r19", "openai/gpt-6-luna": "r19 (via ChatGPT subscription)",
        "moonshotai/kimi-k3": "r19 (via Go)", "xiaomi/mimo-v2.6-pro": "r18, r19 (via Go)", "meta/muse-spark-1.3": "r18, r19 (via Zen)",
        "z-ai/glm-5.3-flash": "r18 (via Go)", "deepseek/deepseek-v4.1-flash": "r18 (via Go)", "stealth/space-bunny-alpha": "r18 as space-bunny-free (via Go)"}
wb = Workbook()
bold = Font(bold=True); hdr = PatternFill("solid", fgColor="DDD6C8"); ours_fill = PatternFill("solid", fgColor="F3E6B8")

# --- Profiles
pr = wb.active; pr.title = "Profiles"
pr["A1"] = "Token profile of one painting in 4 sittings (edit the blue cells; every cost recalculates)"; pr["A1"].font = bold
pr.append([])
pr.append(["profile", "prompt tokens (all)", "output tokens", "images", "based on"])
for c in pr[3]: c.font = bold; c.fill = hdr
pr.append(["light", 4_600_000, 72_000, 55, "r19 Luna F (GPT-6 Luna, stopped early: 36 chunks)"])
pr.append(["typical", 40_000_000, 220_000, 200, "median of the 9 full 4-sitting paintings (rounds 17-19)"])
pr.append(["heavy", 75_000_000, 600_000, 300, "r19 Gemini B (57M, 654K), r18 Space Bunny (90M, 585K)"])
pr.append([])
pr.append(["prompt split", "share"]); pr["A8"].font = bold
pr.append(["cache read", 0.89]); pr.append(["uncached input", 0.08]); pr.append(["cache write", 0.03])
pr.append(["(measured: Opus 97% read + 3-10% writes; Gemini 92%; DeepSeek 85%; Muse 95%)"])
for r in range(4, 7):
    for c in "BCD": pr[f"{c}{r}"].font = Font(color="1F4E9A"); pr[f"{c}{r}"].number_format = "#,##0"
for r in range(9, 12): pr[f"B{r}"].font = Font(color="1F4E9A"); pr[f"B{r}"].number_format = "0%"
for col, w in zip("ABCDE", (16, 20, 16, 10, 60)): pr.column_dimensions[col].width = w
P = {"light": 4, "typical": 5, "heavy": 6}

# --- Models
ms = wb.create_sheet("Models")
cols = ["model id", "name", "used by us", "tool calls", "text out", "context", "kind", "$/M input", "$/M output",
        "$/M cache read", "$/M cache write", "$/image", "light $", "typical $", "heavy $", "typical $ no cache", "created", "expires"]
ms.append(cols)
for c in ms[1]: c.font = bold; c.fill = hdr; c.alignment = Alignment(wrap_text=True, vertical="top")
def f(x):
    try: return float(x)
    except (TypeError, ValueError): return None
rows = []
for m in models:
    if "image" not in (m.get("architecture", {}).get("input_modalities") or []): continue
    p = m.get("pricing", {})
    mid = m["id"]
    kind = ("batch (not interactive)" if mid.endswith(":batch") else "alias (latest)" if mid.startswith("~")
            else "router / variable price" if f(p.get("prompt")) is not None and f(p.get("prompt")) < 0 else "free" if mid.endswith(":free") or (f(p.get("prompt")) == 0 and f(p.get("completion")) == 0) else "")
    per_m = lambda k: (f(p.get(k)) * 1e6 if f(p.get(k)) is not None and f(p.get(k)) >= 0 else None)
    rows.append([mid, m.get("name"), OURS.get(mid, ""), "yes" if "tools" in (m.get("supported_parameters") or []) else "no",
                 "yes" if "text" in (m.get("architecture", {}).get("output_modalities") or []) else "no", m.get("context_length"), kind,
                 per_m("prompt"), per_m("completion"), per_m("input_cache_read"), per_m("input_cache_write"),
                 f(p.get("image")) if f(p.get("image")) and f(p.get("image")) > 0 else None,
                 datetime.date.fromtimestamp(m["created"]).isoformat() if m.get("created") else "", m.get("expiration_date") or ""])
def est(r):  # for sorting: the typical cost in Python (the sheet has formulas)
    i, o, cr, cw = r[7], r[8], r[9], r[10]
    if i is None or o is None: return 1e9
    t = 40e6
    return (t * .08 * i + t * .89 * (cr if cr is not None else i) + t * .03 * (cw if cw is not None else i) + 220e3 * o) / 1e6
rows.sort(key=lambda r: (r[3] != "yes" or r[6].startswith(("batch", "router")), est(r)))
for n, r in enumerate(rows, start=2):
    ms.append(r[:12] + [None, None, None, None] + r[12:])
    def cost(prof, nocache=False):
        row = P[prof]
        inp = f"H{n}"; out = f"I{n}"
        cr = inp if nocache else f'IF(J{n}="",H{n},J{n})'
        cw = inp if nocache else f'IF(K{n}="",H{n},K{n})'
        img = f'IF(L{n}="",0,L{n})'
        return (f'=IF(OR(H{n}="",I{n}=""),"",(Profiles!$B${row}*(Profiles!$B$10*{inp}+Profiles!$B$9*{cr}+Profiles!$B$11*{cw})'
                f'+Profiles!$C${row}*{out})/1000000+Profiles!$D${row}*{img})')
    ms[f"M{n}"] = cost("light"); ms[f"N{n}"] = cost("typical"); ms[f"O{n}"] = cost("heavy"); ms[f"P{n}"] = cost("typical", True)
    for c in "HIJKL": ms[f"{c}{n}"].number_format = "0.000"
    for c in "MNOP": ms[f"{c}{n}"].number_format = "$#,##0.00"
    ms[f"F{n}"].number_format = "#,##0"
    if r[2]:
        for c in ms[n]: c.fill = ours_fill
widths = (38, 34, 22, 8, 7, 11, 18, 9, 9, 9, 9, 9, 10, 10, 10, 12, 11, 11)
for i, w in enumerate(widths, 1): ms.column_dimensions[get_column_letter(i)].width = w
ms.freeze_panes = "C2"; ms.auto_filter.ref = f"A1:R{len(rows) + 1}"

# --- Our models only
om = wb.create_sheet("Our models")
om.append(["model id", "used", "light $", "typical $", "heavy $", "typical $ no cache"]); [setattr(c, "font", bold) or setattr(c, "fill", hdr) for c in om[1]]
for n, r in enumerate(rows, start=2):
    if r[2]:
        k = om.max_row + 1
        om.append([r[0], r[2], f"=Models!M{n}", f"=Models!N{n}", f"=Models!O{n}", f"=Models!P{n}"])
        for c in "CDEF": om[f"{c}{k}"].number_format = "$#,##0.00"
for i, w in enumerate((34, 34, 10, 10, 10, 16), 1): om.column_dimensions[get_column_letter(i)].width = w

# --- Measured
me = wb.create_sheet("Measured paintings")
me.append(["painting", "model (provider/id as run)", "sittings worked", "requests", "uncached input", "cache read", "cache write", "output", "images", "chunks", "OpenRouter id", "$ at OpenRouter list price"])
for c in me[1]: c.font = bold; c.fill = hdr
ORID = {"claude-opus-5-5": "anthropic/claude-opus-5.5", "space-bunny-free": "stellar", "deepseek-v4.1-flash": "deepseek/deepseek-v4.1-flash",
        "glm-5.3-flash": "z-ai/glm-5.3-flash", "mimo-v2.6-pro": "xiaomi/mimo-v2.6-pro", "muse-spark-1.3": "meta/muse-spark-1.3",
        "gemini-3.8-flash": "google/gemini-3.8-flash", "gpt-6-luna": "openai/gpt-6-luna", "kimi-k3": "moonshotai/kimi-k3"}
ORID["space-bunny-free"] = "stealth/space-bunny-alpha"
byid = {m["id"]: m for m in models}
for k, t in runs.items():
    oid = ORID.get(t["model"].split("/", 1)[1], "")
    pr_ = byid.get(oid, {}).get("pricing", {})
    g = lambda key, dflt=None: f(pr_.get(key)) if f(pr_.get(key)) is not None else dflt
    i = g("prompt")
    usd = None if i is None else (t["input"] * i + t["cache_read"] * g("input_cache_read", i) + t["cache_write"] * g("input_cache_write", i)
                                  + t["output"] * g("completion") + t["images"] * g("image", 0))
    me.append([k, t["model"], t["sittings"], t["requests"], t["input"], t["cache_read"], t["cache_write"], t["output"], t["images"], t["chunks"], oid, usd])
for row in me.iter_rows(min_row=2):
    for c in row[3:10]: c.number_format = "#,##0"
    row[11].number_format = "$#,##0.00"
me.column_dimensions["K"].width = 30; me.column_dimensions["L"].width = 14
for i, w in enumerate((14, 36, 9, 9, 13, 13, 12, 11, 8, 8), 1): me.column_dimensions[get_column_letter(i)].width = w

# --- Notes
nt = wb.create_sheet("Notes")
for line in [
    "Vision models on OpenRouter and what one painting would cost there (built 2026-09-28).",
    "Source: https://openrouter.ai/api/v1/models, fetched 2026-09-28: every model whose input modalities include image (list prices, $ per million tokens).",
    "Painting cost = prompt tokens x (uncached share x input price + cache-read share x cache-read price + cache-write share x cache-write price) + output tokens x output price + images x image price.",
    "No cache-read or cache-write price listed: those tokens are charged at the input price. 'typical $ no cache' charges the whole prompt at the input price (if caching doesn't work through a provider).",
    "Output includes reasoning tokens (they're billed as output). Thinking level changes output a lot: our runs were high to max.",
    "Token counts are from our session logs (Measured paintings); other models tokenize text and images differently, so treat every cost as a ballpark, x0.5 to x2.",
    "Painters need tool calls ('tool calls' = yes) and a long context (sessions reached 330K tokens before compaction; compaction works, tested 2026-09-28).",
    "Batch rows are half price but not interactive (not usable for painting). Router rows have variable prices.",
    "Rows shaded yellow are models we have painted with; 'used by us' says where. Subscriptions (ChatGPT for Luna, OpenCode Go/Zen) were cheaper than these list prices.",
]: nt.append([line])
nt.column_dimensions["A"].width = 160
wb.move_sheet("Notes", offset=-4)
wb.save(OUT)
print(OUT, len(rows), "vision models")
