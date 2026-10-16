import { postAuthor } from "./gate.ts";
import type { IngestEvent, Post } from "./types.ts";

function object(value: unknown): Record<string, unknown> | null {
  return typeof value === "object" && value !== null && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : null;
}

function mentions(record: Record<string, unknown>, text: string): string[] {
  if (!Array.isArray(record["facets"])) {
    return [];
  }
  const result = new Set<string>();
  const byteLength = Buffer.byteLength(text);
  for (const value of record["facets"] as unknown[]) {
    const facet = object(value);
    const index = object(facet?.["index"]);
    const start = index?.["byteStart"];
    const end = index?.["byteEnd"];
    if (
      typeof start !== "number" ||
      typeof end !== "number" ||
      !Number.isInteger(start) ||
      !Number.isInteger(end) ||
      start < 0 ||
      end <= start ||
      end > byteLength ||
      !Array.isArray(facet?.["features"])
    ) {
      continue;
    }
    for (const feature of facet["features"] as unknown[]) {
      const item = object(feature);
      const did = item?.["did"];
      if (
        item?.["$type"] === "town.delve.richtext.facet#mention" &&
        typeof did === "string" &&
        postAuthor(`at://${did}/town.delve.feed.post/test`) === did
      ) {
        result.add(did);
      }
    }
  }
  return [...result];
}

export function parseEvent(value: unknown, botDids: string[] = []): IngestEvent | null {
  const message = object(value);
  if (message?.["kind"] !== "commit") {
    return null;
  }
  const commit = object(message["commit"]);
  if (commit?.["collection"] !== "town.delve.feed.post") {
    return null;
  }
  const cursor = message["time_us"];
  const { did } = message;
  const key = commit["rkey"];
  const { operation } = commit;
  if (
    typeof cursor !== "number" ||
    !Number.isSafeInteger(cursor) ||
    cursor < 0 ||
    typeof did !== "string" ||
    typeof key !== "string" ||
    (operation !== "create" && operation !== "update" && operation !== "delete")
  ) {
    throw new Error("Invalid Jetstream post envelope");
  }
  const uri = `at://${did}/town.delve.feed.post/${key}`;
  if (postAuthor(uri) !== did) {
    throw new Error("Invalid Jetstream post URI");
  }
  if (operation === "delete") {
    return { cursor, operation, uri, post: null };
  }
  const record = object(commit["record"]);
  const text = record?.["text"];
  const createdAt = record?.["createdAt"];
  const { cid } = commit;
  if (
    record?.["$type"] !== "town.delve.feed.post" ||
    typeof text !== "string" ||
    Buffer.byteLength(text) > 500_000 ||
    typeof createdAt !== "string" ||
    !Number.isFinite(Date.parse(createdAt)) ||
    typeof cid !== "string" ||
    cid === ""
  ) {
    throw new Error("Invalid Jetstream post record");
  }
  let rootUri = uri;
  let parentUri: string | null = null;
  if (record["reply"] !== undefined) {
    const reply = object(record["reply"]);
    const root = object(reply?.["root"]);
    const parent = object(reply?.["parent"]);
    if (
      typeof root?.["uri"] !== "string" ||
      typeof parent?.["uri"] !== "string" ||
      postAuthor(root["uri"]) === null ||
      postAuthor(parent["uri"]) === null
    ) {
      throw new Error("Invalid Jetstream reply references");
    }
    rootUri = root["uri"];
    parentUri = parent["uri"];
  }
  const post: Post = {
    uri,
    cid,
    did,
    handle: "",
    text,
    createdAt,
    rootUri,
    parentUri,
    mentions: mentions(record, text),
    isBot: botDids.includes(did),
  };
  return { cursor, operation, uri, post };
}
