import argparse
import json
from pathlib import Path
import re
import shutil


def replace_once(text, old, new):
    if text.count(old) != 1:
        raise ValueError(f"trace insertion no longer unique: {old[:100]}")
    return text.replace(old, new)


def instrument(source, target, footprint=None):
    target.mkdir(parents=True, exist_ok=False)
    shutil.copytree(source / "crates", target / "crates")
    for name in ["Cargo.toml", "Cargo.lock", ".cargo"]:
        src = source / name
        if src.is_dir():
            shutil.copytree(src, target / name)
        elif src.exists():
            shutil.copy2(src, target / name)
    src = target / "crates/easel/src"
    shutil.copy2(Path(__file__).with_name("trace_runtime.rs"), src / "engine7_trace.rs")
    main = src / "main.rs"
    main.write_text(replace_once(main.read_text(), "mod api;", "mod api;\nmod engine7_trace;"))
    api = src / "api.rs"
    text = api.read_text()
    start = text.index("impl UserData for Brush {")
    end = text.index("\n}", start) + 2
    brush = text[start:end]
    pattern = r'(m\.add_method\("([a-z_]+)", \|[^\n]+?\|) ([^\n]+)'
    count = 0

    def method(match):
        nonlocal count
        head, name, body = match.groups()
        count += 1
        scope = f'let _event = crate::engine7_trace::scope(&b.st, "brush:{name}", b.trace_id);'
        if body.startswith("{"):
            return head + " { " + scope + body[1:]
        return head + " { " + scope + " " + body[:-2] + " });"

    brush = re.sub(pattern, method, brush)
    if count < 9:
        raise ValueError(f"only {count} brush methods found")
    text = text[:start] + brush + text[end:]
    text = replace_once(text, "pub struct Brush {", "pub struct Brush {\n    trace_id: usize,")
    text = replace_once(text, "Ok(Brush { held, st: st1.clone() })", "Ok(Brush { held, st: st1.clone(), trace_id: seed as usize })")
    text = replace_once(text, 'let tostring: Function = lua.globals().get("tostring")?;',
                        'let _event = crate::engine7_trace::scope(&st, "print", 0);\n                let tostring: Function = lua.globals().get("tostring")?;')
    text = replace_once(text, 'fn work(st: &S, mask: Rc<Mask>, o: Table, preset: Option<&str>) -> Result<()> {',
                        'fn work(st: &S, mask: Rc<Mask>, o: Table, preset: Option<&str>) -> Result<()> {\n    let _event = crate::engine7_trace::scope(st, "work", 0);')
    api.write_text(text)
    time = src / "time.rs"
    text = time.read_text()
    old = 'pub fn verb<R>(st: &S, kind: Verb, f: impl FnOnce(&mut Studio) -> mlua::Result<R>) -> mlua::Result<R> {'
    text = replace_once(text, old, old + '\n    let _event = crate::engine7_trace::scope(st, "verb", 0);\n    crate::engine7_trace::kind(match &kind { Verb::Marks => "marks", Verb::Pass => "pass", Verb::Query => "query", Verb::Wait => "wait" });')
    for name in ["trip", "knife"]:
        old = f'pub fn {name}(st: &S, color: paint::Rgb) {{'
        text = replace_once(text, old, old + f'\n    let _event = crate::engine7_trace::scope(st, "palette:{name}", 0);\n    crate::engine7_trace::palette();')
    time.write_text(text)
    if footprint is not None:
        shutil.copy2(footprint, target / "crates/paint/src/engine7/footprint.rs")
        bristle = target / "crates/paint/src/bristle.rs"
        text = bristle.read_text()
        old = "    g.assert_valid();\n    let mut bounds: Bounds = None;"
        new = '''    g.assert_valid();
    let trace_start = std::time::Instant::now();
    let trace_conservative = crate::engine7::footprint::Footprint::conservative(&held.tool, g, sf.scale, sf.fw, sf.fh);
    let trace_exact = crate::engine7::footprint::Footprint::exact(held, g, id, sf.scale, sf.fw, sf.fh);
    eprintln!("E7FOOT\\t{}\\t{:?}\\t{:?}\\t{}\\t{}", id, trace_conservative, trace_exact, held.tool.point > 0.0, trace_start.elapsed().as_nanos());
    let mut bounds: Bounds = None;'''
        bristle.write_text(replace_once(text, old, new))
    print(json.dumps({"scratch": str(target), "brush_methods": count}))


