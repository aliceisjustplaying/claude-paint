from copied_shapes import copied_shapes

LOG = """--@ chunk 1
r = sky:at(x, 2*H - y)             -- a reflection read off the sky mask
--@ chunk 2
t = m:at(y, x)
u = m:at(x + 40, y)                -- moved: allowed
v = m:at(x, y)
-- w = m:at(x, 804 - y)  a comment
s:stroke({{100, 200}, {300, 900 - y}})
"""


def test_mirrored_and_rotated_reads_are_flagged_and_moves_and_comments_are_not():
    assert [(f["chunk"], f["kind"]) for f in copied_shapes(LOG)] == [(1, "mirrored"), (2, "rotated"), (2, "mirrored point")]
