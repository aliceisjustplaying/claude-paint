import numpy as np
from PIL import Image, ImageDraw, ImageFont
N='out/'
tiles=[('rock_H lit base','rock_orig','rock_fix',(300,440)),
       ('oakleaf_H crown','oak_old','oak_new',(560,330)),
       ('l5_near boulder','l5c_old','l5c_new',(150,220)),
       ('l3_green rock','l3c_old','l3c_new',(150,70))]
S=240; G=6; top=26; lab=22
W=len(tiles)*S+(len(tiles)+1)*G
H=top+2*(S+lab)+3*G
sheet=Image.new('RGB',(W,H),(30,30,30)); d=ImageDraw.Draw(sheet)
f=ImageFont.load_default(size=15); fb=ImageFont.load_default(size=17)
d.text((G,4),'varnish{coats=0.3, vary=0.1}; relief() at 3200px, 1:1 pixels. Top: before (old settle). Bottom: after (settle_film).',font=f,fill=(235,235,235))
for i,(name,o,n,(x,y)) in enumerate(tiles):
    X=G+i*(S+G)
    for j,(k,tag) in enumerate([(o,'before'),(n,'after')]):
        Y=top+G+j*(S+lab+G)
        d.text((X+2,Y),f'{tag}: {name}',font=f,fill=(255,210,150) if j==0 else (170,230,170))
        im=Image.open(N+k+'.png').convert('RGB').crop((x,y,x+S,y+S))
        sheet.paste(im,(X,Y+lab))
sheet.save('owner_sheet.png')
print(sheet.size)
