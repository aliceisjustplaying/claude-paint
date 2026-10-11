import argparse
import base64
import html
import json
import pathlib


DESCRIPTIONS = {
    "dry": "A 14.4 mm stroke with a 2.4 mm belly, tapered at both ends. The brush is loaded with ultramarine and water. The paper starts dry.",
    "damp": "The same gesture after three moving clean-water rows have dried to damp at the center. Surrounding cells can still be wetter.",
    "wet": "The same gesture on freshly prewetted paper. Read the measured initial stage below: this is a nominal shiny-wet setup, not a guaranteed identical initial field.",
    "touch": "Blue and earth strokes run parallel, 2.4 mm apart. Their nominal full-width edges touch with zero designed overlap. Both enter a moving-brush prewet. Soft edges, tooth and flow can change the actual contact.",
    "overlap": "Both variants have a 4.8 mm designed overlap. Blue runs from 2.4 to 12 mm. Earth runs from 7.2 to 16.8 mm. The second stroke goes into the first immediately or after its center reaches damp.",
    "wash": "Ours: five alternating tapered rows at 1.6 mm spacing. Wet Edge: its flat Wash tool fills the same nominal 16.8 × 8.8 mm region with sensing, cut-in and rows, with fiat prewet disabled. Row paths are not identical."
}


