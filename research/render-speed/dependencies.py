import argparse
import heapq
import json
from pathlib import Path


def overlap(a, b):
    return a[0] < b[2] and b[0] < a[2] and a[1] < b[3] and b[1] < a[3]


def model(weights, predecessors):
    successors = [[] for _ in weights]
    finish = []
    for i, before in enumerate(predecessors):
        finish.append(weights[i] + max((finish[j] for j in before), default=0))
        for j in before:
            successors[j].append(i)
    tail = weights[:]
    for i in reversed(range(len(weights))):
        tail[i] += max((tail[j] for j in successors[i]), default=0)
    result = {"work": sum(weights), "span": max(finish, default=0), "nodes": len(weights), "edges": sum(map(len, predecessors))}
    for workers in [1, 2, 4, 8, 16, 32]:
        remaining = list(map(len, predecessors))
        ready = [(-tail[i], i) for i, n in enumerate(remaining) if n == 0]
        heapq.heapify(ready)
        running = []
        now = 0
        while ready or running:
            while ready and len(running) < workers:
                _, i = heapq.heappop(ready)
                heapq.heappush(running, (now + weights[i], i))
            now, i = heapq.heappop(running)
            for j in successors[i]:
                remaining[j] -= 1
                if remaining[j] == 0:
                    heapq.heappush(ready, (-tail[j], j))
        result[str(workers)] = now
    return result


def analyze(batch):
    tiles = {i: [] for i in batch["order"]}
    for tile, index, x0, y0, x1, y1, seconds in batch["strokes"]:
        tiles[int(tile)].append((int(index), (int(x0), int(y0), int(x1), int(y1)), seconds))
    tiles = [sorted(tiles[i]) for i in batch["order"] if tiles[i]]
    bounds = [(min(s[1][0] for s in tile), min(s[1][1] for s in tile), max(s[1][2] for s in tile), max(s[1][3] for s in tile)) for tile in tiles]
    coarse = model([sum(s[2] for s in tile) for tile in tiles], [[j for j in range(i) if overlap(bounds[i], bounds[j])] for i in range(len(tiles))])
    precise_tiles = model([sum(s[2] for s in tile) for tile in tiles], [[j for j in range(i) if overlap(bounds[i], bounds[j]) and any(overlap(a[1], b[1]) for a in tiles[i] for b in tiles[j])] for i in range(len(tiles))])
    weights, predecessors, previous = [], [], []
    for tile in tiles:
        added = []
        for _, rect, seconds in tile:
            i = len(weights)
            before = [j for j, r in previous if overlap(rect, r)]
            if added:
                before.append(i - 1)
            weights.append(seconds)
            predecessors.append(before)
            added.append((i, rect))
        previous.extend(added)
    grouped_weights, grouped_predecessors, previous_groups = [], [], []
    for tile in tiles:
        added = []
        for offset in range(0, len(tile), 4):
            strokes = tile[offset:offset + 4]
            rects = [s[1] for s in strokes]
            i = len(grouped_weights)
            before = [j for j, old in previous_groups if any(overlap(a, b) for a in rects for b in old)]
            if added:
                before.append(i - 1)
            grouped_weights.append(sum(s[2] for s in strokes))
            grouped_predecessors.append(before)
            added.append((i, rects))
        previous_groups.extend(added)
    return {"coarse": coarse, "precise_tiles": precise_tiles, "groups4": model(grouped_weights, grouped_predecessors), "fine": model(weights, predecessors)}


parser = argparse.ArgumentParser()
parser.add_argument("trace", type=Path)
parser.add_argument("output", type=Path)
args = parser.parse_args()
batches = []
for line in args.trace.read_text().splitlines():
    if line.startswith("PROFILE_BATCH,"):
        batches.append({"order": [int(x) for x in line.split(",")[1:] if x], "strokes": []})
    elif line.startswith("PROFILE_STROKE,"):
        batches[-1]["strokes"].append([float(x) for x in line.split(",")[1:]])
rows = [analyze(b) for b in batches if b["strokes"]]
result = {"interpretation": "Trace-weighted idealized scheduling, not an implemented speedup. Preserves tile-held brush chains and all cross-tile overlapping-stroke order. Excludes scheduler overhead and non-drag work.", "batches": len(rows)}
for mode in ["coarse", "precise_tiles", "groups4", "fine"]:
    result[mode] = {k: sum(row[mode][k] for row in rows) for k in rows[0][mode]}
result["rows"] = rows
args.output.write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({k: v for k, v in result.items() if k != "rows"}, indent=2))
