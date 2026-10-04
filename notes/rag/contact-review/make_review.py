from pathlib import Path
from PIL import Image,ImageDraw,ImageFont,ImageChops
import csv,json
root=Path(__file__).resolve().parent
font=ImageFont.truetype('/System/Library/Fonts/Supplemental/Arial.ttf',24)
small=ImageFont.truetype('/System/Library/Fonts/Supplemental/Arial.ttf',18)
def sheet(name,items):
    im=Image.new('RGB',(1296,len(items)*340),'#202020');d=ImageDraw.Draw(im)
    for j,(path,title) in enumerate(items):
        d.text((12,j*340+8),title,font=font,fill='#f1ece4')
        im.paste(Image.open(root/path).convert('RGB').crop((0,285,1296,580)),(0,j*340+42))
    im.save(root/name)
sheet('finding.png',[
 ('baseline/initial-film.png','Starting paint thickness — the islands exist before wiping'),
 ('baseline/after.png','Original paint, one wipe — 12.38% film remains in the center'),
 ('uniform/after.png','Uniform starting thickness, same rag and wipe — 12.44% remains')])
sheet('cause-control.png',[
 ('baseline/after.png','Original brush preparation, unchanged rag'),
 ('no-plough/after.png','Brush paint-pushing disabled during preparation, unchanged rag')])
sheet('diagnosis.png',[
 ('baseline/initial-film.png','Starting thickness — white = 100 µm; larger values clip'),
 ('baseline/contact.png','Accumulated cloth contact — white = 1.0 exposure'),
 ('baseline/pickup.png','Cumulative pickup — white = 100 µm; larger values clip'),
 ('baseline/deposit.png','Cumulative redeposition — white = 5 µm; larger values clip'),
 ('baseline/film.png','Remaining thickness — white = 100 µm; larger values clip')])
