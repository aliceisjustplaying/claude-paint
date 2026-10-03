# Technique sampler

Nine plates that show each material and technique of engine 3, in the
giverny box (`EASEL_BOX=giverny`). Each plate is `common.lua` (the canvas)
then its own chunk; plate 7 (grounds) is run twice, on
`canvas{..., ground={{..., apply="roller"}}}` and with `absorbent=true`;
plate 9 (a small study) is its three files in order, with its own canvas.

    EASEL_BOX=giverny easel open demo-p1
    easel -s demo-p1 do -f common.lua
    easel -s demo-p1 do -f p1_loading.lua
    easel -s demo-p1 save p1.png
    easel -s demo-p1 look --mode relief      # plates 3, 4, 5, 9

See notes/techniques.md for what each technique is for.
