import numpy as np
from PIL import Image
def L(p):
    s=np.asarray(Image.open(p).convert('RGB')).astype(float)/255
    return np.where(s<=0.04045,s/12.92,((s+0.055)/1.055)**2.4)
rows=[('rock_H (coats 0.3)','rock_orig','rock_fix','rock_relief'),
      ('oakleaf_H (coats 0.3 restored)','oak_old','oak_new','oak_rel'),
      ('l5_near crop','l5c_old','l5c_new','l5c_rel'),
      ('l3_green crop','l3c_old','l3c_new','l3c_rel')]
print('| 3200 study | mean varnish transmittance R,G,B old / new | spread of blue transmittance (std) old / new | pixels whose varnish is >2x the mean darkening old / new |')
print('|---|---|---|---|')
for n,o,nw,r in rows:
    R=L(f'out/{r}.png'); ok=R.min(2)>0.01; out=[]
    for x in (o,nw):
        t=L(f'out/{x}.png')/np.maximum(R,1e-4)
        tv=t[ok]; m=tv.mean(0)
        dark=1-tv[:,2]; md=1-m[2]
        out.append((m, tv[:,2].std(), (dark>2*md+0.02).mean()*100))
    f=lambda m:'%.3f, %.3f, %.3f'%tuple(m)
    print(f'| {n} | {f(out[0][0])} / {f(out[1][0])} | {out[0][1]:.3f} / {out[1][1]:.3f} | {out[0][2]:.2f}% / {out[1][2]:.2f}% |')
