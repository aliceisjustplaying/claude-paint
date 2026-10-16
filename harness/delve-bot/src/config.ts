import { readFileSync } from "node:fs";
import { Type, type TSchema } from "typebox";
import { Value } from "typebox/value";
import type { Config } from "./types.ts";

const int = (minimum: number, maximum: number) => Type.Integer({ minimum, maximum });
const cap = (value: TSchema) => Type.Optional(Type.Union([value, Type.Null()]));
const did = Type.String({ pattern: "^did:[a-z]+:[A-Za-z0-9._:%-]+$" });
const path = Type.String({ minLength: 1, maxLength: 1024 });
const schema = Type.Object({
  did,
  handle: Type.String({ pattern: "^[a-z0-9.-]+$", maxLength: 253 }),
  dryRun: Type.Boolean(),
  livePosting: Type.Boolean(),
  liveStreaming: Type.Boolean(),
  database: Type.String({ minLength: 1 }),
  killFile: Type.String({ minLength: 1 }),
  jetstreamUrl: Type.String({ pattern: "^wss?://" }),
  appviewUrl: Type.String({ pattern: "^https?://" }),
  pdsUrl: Type.String({ pattern: "^https?://" }),
  anthropicUrl: Type.String({ pattern: "^https?://" }),
  blockedDids: Type.Array(did),
  optedOutDids: Type.Array(did),
  botDids: Type.Array(did),
  contentCheck: Type.Object({ model: Type.Literal("claude-opus-5-5"), timeoutSeconds: int(1, 120) }, { additionalProperties: false }),
  streaming: Type.Object({
    enabled: Type.Boolean(), includeLink: Type.Boolean(), concurrent: Type.Literal(1),
    maxSeconds: cap(int(1, 7200)), width: int(64, 1280), height: int(64, 720),
    fps: int(1, 15), videoKbps: int(64, 1500), audioKbps: int(32, 96),
    ingestUrl: Type.String({ pattern: "^(rtmps?|file)://" }),
    stationUrl: Type.String({ pattern: "^https://" }),
    viewerUrl: Type.String({ pattern: "^http://127\\.0\\.0\\.1:[0-9]+$" }),
    chromiumPath: Type.String({ pattern: "^/", maxLength: 1024 })
  }, { additionalProperties: false }),
  caps: Type.Object({
    paintingsPerDay: int(0, 20), running: int(1, 4), requesterPerDay: int(0, 4),
    requesterCooldownSeconds: int(3600, 604800), botCooldownSeconds: int(3600, 604800),
    botOnlyTurns: int(1, 4), queue: int(0, 32), queueTtlSeconds: int(1, 86400),
    requestChars: int(1, 2000), requestBytes: int(1, 8000), requestAgeSeconds: int(1, 86400),
    globalPerHour: int(0, 40), threadPerSixHours: int(0, 30),
    pendingEvents: int(1, 10000), pendingReplies: int(1, 256), databaseBytes: int(1048576, 104857600), eventBytes: int(1024, 524288),
    replyChars: int(1, 600), replyBytes: int(1, 3000), imageBytes: int(1, 1000000)
  }, { additionalProperties: false }),
  studio: Type.Object({
    medium: Type.Literal("oil"), width: int(64, 2400), height: int(64, 2400),
    engine: Type.Literal(7), painterModel: Type.Literal("claude-opus-5-5"),
    thinking: Type.Union(["low", "medium", "high", "xhigh", "max"].map(level => Type.Literal(level))),
    template: path, root: Type.String({ pattern: "(^|/)src/a$", maxLength: 1024 }), sessions: path, harness: path,
    maxWallSeconds: cap(int(1, 14400)), maxInputTokens: cap(int(1, 1000000)),
    maxOutputTokens: cap(int(1, 100000)), maxCostUsd: cap(Type.Number({ exclusiveMinimum: 0, maximum: 50 }))
  }, { additionalProperties: false })
}, { additionalProperties: false });

export function validateConfig(value: unknown): Config {
  if (!Value.Check(schema, value)) throw new Error("Invalid config; see config.json for required fields and bounds");
  const config = value as Config;
  for (const endpoint of [config.appviewUrl, config.pdsUrl, config.anthropicUrl]) {
    const url = new URL(endpoint);
    if (url.username || url.password || url.pathname !== "/" || url.search || url.hash ||
        (url.protocol !== "https:" && !isLoopback(endpoint))) throw new Error("Unsafe endpoint");
  }
  if (config.dryRun && (config.livePosting || config.liveStreaming)) throw new Error("Live switches need dryRun off");
  if (config.studio.width * config.studio.height > 4320000) throw new Error("Canvas too large");
  const stream = config.streaming;
  const ingest = new URL(stream.ingestUrl), station = new URL(stream.stationUrl);
  if (ingest.username || ingest.password || ingest.search || ingest.hash || station.username || station.password ||
      station.pathname !== "/" || station.search || station.hash ||
      (ingest.protocol !== "file:" && ingest.pathname !== "/live") ||
      (ingest.protocol === "file:" && (ingest.host || !ingest.pathname.endsWith(".flv"))) ||
      stream.width % 2 || stream.height % 2 || (stream.enabled && config.caps.running !== 1)) throw new Error("Unsafe streaming config");
  return config;
}

export function isLoopback(endpoint: string): boolean {
  const url = new URL(endpoint);
  return url.protocol === "http:" && url.hostname === "127.0.0.1" && !url.username && !url.password;
}

export function loadConfig(path = "config.json"): Config {
  return validateConfig(JSON.parse(readFileSync(path, "utf8")) as unknown);
}
