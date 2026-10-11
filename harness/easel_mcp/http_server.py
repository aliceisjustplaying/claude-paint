# /// script
# requires-python = ">=3.10"
# dependencies = ["mcp==1.30.0", "uvicorn==0.42.0"]
# ///

import argparse
from contextlib import asynccontextmanager
import hmac
import os
from pathlib import Path

from mcp.server.streamable_http_manager import StreamableHTTPSessionManager
from mcp.server.transport_security import TransportSecuritySettings
from starlette.applications import Starlette
from starlette.responses import PlainTextResponse
from starlette.routing import Route
import uvicorn

from remote import RemoteStudio, mcp_server


def app_for(studio, token):
    manager = StreamableHTTPSessionManager(
        mcp_server(studio), json_response=True, stateless=True,
        security_settings=TransportSecuritySettings(allowed_hosts=["127.0.0.1:*", "localhost:*"], allowed_origins=[]),
        max_request_body_size=1_000_000,
    )

    @asynccontextmanager
    async def lifespan(app):
        try:
            async with manager.run():
                yield
        finally:
            await studio.close()

    class Endpoint:
        async def __call__(self, scope, receive, send):
            headers = dict(scope.get("headers", []))
            if not hmac.compare_digest(headers.get(b"authorization", b""), ("Bearer " + token).encode()):
                await PlainTextResponse("Unauthorized", status_code=401, headers={"WWW-Authenticate": "Bearer"})(scope, receive, send)
                return
            await manager.handle_request(scope, receive, send)

    return Starlette(routes=[Route("/mcp", endpoint=Endpoint(), methods=["GET", "POST", "DELETE"])], lifespan=lifespan)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--studio", type=Path, required=True)
    parser.add_argument("--sessions-root", type=Path, default=Path.home() / ".pi/agent/sessions")
    parser.add_argument("--token-file", type=Path)
    parser.add_argument("--port", type=int, default=8790)
    parser.add_argument("--response-wait", type=float, default=20)
    args = parser.parse_args()
    if not 0 < args.response_wait <= 60:
        parser.error("response-wait must be greater than 0 and at most 60 seconds")
    if args.token_file:
        token_file = args.token_file.expanduser()
        if token_file.stat().st_mode & 0o077:
            parser.error("the token file must be readable only by its owner (chmod 600)")
        token = token_file.read_text().strip()
    else:
        token = os.environ.get("EASEL_MCP_TOKEN", "")
    if len(token) < 32 or any(char.isspace() for char in token):
        parser.error("set a token of at least 32 characters in EASEL_MCP_TOKEN or --token-file")
    studio = RemoteStudio(args.studio, args.sessions_root, args.response_wait)
    uvicorn.run(app_for(studio, token), host="127.0.0.1", port=args.port, access_log=False)


if __name__ == "__main__":
    main()
