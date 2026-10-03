"""Tests of defaults_used.py: which ready-made handlings a log calls on."""
from defaults_used import defaults_used

LOG = """--@ chunk 1
canvas{size=300}
--@ chunk 2
work(m, {hand="body", pile=p})
work(m, {pile=p})                       -- the default hand
work(m, {hand='detail', tool='flat 5', pile=p})
work(m, {hand=h, pile=p})
-- work(m, {hand="glaze"})  a comment isn't a call
print("work(m, {hand='broad'})")
blend(m, {angle=0})
o = outline{{1, 2}, {3, 4}, char="searching"}
o2 = outline{{1, 2}, {3, 4}}
b = body_of{spine={{1, 2}}, widths={3}}
--[[ outline{{0,0}} ]]
b2 = brush("filbert", 8)
x = m:work(1)
"""


def test_call_sites_by_hand_and_character_with_comments_and_strings_left_out():
    d = defaults_used(LOG)
    assert d["work"] == {"body": 1, "body (default)": 1, "detail": 1, "set from a variable": 1}
    assert d["work_with_own_tool"] == 1
    assert d["blend"] == 1
    assert d["outline"] == {"searching": 1, "firm (default)": 1}
    assert d["body_of"] == {"soft (default)": 1}
    assert d["brush"] == 1
