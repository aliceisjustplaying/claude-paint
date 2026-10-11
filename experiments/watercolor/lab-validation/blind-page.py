import argparse
import base64
import json
import random
from pathlib import Path

from PIL import Image

parser = argparse.ArgumentParser()
parser.add_argument("original", type=Path)
parser.add_argument("port", type=Path)
parser.add_argument("output", type=Path)
parser.add_argument("key", type=Path)
args = parser.parse_args()
recipe = json.loads(Path(__file__).with_name("marks.json").read_text())
if args.key.exists():
    key = json.loads(args.key.read_text())
else:
    assignments = ["port"] * 4 + ["original"] * 3
    random.SystemRandom().shuffle(assignments)
    key = {mark["name"]: engine for mark, engine in zip(recipe["marks"], assignments)}
    args.key.parent.mkdir(parents=True, exist_ok=True)
    args.key.write_text(json.dumps(key, indent=2))

parts = ["""<!doctype html><html lang="en"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1"><title>Watercolor comparison</title>
<style>body{font:16px/1.5 system-ui,sans-serif;max-width:1160px;margin:24px auto;padding:0 16px;background:#f6f4ef;color:#222}h1{font-size:24px}h2{font-size:19px;border-top:1px solid #ccc;padding-top:20px}section{margin-top:32px}.pair{display:grid;grid-template-columns:1fr 1fr;gap:12px}figure{margin:0}figcaption{font-weight:650}img{width:100%;display:block;border:1px solid #ddd}.pending{aspect-ratio:3/2;display:grid;place-items:center;background:#eee;border:1px solid #ddd;color:#666}.question{background:white;border:1px solid #ddd;padding:14px;margin-top:14px}.question p{margin:5px 0}label{margin-right:12px}textarea{width:100%;box-sizing:border-box;min-height:72px;font:inherit;margin-top:10px}button{font:inherit;padding:10px 16px}#copy{margin:24px 0}#feedback{display:none;min-height:200px}.warning{background:#fff4d6;padding:12px}@media(max-width:600px){.pair{grid-template-columns:1fr}}</style></head><body>
<h1>Watercolor comparison</h1><p>Look at each pair as the paint changes. A and B are shuffled for each mark, so the same letter does not always mean the same version. Both images use the same view. Tell us which feels more like real watercolor and what you notice.</p>
<p>What can still give it away: one version may look a little darker or spread a little farther. Small grain patterns and tiny edge details may differ. The paint may reach a different degree of dryness at the same moment. Some middle samples still carry water, so the stages are numbered instead of named.</p>
"""]
complete = True
for number, mark in enumerate(recipe["marks"], 1):
    name = mark["name"]
    available = all((directory / f"{name}-{stage['name']}.png").exists()
                    for directory in [args.original, args.port] for stage in recipe["stages"])
    complete &= available
    parts.append(f'<section data-mark="{number}"><h2>Mark {number}</h2>')
    for stage_number, stage in enumerate(recipe["stages"], 1):
        parts.append(f'<p>Stage {stage_number}</p><div class="pair">')
        for letter, engine in [("A", key[name]), ("B", "original" if key[name] == "port" else "port")]:
            file = (args.original if engine == "original" else args.port) / f"{name}-{stage['name']}.png"
            parts.append(f'<figure><figcaption>{letter}</figcaption>')
            if available:
                with Image.open(file) as image:
                    if image.size != (recipe["width"], recipe["height"]):
                        raise ValueError(f"Wrong dimensions: {file}")
                data = base64.b64encode(file.read_bytes()).decode()
                parts.append(f'<img alt="Mark {number}, stage {stage_number}, {letter}" src="data:image/png;base64,{data}">')
            else:
                parts.append('<div class="pending">Images are still being captured</div>')
            parts.append('</figure>')
        parts.append('</div>')
    disabled = "" if available else " disabled"
    parts.append('<div class="question"><p>Which looks more like real watercolor?</p>')
    for answer in ["A", "B", "same", "neither"]:
        parts.append(f'<label><input type="radio" name="{number}-choice" value="{answer}"{disabled}>{answer}</label>')
    for letter in ["A", "B"]:
        parts.append(f'<p>{letter}: ')
        for answer in ["pass", "fail"]:
            parts.append(f'<label><input type="radio" name="{number}-{letter}" value="{answer}"{disabled}>{answer}</label>')
        parts.append('</p>')
    parts.append(f'<textarea name="{number}-notes" aria-label="Notes for mark {number}" placeholder="Notes: what looks right or wrong"{disabled}></textarea></div></section>')
if not complete:
    parts.insert(1, '<p class="warning">This comparison is not ready. Please wait until all images appear before judging.</p>')
parts.append(r"""<button id="copy">Copy my feedback</button><textarea id="feedback" aria-label="Feedback to copy"></textarea>
<script>
const storage='watercolor-pairs-feedback';
const fields=[...document.querySelectorAll('input,textarea[name]')];
let saved={};try{saved=JSON.parse(localStorage.getItem(storage)||'{}')}catch{}
fields.forEach(e=>{if(e.type==='radio')e.checked=saved[e.name]===e.value;else e.value=saved[e.name]||'';e.addEventListener('input',save)});
function save(){saved={};fields.forEach(e=>{if(e.type==='radio'){if(e.checked)saved[e.name]=e.value}else if(e.value)saved[e.name]=e.value});try{localStorage.setItem(storage,JSON.stringify(saved))}catch{}}
document.getElementById('copy').onclick=async()=>{save();let text='Watercolor comparison feedback\n';document.querySelectorAll('section').forEach(s=>{let n=s.dataset.mark;text+=`\nMark ${n}\nWhich looks more like real watercolor: ${saved[n+'-choice']||'-'}\nA: ${saved[n+'-A']||'-'}; B: ${saved[n+'-B']||'-'}\nNotes: ${saved[n+'-notes']||'-'}\n`});try{await navigator.clipboard.writeText(text);document.getElementById('copy').textContent='Copied'}catch{let out=document.getElementById('feedback');out.style.display='block';out.value=text;out.select()}};
</script></body></html>""")
args.output.parent.mkdir(parents=True, exist_ok=True)
args.output.write_text("".join(parts))
print("ready" if complete else "incomplete: missing captures")
