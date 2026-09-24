import numpy as np, sys
from PIL import Image
d,png,out=sys.argv[1:4]
r=np.load(d+'/ratio.npy')
im=np.asarray(Image.open(png).convert('RGB')).astype(float)
H,W=r.shape
im=im[:H,:W].copy()
g=im.mean(2,keepdims=True)*0.5+60
v=np.repeat(g,3,2)
v[r>3]=[255,0,0]
v[r<0.1]=[0,120,255]
Image.fromarray(v.clip(0,255).astype(np.uint8)).save(out)
