from pathlib import Path
from PIL import Image, ImageDraw, ImageFont, ImageChops
import csv
root=Path(__file__).resolve().parent
font=ImageFont.truetype('/System/Library/Fonts/Supplemental/Arial.ttf',22)
small=ImageFont.truetype('/System/Library/Fonts/Supplemental/Arial.ttf',18)
bold=ImageFont.truetype('/System/Library/Fonts/Supplemental/Arial Bold.ttf',24)
settings=[('current','Current rate'),('half','Half rate'),('quarter','Quarter rate')]
data={k:list(csv.DictReader((root/k/'measurements.csv').open())) for k,_ in settings}
def measurement(k,kind,p):
    return next(r for r in data[k] if r['case']==kind and r['pass']==str(p) and r['region']=='core')
# Native images remain untouched. Sheets use identical crops and downsampling.
for kind in ['damp','dry']:
    sheet=Image.new('RGB',(1320,962),'#202020'); draw=ImageDraw.Draw(sheet)
    draw.text((18,15),'Solvent-damp rag' if kind=='damp' else 'Dry rag',font=bold,fill='#f0ece5')
    draw.text((18,49),'Same cloth, paint, pressure and path. Only pickup rate changes.',font=font,fill='#c2bdb5')
    draw.text((18,88),'Untouched paint — same starting image for every case',font=small,fill='#eee')
    before=Image.open(root/'current/untouched.png').crop((0,285,1296,405))
    sheet.paste(before,(12,118))
    draw.text((18,256),'After one pass',font=bold,fill='#f0ece5')
    draw.text((678,256),'After two passes, same rag face',font=bold,fill='#f0ece5')
    for row,(k,label) in enumerate(settings):
        y=298+row*218
        for p in [1,2]:
            x=12+(p-1)*660
            r=measurement(k,kind,p)
            draw.text((x+6,y),f'{label} · {float(r["film_left_percent"]):.1f}% film left in center',font=small,fill='#e6ded0')
            im=Image.open(root/k/f'{kind}-{p}.png').crop((0,270,1296,620)).resize((648,175),Image.Resampling.LANCZOS)
            sheet.paste(im,(x,y+30))
    sheet.save(root/f'{kind}-comparison.png')
# Exact region annotation on an untouched copy, separate from all comparisons.
im=Image.open(root/'current/untouched.png').copy(); d=ImageDraw.Draw(im)
# Window origin is (230,160) units at 2.4 pixels/unit.
box=tuple(round((v-origin)*2.4) for v,origin in zip([350,310,650,370],[230,160,230,160]))
d.rectangle(box,outline='#66e5ff',width=3); d.text((box[0]+8,box[1]+8),'Measured central band',font=font,fill='#66e5ff'); im.save(root/'measurement-region.png')
checks=[]
for k,_ in settings:
    for name in ['untouched','ground']:
        same=ImageChops.difference(Image.open(root/'current'/f'{name}.png'),Image.open(root/k/f'{name}.png')).getbbox() is None
        assert same
        checks.append(f'{k}/{name}: identical pixels to current = {same}')
with (root/'verification.txt').open('a') as f:f.write('\n'.join(checks)+'\n')
rows=''
for k,label in settings:
    a,b=measurement(k,'damp',1),measurement(k,'damp',2)
    d=measurement(k,'dry',1)
    rows+=f'<tr><th>{label}</th><td>{float(a["film_left_percent"]):.1f}%</td><td>{float(b["film_left_percent"]):.1f}%</td><td>{float(a["color_left_percent"]):.1f}%</td><td>{float(d["film_left_percent"]):.1f}%</td></tr>'