def analyze(path, output):
    events = []
    footprints = []
    for line in path.read_text().splitlines():
        if line.startswith("E7BEGIN\t"):
            footprints = []
        if line.startswith("E7FOOT\t"):
            _, stroke, conservative, exact, pointed, nanos = line.split("\t")
            rectangle = lambda text: list(map(int, re.findall(r"\d+", text))) or None
            footprints.append({"stroke": int(stroke), "conservative": rectangle(conservative),
                               "exact": rectangle(exact), "pointed_fallback": pointed == "true",
                               "dry_pass_ns": int(nanos)})
        if not line.startswith("E7TRACE\t"):
            continue
        _, chunk, name, brush, kind, palette, t0, t1, o0, o1, secs = line.split("\t")
        events.append({"id": len(events), "chunk": int(chunk), "name": name,
                       "brush": int(brush), "kind": kind, "palette": palette == "true",
                       "clock0_min": float(t0), "clock1_min": float(t1),
                       "owed0_s": float(o0), "owed1_s": float(o1), "wall_s": float(secs),
                       "footprints": footprints})
        footprints = []
    if not events:
        raise ValueError("no runtime trace events")
    edges = []
    last_brush = {}
    last_palette = None
    barrier = None
    pending = []
    chunk = None
    for event in events:
        i = event["id"]
        if event["chunk"] != chunk:
            edges.extend((p, i, "chunk-observation") for p in pending)
            pending = []
            barrier = i
            chunk = event["chunk"]
        elif barrier is not None and barrier != i:
            edges.append((barrier, i, "barrier"))
        if event["brush"]:
            b = event["brush"]
            if b in last_brush:
                edges.append((last_brush[b], i, "brush"))
            last_brush[b] = i
        if event["palette"]:
            if last_palette is not None:
                edges.append((last_palette, i, "palette"))
            last_palette = i
        for p in pending:
            if any(a["exact"] and b["exact"] and
                   a["exact"][0] < b["exact"][2] and b["exact"][0] < a["exact"][2] and
                   a["exact"][1] < b["exact"][3] and b["exact"][1] < a["exact"][3]
                   for a in events[p]["footprints"] for b in event["footprints"]):
                edges.append((p, i, "material-exact"))
        global_time = event["clock0_min"] != event["clock1_min"]
        observe = event["kind"] in ["query", "wait"] or event["name"] == "print"
        if global_time or observe:
            reason = "time" if global_time else "observation"
            edges.extend((p, i, reason) for p in pending)
            pending = []
            barrier = i
        pending.append(i)
    successors = [[] for _ in events]
    indegree = [0] * len(events)
    for a, b in set((a, b) for a, b, _ in edges):
        if a >= b:
            raise ValueError("non-forward dependency")
        successors[a].append(b)
        indegree[b] += 1
    ready = [i for i, d in enumerate(indegree) if d == 0]
    levels = []
    span = [e["wall_s"] for e in events]
    while ready:
        levels.append(ready)
        following = []
        for i in ready:
            for j in successors[i]:
                span[j] = max(span[j], span[i] + events[j]["wall_s"])
                indegree[j] -= 1
                if indegree[j] == 0:
                    following.append(j)
        ready = following
    total = sum(e["wall_s"] for e in events)
    reach = [0] * len(events)
    for i in reversed(range(len(events))):
        for j in successors[i]:
            reach[i] |= (1 << j) | reach[j]
    matched = {}

    def augment(i, visited):
        bits = reach[i]
        while bits:
            bit = bits & -bits
            bits ^= bit
            j = bit.bit_length() - 1
            if j in visited:
                continue
            visited.add(j)
            if j not in matched or augment(matched[j], visited):
                matched[j] = i
                return True
        return False

    for i in range(len(events)):
        augment(i, set())
    width = len(events) - len(matched)
    all_footprints = [f for event in events for f in event["footprints"]]
    area = lambda rect: (rect[2] - rect[0]) * (rect[3] - rect[1]) if rect else 0
    conservative_area = sum(area(f["conservative"]) for f in all_footprints)
    exact_area = sum(area(f["exact"]) for f in all_footprints)
    result = {"boundary": "whole synchronous verb graph with current global time joins; known exact spatial edges included, unsupported spatial edges omitted, so width is an optimistic upper bound",
              "exact_footprints": bool(all_footprints), "events": events, "edges": edges,
              "ready_width_upper_bound": width,
              "asap_max_ready": max(map(len, levels)),
              "mean_level_width": len(events) / len(levels),
              "traced_wall_s": total, "weighted_span_s": max(span),
              "ideal_parallelism_upper_bound": total / max(span),
              "footprint_summary": {"gestures": len(all_footprints), "conservative_area_sum": conservative_area,
                                    "exact_area_sum": exact_area,
                                    "area_ratio": conservative_area / exact_area if exact_area else None,
                                    "zero_exact": sum(f["exact"] is None for f in all_footprints),
                                    "pointed_fallback": sum(f["pointed_fallback"] for f in all_footprints),
                                    "dry_pass_ns": sum(f["dry_pass_ns"] for f in all_footprints)},
              "edge_counts": {k: sum(e[2] == k for e in edges) for k in sorted(set(e[2] for e in edges))}}
    output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({k: v for k, v in result.items() if k not in ["events", "edges"]}))


def main():
    parser = argparse.ArgumentParser(description="Disposable runtime trace; never rewrites the real checkout")
    sub = parser.add_subparsers(dest="action", required=True)
    p = sub.add_parser("instrument")
    p.add_argument("source", type=Path)
    p.add_argument("target", type=Path)
    p.add_argument("--footprint", type=Path, help="Footprint lane file copied into disposable source only")
    p = sub.add_parser("analyze")
    p.add_argument("stderr", type=Path)
    p.add_argument("output", type=Path)
    args = parser.parse_args()
    if args.action == "instrument":
        instrument(args.source.resolve(), args.target.resolve(), args.footprint)
    else:
        analyze(args.stderr, args.output)


if __name__ == "__main__":
    main()
