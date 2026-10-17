import asyncio
import json
import os
from pathlib import Path

import anyio


async def visit(studio, occasion, note=None):
    if not os.environ.get("PAINTER_CRITIC"):
        return None
    cli = os.environ.get("PAINTER_CRITIC_CLI", str(Path(__file__).resolve().parents[1] / "painter/visit-cli.ts"))
    request = {"occasion": occasion, **({"note": note} if note is not None else {})}
    process = await asyncio.create_subprocess_exec(
        "node", cli, cwd=studio, stdin=asyncio.subprocess.PIPE,
        stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.PIPE,
    )
    try:
        out, err = await process.communicate(json.dumps(request).encode())
    except BaseException:
        if process.returncode is None:
            process.terminate()
        with anyio.CancelScope(shield=True):
            await process.wait()
        raise
    if process.returncode:
        raise ValueError(err.decode(errors="replace").strip())
    return json.loads(out)