def main():
    p = argparse.ArgumentParser()
    p.add_argument("output", type=pathlib.Path)
    p.add_argument("page", type=pathlib.Path)
    args = p.parse_args()
    manifest = json.loads(pathlib.Path(__file__).with_name("marks.json").read_text())
    groups = [([c for c in manifest["cases"] if c.get("group", c["id"]) == group], group)
              for group in ["dry", "damp", "wet", "touch", "overlap", "wash"]]

    def image(path):
        return "data:image/png;base64," + base64.b64encode(path.read_bytes()).decode()

    def measured(s):
        pore = s.get("pore", s.get("damp", 0))
        return f"{s['stage']} · t={s['time']:.2f} s · center film {s['water']:.4f} mm · pore {pore:.4f} mm"

    parts = []
    for number, (cases, group) in enumerate(groups, 1):
        title = "Controlled overlap at two wetness stages" if group == "overlap" else cases[0]["title"]
        parts.append(f'<section class="mark" data-mark="{group}" data-title="{html.escape(title)}"><h2>{number}. {html.escape(title)}</h2><p>{DESCRIPTIONS[group]}</p>')
        for case in cases:
            ours = json.loads((args.output / "ours" / case["id"] / "receipt.json").read_text())
            edge = json.loads((args.output / "wet-edge" / case["id"] / "receipt.json").read_text())
            if group == "overlap":
                parts.append(f'<h3>{html.escape(case["title"])}</h3>')
            initial_o = ours.get("before_second", ours.get("before_paint", ours["initial"]))
            initial_e = edge.get("before_second", edge.get("before_paint", edge["initial"]))
            parts.append(f'<p class="setup">Before {"second color" if "before_second" in ours else "paint"}:<br>Ours: {measured(initial_o)}<br>Wet Edge: {measured(initial_e)}</p>')
            for os, es in zip(ours["stages"], edge["stages"], strict=True):
                label = os["label"]
                assert label == es["label"]
                parts.append(f'<div class="stage"><h4>{dict(laid="As laid", satin="Satin checkpoint", damp="Damp checkpoint", dry="Dry checkpoint")[label]}</h4><div class="pair">')
                o = args.output / "ours" / case["id"]
                e = args.output / "wet-edge" / case["id"]
                parts.append(f'<figure><figcaption><strong>Ours</strong><span>{measured(os)}</span></figcaption><div class="paper"><img class="gallery" src="{image(o / (label + "-gallery.png"))}" alt="Our {case["id"]} at {label}, gallery light"><img class="color" src="{image(o / (label + "-color.png"))}" alt="Our {case["id"]} at {label}, color"></div></figure>')
                parts.append(f'<figure><figcaption><strong>Wet Edge · default view</strong><span>{measured(es)}</span></figcaption><div class="paper"><img src="{image(e / (label + ".png"))}" alt="Wet Edge {case["id"]} at {label}"></div></figure></div>')
                parts.append(f'<details><summary>Whole-sheet water check</summary><p>Ours: maximum film {os["max_water"]:.6f} mm, pore {os["max_pore"]:.6f} mm.<br>Wet Edge: maximum film {es["max_water"]:.6f} mm, pore {es["max_pore"]:.6f} mm. Stage names use center samples, not the entire sheet. Wet Edge dry includes its gum-setting wait.</p></details></div>')
        parts.append('<fieldset class="verdict"><legend>Your verdict for this mark</legend>')
        for engine in ["ours", "wet-edge"]:
            parts.append(f'<div>{"Ours" if engine == "ours" else "Wet Edge"}: ' + " ".join(f'<label><input type="radio" name="{group}-{engine}" value="{v}"> {v}</label>' for v in ["Pass", "Fail"]) + '</div>')
        parts.append(f'<label class="notes">Notes<textarea name="{group}-notes" rows="3" placeholder="Edges, mixing, grain, wash continuity or what would make this pass"></textarea></label></fieldset></section>')
    page = '''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Watercolor reference pilot</title>
<style>
:root{color-scheme:light;font-family:system-ui,sans-serif;color:#252624;background:#f1efe9}*{box-sizing:border-box}body{margin:0}main{max-width:900px;margin:auto;padding:30px 24px 60px}h1{font-size:30px;font-weight:600;letter-spacing:-.7px;margin:0 0 12px}h2{font-size:22px;line-height:1.3}h3{font-size:18px;margin-top:30px}h4{font-size:15px;margin:0 0 12px}p{line-height:1.6}a{color:#35557a}header{margin-bottom:28px}.eyebrow{font-size:12px;letter-spacing:1.2px;text-transform:uppercase;color:#686b61}.caveat{background:#e5e3d9;padding:16px;border-radius:6px}.controls{position:sticky;top:0;background:#f1efe9ed;backdrop-filter:blur(10px);z-index:2;padding:12px 0;display:flex;align-items:center;gap:18px;border-bottom:1px solid #c7c6bc}.controls label{cursor:pointer}button{border:1px solid #454d42;background:#394a40;color:white;padding:10px 15px;border-radius:5px;font:inherit;cursor:pointer}.mark{background:#faf9f5;border:1px solid #d9d8ce;border-radius:8px;padding:22px;margin-top:28px}.setup{font-size:12px;color:#555d53;background:#f0efe7;padding:10px}.stage{margin:25px 0;border-top:1px solid #d9d8ce;padding-top:18px}.pair{display:grid;grid-template-columns:1fr 1fr;gap:18px}figure{margin:0;min-width:0}figcaption{margin-bottom:8px;font-size:13px}figcaption span{display:block;font-size:10px;color:#596157;line-height:1.5;margin-top:5px}.paper{width:100%;max-width:384px;aspect-ratio:1.5;background:#eee}.paper img{display:block;width:100%;height:100%;object-fit:contain}body:not(.color-mode) img.color{display:none}body.color-mode img.gallery{display:none}details{font-size:12px;color:#62685e;margin-top:12px}summary{cursor:pointer}.verdict{border:0;border-top:1px solid #c7c6bc;padding:18px 0 0;margin-top:25px}.verdict legend{font-weight:600;padding:8px 0}.verdict div{margin:9px 0}.verdict label{margin-right:16px}.notes{display:block;margin-top:14px}textarea{display:block;width:100%;margin:8px 0;padding:10px;border:1px solid #b6bbb0;border-radius:4px;font:inherit;resize:vertical;background:white}#feedback{min-height:230px}#copy-status{font-size:12px}footer{padding-top:30px}.scale{width:52.0833%;max-width:200px;border-bottom:2px solid #455448;font-size:11px;text-align:center;padding:5px;margin-top:8px}.small{font-size:12px;color:#62685e}@media(max-width:620px){main{padding:20px 12px}.mark{padding:14px}.pair{grid-template-columns:1fr}.controls{flex-wrap:wrap;gap:8px;font-size:13px}.paper{max-width:none}figcaption span{font-size:12px}}
</style><main><header><div class="eyebrow">Accepted round 5 × Wet Edge / One-pass pilot</div><h1>What changes when the brush moves?</h1><p>Six test marks, seven setups, four wet-to-dry checkpoints. Judge the marks below, then copy your feedback. Nothing in either simulation core was tuned or ported.</p>
<div class="caveat"><strong>This matches physical gestures, not identical material fields.</strong><p>Each paper is 19.2 × 12.8 mm, seed 71, cold press. Ours runs at 0.05 mm per cell; Wet Edge runs at its native 0.2 mm per cell. Both images occupy the same physical scale here. Wet Edge's pixels are enlarged 4×. The pigments, paper texture, clock and application models differ. The small sheet is a harness-only dimension override.</p><p>Ours has no moving wet-brush or pressure-taper API. Its closest public-API gesture is 24 timed, variable-width, finite-load capsule segments. Wet Edge uses its moving brush and row Wash tool. No instant rectangle deposits are used. The Wash tool uses a deterministic headless path adapter and skips its optional fiat prewet.</p></div>
<p class="small">The first three marks separate dry, damp and freshly wet paper. Ours supplies 0.0015 areal pigment with a nominal 0.12 mm water dose. Wet Edge uses paint strength 0.2 and its round brush's normal 0.3 mm water level. These are not calibrated equal pigment masses or colors. The captions give measured center film and pore water so “wet” does not conceal a missed stage. Satin and damp are center checkpoints; details show whole-sheet maxima. The clocks are engine-local and should not be compared as equal elapsed real-paper time.</p>
</header><div class="controls"><strong>Ours view</strong><label><input type="radio" name="view" value="gallery" checked> Gallery</label><label><input type="radio" name="view" value="color"> Color</label><button id="copy" type="button">Copy my feedback</button><span id="copy-status" role="status"></span></div>
<div class="scale">10 mm at full-width display</div><p class="small">The same image dimensions are used for both engines. Responsive resizing changes the screen scale equally.</p>
''' + "\n".join(parts) + '''<footer><h2>Feedback to send back</h2><p>Your selections and notes are saved in this browser when storage is available. The copy button also puts the text here if clipboard access is blocked.</p><textarea id="feedback" readonly aria-label="Collected feedback"></textarea><p class="small">Sources: <a href="https://github.com/aliceisjustplaying/claude-paint/tree/f75ed9aa57d7e9a5050914221dec777a1cf50cbe/experiments/watercolor">ours f75ed9a</a> and <a href="https://github.com/lambentLogic/wet-edge/tree/84aa409d5032a6fc1b613665b4626be1cf410eee">Wet Edge 84aa409</a>, both MIT. These are simulator outputs. No real-paint photos are included because no matched CC0/CC-BY photo set was established for this pilot.</p></footer></main>
<script>
const key='watercolor-reference-f75ed9a-84aa409-v1';
const fields=[...document.querySelectorAll('.mark input,.mark textarea')];
try{const saved=JSON.parse(localStorage.getItem(key)||'{}');for(const f of fields){if(f.type==='radio')f.checked=saved[f.name]===f.value;else f.value=saved[f.name]||'';}}catch{}
for(const f of fields)f.addEventListener('input',()=>{const saved={};for(const x of fields){if(x.type!=='radio'||x.checked)saved[x.name]=x.value;}try{localStorage.setItem(key,JSON.stringify(saved));}catch{}});
for(const input of document.querySelectorAll('[name=view]'))input.addEventListener('change',()=>document.body.classList.toggle('color-mode',input.value==='color'));
document.getElementById('copy').addEventListener('click',async()=>{
 const text=['Watercolor reference pilot — f75ed9a / 84aa409',''];
 for(const section of document.querySelectorAll('.mark')){
  const id=section.dataset.mark;
  text.push(section.querySelector('h2').textContent);
  for(const engine of ['ours','wet-edge'])text.push(`${engine==='ours'?'Ours':'Wet Edge'}: ${section.querySelector(`input[name="${id}-${engine}"]:checked`)?.value||'Unrated'}`);
  text.push(`Notes: ${section.querySelector('textarea').value.trim()||'(none)'}`,'');
 }
 const output=document.getElementById('feedback');output.value=text.join('\n');
 try{await navigator.clipboard.writeText(output.value);document.getElementById('copy-status').textContent='Feedback copied.';}
 catch{output.focus();output.select();document.getElementById('copy-status').textContent='Feedback collected below. Copy the selected text.';}
});
</script></html>'''
    args.page.parent.mkdir(parents=True, exist_ok=True)
    args.page.write_text(page)
    print(args.page, args.page.stat().st_size)


if __name__ == "__main__":
    main()
