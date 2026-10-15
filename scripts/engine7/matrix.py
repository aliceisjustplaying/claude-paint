import argparse
import json
from pathlib import Path
import subprocess
import sys


def run(args):
    script = Path(__file__).with_name("replay.py")
    for case in args.cases:
        source = args.inputs / f"{case}.lua"
        if not source.is_file():
            raise ValueError(f"missing prepared input: {source}")
    for case in args.cases:
        for workers in args.workers:
            for engine in [5, 7]:
                command = [sys.executable, str(script), "replay", str(args.inputs / f"{case}.lua"),
                           "--easel", str(args.easel), "--output", str(args.output / f"{case}-e{engine}-w{workers}"),
                           "--workers", str(workers), "--engine", str(engine)]
                if args.revision:
                    command.extend(["--revision", args.revision])
                print(" ".join(command), flush=True)
                subprocess.run(command, check=True)


def report(root):
    print("| Case | Engine | Workers | Wall s | CPU/wall | Peak RSS GiB | Ready width |")
    print("|---|---:|---:|---:|---:|---:|---:|")
    for path in sorted(root.glob("*/receipt.json")):
        row = json.loads(path.read_text())
        if row["exit_code"] != 0:
            print(f"Failed receipt: {path}", file=sys.stderr)
            continue
        print(f"| {path.parent.name} | {row['engine']} | {row['workers']} | "
              f"{row['wall_s']:.3f} | {row['cpu_over_wall']:.3f} | {row['peak_rss_bytes'] / 2**30:.3f} | "
              f"{row.get('ready_width') or 'not measured'} |")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Sequential matched hyper runs; no build or encoding overlap")
    sub = parser.add_subparsers(dest="action", required=True)
    p = sub.add_parser("run")
    p.add_argument("--inputs", type=Path, required=True)
    p.add_argument("--easel", type=Path, required=True)
    p.add_argument("--output", type=Path, required=True)
    p.add_argument("--cases", nargs="+", default=["r40-12", "cezanne-12", "r40-full"])
    p.add_argument("--workers", type=int, nargs="+", default=[1, 4, 8, 16])
    p.add_argument("--revision")
    p = sub.add_parser("report")
    p.add_argument("root", type=Path)
    args = parser.parse_args()
    if args.action == "run":
        run(args)
    else:
        report(args.root)
