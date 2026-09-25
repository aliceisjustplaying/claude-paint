import json
import numpy as np
from PIL import Image
from scipy import ndimage
S = 3.2
bg = np.array([0xd9, 0xd2, 0xc2], float)
ink = np.array([0x2a, 0x23, 0x1d], float)
res = {}
for v in ['main', 'fix']:
    a = np.asarray(Image.open(f'<scratch>/out/tree-{v}/tree.png').convert('RGB')).astype(float)
    dark = ((bg - a) / (bg - ink)).mean(axis=2)
    m = dark > 0.15
    lab, n = ndimage.label(m, structure=np.ones((3, 3)))
    sizes = ndimage.sum(m, lab, range(1, n + 1))
    seeds = {lab[int(450 * S), int(500 * S)], lab[int(345 * S), int(500 * S)]} - {0}
    off = [s for i, s in enumerate(sizes) if i + 1 not in seeds and s > 6]
    js = json.load(open(f'<scratch>/out/tree-{v}/junctions.json'))
    joined = 0
    for x, y, d in js:
        r = int(1.0 * S)
        win = lab[int(y * S) - r:int(y * S) + r + 1, int(x * S) - r:int(x * S) + r + 1]
        joined += int(any(s in win for s in seeds))
    res[v] = dict(tree_components=len(seeds), pieces_off_tree=len(off), px_off_tree=int(sum(off)), largest_off=int(max(off, default=0)), ends_joined=f'{joined}/{len(js)}')
    print(v, res[v])
json.dump(res, open('<scratch>/out/tree_measure.json', 'w'), indent=1)
