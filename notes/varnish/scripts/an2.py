import numpy as np, sys
from scipy.ndimage import uniform_filter
d=sys.argv[1]
w,h,pxmm=open(d+'/dims.txt').read().split(); w=int(w);h=int(h);pxmm=float(pxmm)
L=lambda n: np.fromfile(f'{d}/{n}.f32',dtype='<f4').reshape(h,w)
old,add,film=L('old'),L('add'),L('film')
r1=max(round(0.15/pxmm),1); r2=max(round(0.9/pxmm),r1+1)
bb=lambda a,r: uniform_filter(uniform_filter(a,2*r+1,mode='nearest'),2*r+1,mode='nearest')
s=old+add; l1=bb(s,r1); l2=bb(l1,r2)
d1=s-l1; d2=l1-l2
m=128; sl=(slice(m,h-m),slice(m,w-m))
r=(film/np.maximum(add,1e-9))[sl]; D1=d1[sl]; D2=d2[sl]
print('r1 r2',r1,r2)
print('|d1| pct', np.percentile(abs(D1),[50,90,99,99.9]).round(1), ' add', add[sl].mean().round(2))
deep=r>3; bare=r<0.1; ok=(r>0.8)&(r<1.2)
for n,s_ in [('deep',deep),('bare',bare),('normal',ok)]:
    print(f'{n:7s} d1 median {np.median(D1[s_]):7.1f} um  |d1| median {np.median(abs(D1[s_])):6.1f}  d2 median {np.median(D2[s_]):7.1f}')
# film fraction of volume that sits in deep pixels
f=film[sl]; print('share of varnish volume in the 0.9%% deep pixels: %.1f%%'%(100*f[deep].sum()/f.sum()))
# fillet estimate: step heights at deep pixels
print('deep pixels film um: median %.1f max %.1f'%(np.median(f[deep]),f.max()))
