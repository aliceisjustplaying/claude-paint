"""Test fixtures for structured chain records: the design's worked example (section 7), two real
records converted by hand (r17 F p1 and r16 D p1), with the tool-call ids of their logs."""
import copy

R17F = [
    {"category": "blending", "operation": "blend",
     "conditions": {"surface": "open", "clip": False, "brush": {"kind": "badger", "width": [30, 40]},
                    "params": {"coverage": [1.2, 2.0], "pressure": [0.2, 0.5]},
                    "tubes": [], "extent_units": None, "note": None},
     "effect": "The badger dragged wet paint across the edges of its mask and broke shapes beside it into "
               "separate lumps.",
     "cause": None, "basis": "painter_reported",
     "evidence": [{"log": 1, "call": "toolu_01UubR4dykeR1KExnqQN1haA", "role": "operation"},
                  {"log": 1, "call": "toolu_01VZ2LgtmdiXAN98rnrHZ1aE", "role": "report"}]},
    {"category": "easel_errors", "operation": "work",
     "conditions": {"params": {"hand": "hatch"}, "note": "coverage given as a function"},
     "effect": "work refused a function as its coverage and the chunk changed nothing.",
     "basis": "printed",
     "evidence": [{"log": 1, "call": "toolu_01Fi7k928j58wVXh8f8j1XYK", "role": "operation"}]},
    {"category": "drying_and_time", "operation": "wait",
     "conditions": {"params": {"wait_minutes": 1800}},
     "effect": "The wait counted in minutes, so 1800 of them moved the canvas clock on by 30 hours.",
     "basis": "printed",
     "evidence": [{"log": 2, "call": "toolu_01Y9c22xk1JRbFerzrrjzSRT", "role": "operation"}]},
]

R16D = [
    {"category": "mixing_piles", "operation": "work",
     "conditions": {"brush": {"kind": "filbert", "width": 20},
                    "params": {"hand": "body", "medium": 0.2, "fill": True},
                    "tubes": ["raw umber", "bone black", "red earth", "lead white"]},
     "effect": "A dark pile holding a little lead white came out on the canvas as a pale mauve gray, far "
               "lighter than its masstone.",
     "basis": "seen",
     "evidence": [{"log": 1, "call": "call_452249", "role": "source"},
                  {"log": 1, "call": "call_998948", "role": "operation"},
                  {"log": 1, "call": "call_4155479", "role": "image"}]},
    {"category": "strokes", "operation": "work",
     "conditions": {"brush": {"kind": "filbert", "width": 20},
                    "params": {"hand": "body", "fill": True, "coverage": 2.0}, "extent_units": 100},
     "effect": "The passage came out as a patchwork of separate rectangular dabs in two tones, not a smooth "
               "plane.",
     "basis": "seen",
     "evidence": [{"log": 1, "call": "call_452249", "role": "source"},
                  {"log": 1, "call": "call_998948", "role": "operation"},
                  {"log": 1, "call": "call_4155479", "role": "image"}]},
    {"category": "drying_and_time", "operation": "drying",
     "conditions": {"params": {"medium": [0.2, 0.25]}, "note": "the later layer at medium 0.15"},
     "effect": "Queried at the same moment, the earth-and-black layer read setting while a later "
               "lead-white-rich layer read open.",
     "basis": "printed",
     "evidence": [{"log": 1, "call": "call_7595596", "role": "source"},
                  {"log": 1, "call": "call_214951", "role": "operation"}]},
]


def doc(observations):
    return {"schema": "chain-observations/1", "observations": copy.deepcopy(observations)}


# the default box's tubes (notes/easel_guide.md's tube table)
TUBES = ["lead white", "smalt", "pale smalt", "yellow ochre", "red earth", "vermilion", "raw umber", "bone black",
         "cobalt blue", "chrome yellow", "Prussian blue", "green earth", "Rinmann's green", "copper green"]
