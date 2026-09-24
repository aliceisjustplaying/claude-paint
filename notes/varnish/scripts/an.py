import numpy as np, sys
d=sys.argv[1]
w,h,pxmm=open(d+'/dims.txt').read().split(); w=int(w);h=int(h);pxmm=float(pxmm)
L=lambda n: np.fromfile(f'{d}/{n}.f32',dtype='<f4').reshape(h,w)
old,add,film=L('old'),L('add'),L('film')
m=128
sl=(slice(m,h-m),slice(m,w-m))
o,a,f=old[sl],add[sl],film[sl]
print('px mm',pxmm,'add mean %.3f'%a.mean())
r=f/np.maximum(a,1e-9)
print('film/add quantiles', np.percentile(r,[0,1,5,10,25,50,75,90,95,99,99.9,100]).round(2))
print('bare (film<0.1 add): %.2f%%'%(100*(r<0.1).mean()), ' deep (>3x): %.2f%%'%(100*(r>3).mean()), '>6x %.2f%%'%(100*(r>6).mean()))
# relation to local height: old minus local mean (2mm box)
from scipy.ndimage import uniform_filter
k=int(round(2/pxmm))
loc=o-uniform_filter(o,k)
for lo,hi in [(-99,-10),(-10,-3),(-3,-1),(-1,1),(1,3),(3,10),(10,99)]:
    s=(loc>=lo)&(loc<hi)
    if s.sum(): print(f'rel height {lo:>4}..{hi:<3} n={s.mean()*100:5.1f}%  film/add mean {r[s].mean():.2f} bare {(r[s]<0.1).mean()*100:5.1f}%  >3x {(r[s]>3).mean()*100:5.1f}%')
# slope of old
gy,gx=np.gradient(o); g=np.hypot(gx,gy)/(pxmm*1000)
for lo,hi in [(0,0.02),(0.02,0.05),(0.05,0.1),(0.1,0.2),(0.2,9)]:
    s=(g>=lo)&(g<hi)
    if s.sum(): print(f'slope {lo}..{hi} n={s.mean()*100:5.1f}% film/add mean {r[s].mean():.2f}  >3x {(r[s]>3).mean()*100:5.1f}%  bare {(r[s]<0.1).mean()*100:5.1f}%')
# film height change: new surface pits? new = old+film; compare with old local max
new=o+f
print('old std %.2f um, local rel range'%o.std(), np.percentile(loc,[1,50,99]).round(1))
np.save(d+'/ratio.npy',r)
