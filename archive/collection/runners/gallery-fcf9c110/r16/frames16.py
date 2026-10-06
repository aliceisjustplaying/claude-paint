# /// script
# dependencies = ["pillow"]
# ///
# Round 16 frames, the same way as ~/tmp/catchup-r15-04ce1082/clip/build.py: every whole look
# (the session's modal image size), consecutive duplicates dropped, the final 1000 px image appended,
# 1280 px wide JPEGs. Plus a numbered contact sheet per painting for review.
import json,base64,io,glob,os,collections,shutil,hashlib,sys
from PIL import Image, ImageDraw
S=os.path.expanduser("~/.pi/agent/sessions"); T=os.path.dirname(os.path.abspath(__file__))
LOOK=os.path.expanduser("~/src/a/claude-paint/notes/round16/look")
ST=json.load(open(os.path.expanduser("~/tmp/gallery-fcf9c110/r16/run/studios.json")))
EXCL=json.loads(os.environ.get("EXCL","{}"))
keys=sys.argv[1:] or ["A1","A2","A3","B1","B2","B3","C1","C2","C3","D2","D3"]
for k in keys:
    f,=glob.glob(f"{S}/--Users-alice-src-a-{ST[k]}--/*.jsonl")
    imgs=[]
    for line in open(f):
        try:e=json.loads(line)
        except:continue
        c=(e.get('message') or {}).get('content')
        if not isinstance(c,list): continue
        for x in c:
            if isinstance(x,dict) and x.get('type')=='image':
                try: im=Image.open(io.BytesIO(base64.b64decode(x['data']))); im.load()
                except: continue
                imgs.append(im)
    wide=[im.size for im in imgs if im.size[0]>=900]
    mode=collections.Counter(wide).most_common(1)[0][0]
    sel=[];last=None
    for im in imgs:
        if im.size!=mode: continue
        h=hashlib.md5(im.convert('RGB').tobytes()).hexdigest()
        if h==last: continue
        last=h; sel.append(im)
    ex=set(EXCL.get(k,[])); sel=[im for i,im in enumerate(sel) if i+1 not in ex]
    fin=Image.open(f"{LOOK}/{k}.png"); sel.append(fin)
    W=1280; H=round(W*mode[1]/mode[0]/2)*2
    name=f"r16-{k.lower()}"; d=f"{T}/frames/{name}"; shutil.rmtree(d,ignore_errors=True); os.makedirs(d)
    for i,im in enumerate(sel): im.convert('RGB').resize((W,H),Image.LANCZOS).save(f"{d}/{i+1:03d}.jpg",quality=93)
    TW=150; cols=8; rows=(len(sel)+cols-1)//cols; th=round(TW*H/W)
    sh=Image.new('RGB',(cols*(TW+4),rows*(th+4)),'black'); dr=ImageDraw.Draw(sh)
    for i,im in enumerate(sel):
        t=im.convert('RGB').resize((TW,th)); x,y=(i%cols)*(TW+4),(i//cols)*(th+4); sh.paste(t,(x,y)); dr.text((x+3,y+3),str(i+1),fill='yellow')
    sh.save(f"{T}/frames/{name}_sheet.jpg",quality=88)
    print(k, "mode",mode,"final",fin.size,"frames",len(sel))
