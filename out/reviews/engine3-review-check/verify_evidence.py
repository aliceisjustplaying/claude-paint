import json,hashlib,re,sys
from pathlib import Path
root=Path('engine3-rag-review'); rev=Path('engine3-independent-review')
man=json.loads((rev/'source_manifest.json').read_text())
for m in man:
    p=root/m['file']
    h=hashlib.sha256(p.read_bytes()).hexdigest()
    print(m['id'], 'HASH OK' if h==m['sha256'] else 'HASH MISMATCH', m['file'])
# verify excerpts
ev=(rev/'EVIDENCE.md').read_text().split('\n')
cur=None; bad=0; total=0; src=None
for line in ev:
    m=re.match(r'Source: `(.+?)`',line)
    if m:
        cur=m.group(1); src=(root/cur).read_text().split('\n'); continue
    m=re.match(r'\s*(\d+): ?(.*)$',line)
    if m and src is not None:
        n=int(m.group(1)); txt=m.group(2)
        total+=1
        if n-1>=len(src) or src[n-1].rstrip()!=txt.rstrip():
            bad+=1
            if bad<15: print('MISMATCH',cur,n,repr(txt[:80]),'|',repr(src[n-1][:80] if n-1<len(src) else None))
print('excerpt lines',total,'mismatches',bad)
