import argparse
import json
import math
import pathlib
import subprocess


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("runner", type=pathlib.Path)
    parser.add_argument("output", type=pathlib.Path)
    args = parser.parse_args()
    manifest = json.loads(pathlib.Path(__file__).with_name("marks.json").read_text())
    args.output.mkdir(parents=True, exist_ok=True)
    for case in manifest["cases"]:
        root = args.output / case["id"]
        root.mkdir()
        subprocess.run([str(args.runner), "--box", "watercolor", "init", str(root)], check=True)
        counter = 0

        def run(source):
            nonlocal counter
            counter += 1
            script = root / "chunk.lua"
            script.write_text(f'local chunk_id={counter}\n' + source + '\nassert(balance().conserved)\n')
            subprocess.run([str(args.runner), "--box", "watercolor", "run", str(root), str(script)], check=True, stdout=subprocess.DEVNULL)

        def state(x=9.6, y=6.4):
            canvas = json.loads((root / "checkpoint.json").read_text())["canvas"]
            cell = canvas["cell_mm"]
            indices = [yy * canvas["width"] + xx
                       for yy in range(int((y - .6) / cell), int((y + .6) / cell))
                       for xx in range(int((x - .6) / cell), int((x + .6) / cell))]
            water = sum(canvas["water"][i] for i in indices) / len(indices)
            pore = sum(canvas["capillary"][i] for i in indices) / len(indices)
            stage = "shiny" if water >= .04 else "satin" if water > .004 else "moist" if pore >= .05 else "damp" if pore >= .0125 else "dry"
            return {"time": canvas["tick"] / 16 + canvas["residual_ns"] / 1e9,
                    "water": water, "pore": pore, "stage": stage,
                    "max_water": max(canvas["water"]), "max_pore": max(canvas["capillary"])}

        def wait_stage(target, x=9.6, y=6.4):
            elapsed = 0
            order = ["dry", "damp", "moist", "satin", "shiny"]
            while order.index(state(x, y)["stage"]) > order.index(target):
                run("wait(2)")
                elapsed += 2
                if elapsed > 300:
                    raise RuntimeError(f"{case['id']} did not reach {target}")
            return state(x, y)

        def stroke(pigment, y, x0=2.4, x1=16.8, width=2.4):
            parts = []
            n = manifest["brush"]["segments"]
            for k in range(n):
                t = (k + .5) / n
                pressure = min(1, t / .2, (1 - t) / .2)
                w = width * (.15 + .85 * pressure)
                a, b = x0 + (x1 - x0) * k / n, x0 + (x1 - x0) * (k + 1) / n
                area = w * abs(b - a)
                mass = manifest["brush"]["ours_areal_pigment"] * area
                water = manifest["brush"]["water_film_mm"] * area
                name = f"c{counter + 1}s{k}"
                colors = "{}" if pigment is None else f"{{{pigment}={mass:.12f}}}"
                parts.append(f'local b=brush{{name="{name}",width={w:.12f}}}\nb:load{{water={water:.12f},pigments={colors}}}\nb:stroke({{{{{a:.12f},{y}}},{{{b:.12f},{y}}}}},{{dry=false,pressure={pressure:.12f}}})\nwait(1/60)')
            run("\n".join(parts))

        run('paper{kind="cold-press",size=19.2,aspect=1.5,cell=0.05,seed=71}')
        receipt = {"id": case["id"], "initial": state()}
        if case.get("paper_stage") in ("damp", "shiny"):
            for y in [4.8, 6.4, 8.0]:
                stroke(None, y)
            if case["paper_stage"] == "damp":
                wait_stage("damp")
            receipt["before_paint"] = state()
        if case["kind"] == "stroke":
            stroke("ultramarine", 6.4)
        elif case["kind"] == "touch":
            stroke("ultramarine", 5.2)
            receipt["before_second"] = state(9.6, 5.2)
            stroke("earth", 7.6)
        elif case["kind"] == "overlap":
            stroke("ultramarine", 6.4, x1=12)
            if case["second_stage"] == "damp":
                wait_stage("damp")
            receipt["before_second"] = state()
            stroke("earth", 6.4, x0=7.2)
        elif case["kind"] == "wash":
            for row, y in enumerate([3.2, 4.8, 6.4, 8, 9.6]):
                stroke("ultramarine", y, x0=2.4 if row % 2 == 0 else 16.8, x1=16.8 if row % 2 == 0 else 2.4)
        receipt["stages"] = []
        for stage in ["laid", "satin", "damp", "dry"]:
            if stage in ("satin", "damp"):
                wait_stage(stage)
            elif stage == "dry":
                run('wait(300)\nassert(balance().dry)')
            run(f'look{{path="{stage}-color.png",mode="color",width=384}}\nlook{{path="{stage}-gallery.png",mode="gallery",width=384}}')
            receipt["stages"].append({"label": stage, **state()})
            print(case["id"], stage, state(), flush=True)
        (root / "receipt.json").write_text(json.dumps(receipt, indent=2))
        (root / "checkpoint.json").unlink()
        (root / "chunk.lua").unlink()


if __name__ == "__main__":
    main()
