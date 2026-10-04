# Technique sampler

Nine plates that show each material and technique of engine 4, in the
giverny box (`EASEL_BOX=giverny`). Each plate is `common.lua` (the canvas)
then its own chunk (run from this folder, or give the files' paths:
`-f paintings/sampler/common.lua`); plate 7 (grounds) is run twice, on
`canvas{..., ground={{..., apply="roller"}}}` and with `absorbent=true`;
plate 9 (a small study) is its three files in order, with its own canvas.

    EASEL_BOX=giverny easel open demo-p1
    easel -s demo-p1 do -f common.lua
    easel -s demo-p1 do -f p1_loading.lua
    easel -s demo-p1 save p1.png
    easel -s demo-p1 look --mode relief      # plates 3, 4, 5, 9

See notes/techniques.md for what each technique is for.

Plate 10 is plate 9 saved under six views:

    EASEL_BOX=giverny easel open demo-p9
    easel -s demo-p9 do -f p9_study.lua
    easel -s demo-p9 do -f p9_study2.lua
    easel -s demo-p9 do -f p9_study3.lua
    easel -s demo-p9 save flat.png
    easel -s demo-p9 save gallery.png --gallery
    easel -s demo-p9 save 40.png --light 125,40
    easel -s demo-p9 save 25.png --light 135,25
    easel -s demo-p9 save 10.png --light 135,10
    easel -s demo-p9 save right.png --light 20,30
