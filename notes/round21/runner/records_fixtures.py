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


# The logs those ids come from, cut down to the calls cited and a few around them (the tool
# results' texts are the real ones, the studio's path shortened). r17 F's painter ran chunks with
# bash and looked in the same command; r16 D's wrote a chunk file, ran it with bash and read the
# look's .png; r19's painters use the paint and look tools.
def _call(cid, name, args):
    return {"type": "message", "message": {"role": "assistant", "content": [
        {"type": "thinking", "thinking": "..."}, {"type": "toolCall", "id": cid, "name": name, "arguments": args}]}}


def _result(cid, name, text, is_error=False, image=False):
    content = [{"type": "text", "text": text}] + ([{"type": "image", "data": "iVBORw0K"}] if image else [])
    return {"type": "message", "message": {"role": "toolResult", "toolCallId": cid, "toolName": name,
                                           "content": content, "isError": is_error}}


LOOK = "~/src/a/paint-studio-x/out/easel/painting/look-{}.png (1000x714, 0.07s)\n"
R17F_LOGS = [
    [_call("toolu_01Fi7k928j58wVXh8f8j1XYK", "bash", {"command": "cat > c100.lua <<'EOF'\nwork(band, {hand=\"hatch\", "
                                                                 "coverage=function(x,y) return 0.6 end})\nEOF\n"
                                                                 "bin/easel do -f c100.lua; bin/easel look"}),
     _result("toolu_01Fi7k928j58wVXh8f8j1XYK", "bash",
             "error converting Lua function to f32 (expected number or string coercible to number)\n"
             "(the chunk failed and changed nothing)\n" + LOOK.format("0099")),
     _call("toolu_01UubR4dykeR1KExnqQN1haA", "bash", {"command": "bin/easel note \"Sky trouble.\"\n"
                                                                 "bin/easel do -f c129.lua; bin/easel look"}),
     _result("toolu_01UubR4dykeR1KExnqQN1haA", "bash",
             "noted in ~/src/a/paint-studio-x/notes/journal.md\nok · chunk 129 (33.08 s to compute)\n"
             + LOOK.format("0135")),
     _call("toolu_01VZ2LgtmdiXAN98rnrHZ1aE", "bash", {"command": "bin/easel note \"The sky repaint went badly wrong. "
                                                                 "A 40-wide badger without clip ran across the masks.\""}),
     _result("toolu_01VZ2LgtmdiXAN98rnrHZ1aE", "bash",
             "noted in ~/src/a/paint-studio-x/notes/journal.md\nok · chunk 139 (8.54 s to compute)\n"
             + LOOK.format("0146"))],
    [_call("toolu_01Y9c22xk1JRbFerzrrjzSRT", "bash", {"command": "cat > r6.lua <<'EOF'\nprint(wait(60*30))\nEOF\n"
                                                                 "bin/easel do -f r6.lua"}),
     _result("toolu_01Y9c22xk1JRbFerzrrjzSRT", "bash",
             "day 93, 21:43\ntacky\ttacky\nok · chunk 162 (0.37 s to compute)\n" + LOOK.format("0179"))],
]
R16D_LOGS = [
    [_call("call_452249", "write", {"path": "c3.lua", "content": "p_bg_dark = pile{...}\nwork(m_bg, {hand=\"body\"})\n"}),
     _result("call_452249", "write", "Successfully wrote to c3.lua"),
     _call("call_998948", "bash", {"command": "timeout 120 bin/easel do -f c3.lua --look", "timeout": 130}),
     _result("call_998948", "bash", "ok · chunk 3 (27.24 s to compute)\n" + LOOK.format("0003")),
     _call("call_4155479", "read", {"limit": 10, "offset": 1, "path": "out/easel/painting/look-0003.png"}),
     _result("call_4155479", "read", "Read image file [image/png]", image=True),
     _call("call_7595596", "write", {"path": "check_dry.lua", "content": "print(\"table drying:\", drying(500, 570))\n"}),
     _result("call_7595596", "write", "Successfully wrote to check_dry.lua"),
     _call("call_214951", "bash", {"command": "timeout 20 bin/easel do -f check_dry.lua", "timeout": 25}),
     _result("call_214951", "bash", "canvas time:\tday 1, 11:04\ntable drying:\topen\nbg drying:\tsetting\n"
                                    "ok · chunk 5 (0.02 s to compute)\n")],
]


def write_logs(d, logs):
    """Write each log's entries as a pi session file in d (after a session line); their paths."""
    import json
    paths = []
    for i, entries in enumerate(logs, 1):
        p = d / f"log{i}.jsonl"
        p.write_text("".join(json.dumps(e) + "\n" for e in [{"type": "session", "version": 3}] + entries))
        paths.append(p)
    return paths