# A second redeposition view uses its actual peak so clipping cannot hide structure.
# It is written below from the saved floating-point field before that scratch data is removed.
rows=list(csv.DictReader((root/'baseline/cloth-shape.csv').open()))
shapes={}
for r in rows: shapes.setdefault(int(r['frame']),[]).append(tuple(float(r[k]) for k in ['x_mm','y_mm','z_mm']))
im=Image.new('RGB',(1296,700),'#202020');d=ImageDraw.Draw(im)
for frame,nodes in list(shapes.items())[:4]:
    col,row=frame%2,frame//2; ox,oy=col*648+324,row*350+190
    d.text((col*648+18,row*350+12),f'Cloth shape after {frame*44+0.5:.1f} mm travel',font=font,fill='#eee')
    def project(p):return (ox+5.5*(p[0]-0.55*p[1]),oy+2.2*p[1]-18*p[2])
    # Height is exaggerated relative to horizontal dimensions for visibility.
    for i,p in enumerate(nodes):
        for j in ([i+1] if i%19<18 else [])+([i+19] if i//19<18 else []):
            color='#dfbc79' if min(p[2],nodes[j][2])<0.18 else '#819aae'
            d.line([project(p),project(nodes[j])],fill=color,width=1)
    d.text((col*648+18,row*350+316),'Warm lines: an endpoint within 0.18 mm of the plane',font=small,fill='#bbb')
im.save(root/'cloth-shape.png')
checks=[]
a=Image.open(root/'baseline/after.png');b=Image.open(root.parent/'pickup-review/quarter/damp-1.png')
checks.append(f'Instrumented baseline matches previous quarter-rate wipe: {ImageChops.difference(a,b).getbbox() is None}')
for case in ['uniform','no-plough']:
    checks.append(f'{case}: accumulated contact map identical to baseline: {(root/case/"contact.f32").read_bytes()==(root/"baseline/contact.f32").read_bytes()}')
(root/'verification.txt').write_text('\n'.join(checks)+'\n')
(root/'index.html').write_text('''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Rag islands: traced to starting paint</title><style>
:root{color-scheme:dark}body{max-width:1296px;margin:32px auto;padding:0 20px;background:#202020;color:#eee;font:17px/1.5 system-ui}h1{font-size:30px}h2{font-size:22px}p{max-width:960px}a{color:#bcdafa}img{width:100%;height:auto;display:block}figure{margin:24px 0}figcaption{font-size:15px;color:#c8c1b7}details{margin:26px 0}summary{font-weight:600;cursor:pointer}table{border-collapse:collapse}td,th{padding:10px 18px;border-bottom:1px solid #555;text-align:left}.note{padding-left:16px;border-left:3px solid #c3a575}</style>
<h1>The islands were already in the paint</h1>
<p>The repeated outlines are raised brush marks in the starting layer. The wipe makes them visible. With the same cloth and wipe, a uniform starting thickness loses the islands while retaining nearly the same amount of paint.</p>
<figure><a href="finding.png"><img src="finding.png" alt="Starting thickness has islands; original paint reveals them after wiping; uniform-thickness control has no outlined islands"></a><figcaption>All wipes use the previous quarter pickup rate (LIFT = 1.0), pressure 0.8 and solvent dip 0.5. The uniform-thickness row is a diagnostic control, not a production fix. Grayscale thickness is not a photograph or rendered paint color.</figcaption></figure>
<h2>What this establishes</h2>
<p>The cloth-contact hypothesis did not survive the controls. The three runs below have identical accumulated contact maps. Changing the starting thickness removes the islands. Disabling only the brush’s paint-pushing step during preparation also removes the repeated outlines. The rag itself is unchanged.</p>
<table><tr><th>Starting layer</th><th>Mean starting thickness</th><th>Mean thickness after wiping</th><th>Film remaining</th></tr><tr><td>Original brush preparation</td><td>69.29 µm</td><td>8.58 µm</td><td>12.38%</td></tr><tr><td>Uniform-thickness control</td><td>69.29 µm</td><td>8.62 µm</td><td>12.44%</td></tr><tr><td>Brush paint-pushing disabled</td><td>70.39 µm</td><td>8.75 µm</td><td>12.43%</td></tr></table>
<p>Measurements use the same central band, x = 350–650 and y = 310–370 canvas units. <a href="measurements.json">Exact measurements</a> · <a href="verification.txt">Replay and contact checks</a>.</p>
<p class="note">This identifies where the pattern comes from. It does not establish that disabling paint-pushing is a correct brush model, that a uniform layer represents every painting or that the rag is physically calibrated. No cloth rewrite or production brush fix was retained.</p>
<details><summary>Brush paint-pushing control</summary><figure><a href="cause-control.png"><img src="cause-control.png" alt="Same rag over original paint versus paint prepared with brush pushing disabled"></a><figcaption>Only the preparing brush’s push coefficient changes to zero. Its paint pickup and deposition remain active; the rag settings stay fixed. This control also changes the starting film distribution and reveals more linen in places.</figcaption></figure></details>
<details><summary>Contact, pickup and redeposition separated</summary><figure><a href="diagnosis.png"><img src="diagnosis.png" alt="Separate starting thickness, accumulated contact, pickup, redeposition and remaining thickness maps"></a></figure><p>These fields were recorded during the same wipe, with no changes to its arithmetic. The resulting paint image matches the earlier quarter-rate image pixel for pixel. Starting thickness − cumulative pickup + cumulative redeposition reconstructs the remaining film with a maximum per-pixel error of 0.00035 µm. <a href="baseline/accounting.txt">Accounting check</a>.</p><p>Paint can be picked up and deposited more than once, so the cumulative maps are not final deposited layers. The separate <a href="deposit-unclipped.png">unclipped redeposition view</a> reveals its structure without saturating bright areas.</p></details>
<details><summary>The unchanged cloth shape</summary><figure><a href="cloth-shape.png"><img src="cloth-shape.png" alt="Four wireframe views of the deforming cloth during the wipe"></a><figcaption>Height is exaggerated for visibility. These are actual cloth nodes captured during the baseline wipe, not a replacement shape. <a href="baseline/cloth-shape.csv">Coordinates</a>.</figcaption></figure></details>
<h2>Correction to the research setup</h2>
<p>The starting film’s thickness now appears beside the wipe. The uniform layer isolates rag behavior; the original brush layer remains as a separate textured-paint case. This prevents a brush-preparation artifact from being treated as a cloth defect.</p>
<p>An experimental change that deferred brush transfers until the end of each bristle step did not remove the outlines and was reverted. <a href="transport-check.txt">Result</a>. Changing the order of transport was insufficient; there is no validated production correction to brush displacement from this round.</p>
<details><summary>Sources and reproduction</summary><p><a href="research.patch">Unapplied research patch against main 4e525e5</a> includes the previous cloth prototype and temporary recording code, not a release candidate. <a href="trace.rs">Render source</a> · <a href="probe.rs">Recording and uniform-thickness control</a> · <a href="make_review.py">Figure source</a>.</p><p>Images: <a href="baseline/after.png">original wipe</a> · <a href="uniform/after.png">uniform control</a> · <a href="no-plough/after.png">brush-pushing control</a> · <a href="baseline/untouched.png">original starting color</a> · <a href="baseline/initial-film.png">original starting thickness</a>.</p><p>Development iteration profile; diagnostic checks only. These comparisons provide causal evidence about the artifact, not physical calibration or release validation. Shipping source was restored.</p></details></html>''')
