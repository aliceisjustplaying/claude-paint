import sys, re
# write the head of a log (everything before the first chunk) plus its first k chunks
src, k, out = sys.argv[1], int(sys.argv[2]), sys.argv[3]
text = open(src).read()
parts = re.split(r'(?m)^(?=--@ chunk )', text)
head, chunks = parts[0], parts[1:]
open(out, 'w').write(head + ''.join(chunks[:k]).rstrip('\n') + '\n')