links=''.join(f'<li>{label}: <a href="{k}/dry-1.png">dry 1</a> · <a href="{k}/dry-2.png">dry 2</a> · <a href="{k}/damp-1.png">damp 1</a> · <a href="{k}/damp-2.png">damp 2</a> · <a href="{k}/measurements.csv">measurements</a></li>' for k,label in settings)
(root/'index.html').write_text('''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Rag pickup comparison</title><style>
:root{color-scheme:dark}body{max-width:1320px;margin:32px auto;padding:0 20px;background:#202020;color:#eee;font:17px/1.5 system-ui}h1{font-size:30px;margin-bottom:8px}h2{font-size:22px}p{max-width:950px}a{color:#bedbfa}img{display:block;width:100%;height:auto}figure{margin:24px 0}figcaption{color:#c7c1b9;font-size:15px;margin-top:8px}table{border-collapse:collapse;width:100%;font-variant-numeric:tabular-nums}th,td{border-bottom:1px solid #555;padding:12px;text-align:left}thead{color:#cbc3b7}details{margin:28px 0}summary{cursor:pointer;font-weight:600}.table{overflow-x:auto}.note{border-left:3px solid #b9a17c;padding-left:16px}li{margin:10px 0}</style>
<h1>Rag: how much should one wipe remove?</h1>
<p>Three pickup rates, with the same deforming cloth. Lower rates leave more paint for a second damp pass to remove, while also making the dry wipe weaker. These are sensitivity comparisons, not calibrated material settings.</p>
<figure><a href="damp-comparison.png"><img src="damp-comparison.png" alt="Current, half and quarter pickup rate; one and two solvent-damp passes"></a><figcaption>All panels are after wiping except the explicitly labeled untouched strip. Wipe panels use identical crops and scale; colors are unadjusted. Percentages describe the measured central band, not the whole image.</figcaption></figure>
<div class="table"><table><thead><tr><th>Pickup rate</th><th>Film left: damp, 1 pass</th><th>Film left: damp, 2 passes</th><th>Darkness left: damp, 1 pass</th><th>Film left: dry, 1 pass</th></tr></thead><tbody>'''+rows+'''</tbody></table></div>
<p>Each percentage is relative to its own untouched starting value in the same central band. The starting film averages 69.3 µm there. The current rate leaves 1.38 µm after one damp pass and 1.05 µm after two. Film thickness and visible darkness are different measurements.</p>
<details><summary>Dry rag: the same three pickup rates</summary><figure><a href="dry-comparison.png"><img src="dry-comparison.png" alt="Dry wipes at current, half and quarter pickup rates, after one and two passes"></a></figure></details>
<h2>What the real reference supports</h2>
<p><a href="https://rpalescafineart.com/blogs/in-the-studio/how-to-create-a-wipe-out-underpainting-in-oil">Palesca’s wipe-out demonstration, steps 9–12</a>, starts with paint spread as thinly as possible, lifts lights with a rag, then uses mineral spirits for stronger clearing. She says a further wipe with a dry part of the rag may be needed. This supports strong clearing and additional removal on another wipe.</p>
<p class="note">It does not establish the right rate here: our repeat uses the same solvent-damp face, our starting film is simulated and the demonstration provides no measured thickness or removal per pass. None of these three rates is established as physically accurate.</p>
<p><a href="https://www.jacksonsart.com/en-us/a-guide-to-grounds">Jackson’s guide to grounds</a> also describes smooth, relatively nonabsorbent grounds as supporting wipe-back techniques. A matched physical comparison would need the ground as well as the paint and wiping procedure specified.</p>
<details><summary>Measurement method and full-size originals</summary><p>The central band is x = 350–650, y = 310–370 canvas units, fixed across all runs and away from the stroke’s starting, ending and side edges. Film is the mean wet thickness sampled directly from the canvas. Darkness is the mean linear-light luminance deficit relative to the primed ground, divided by that of the untouched paint. It is an image measurement, not a pigment-mass estimate.</p><a href="measurement-region.png"><img src="measurement-region.png" alt="Cyan rectangle marks the central band used for measurements"></a><p>The CSVs also contain a larger envelope (x = 250–750, y = 280–400), including stroke edges and some untouched paint. Those values are not used in this page’s table.</p><ul>'''+links+'''</ul><p><a href="current/untouched.png">Untouched paint</a> · <a href="current/ground.png">Primed ground</a></p></details>
<details><summary>Reproducibility and limits</summary><p>Only LIFT changes: 4.0, 2.0 or 1.0. The damp multiplier, cloth, seeds, paint, pressure, path, sampling and paint-transfer rules remain fixed. Pressure 0.8; solvent dip 0.5; 2400-pixel canvas width. No refolding between passes. Each dry/damp sequence starts from the same paint and a fresh rag.</p><p>The current-rate rerender is pixel-identical to all five earlier material-review originals. All three settings also share pixel-identical starting paint and ground. <a href="verification.txt">Checks</a> · <a href="settings.json">Settings</a> · <a href="render.rs">Render and measurement source</a> · <a href="../material-review/prototype.patch">Base prototype patch</a>.</p><p>This uses the development iteration profile. No release-validation claim is made. Shipping source was restored after the comparison; none of these rate changes was selected or shipped.</p></details>
</html>''')
