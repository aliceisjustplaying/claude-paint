import type { Config, Post } from "./types.ts";

function object(value: unknown): Record<string, unknown> | null {
  return typeof value === "object" && value !== null && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : null;
}

export function parsePostView(value: unknown): Post | null {
  const view = object(value);
  const author = object(view?.["author"]);
  const record = object(view?.["record"]);
  if (
    !view ||
    !author ||
    !record ||
    typeof view["uri"] !== "string" ||
    typeof view["cid"] !== "string" ||
    typeof author["did"] !== "string" ||
    typeof author["handle"] !== "string" ||
    typeof record["text"] !== "string" ||
    typeof record["createdAt"] !== "string"
  ) {
    return null;
  }
  const reply = object(record["reply"]);
  const root = object(reply?.["root"]);
  const parent = object(reply?.["parent"]);
  const mentions: string[] = [];
  if (Array.isArray(record["facets"])) {
    for (const facet of record["facets"]) {
      const features = object(facet)?.["features"];
      if (!Array.isArray(features)) {
        continue;
      }
      for (const feature of features) {
        const item = object(feature);
        if (
          item?.["$type"] === "town.delve.richtext.facet#mention" &&
          typeof item["did"] === "string"
        ) {
          mentions.push(item["did"]);
        }
      }
    }
  }
  return {
    uri: view["uri"],
    cid: view["cid"],
    did: author["did"],
    handle: author["handle"],
    text: record["text"],
    createdAt: record["createdAt"],
    rootUri: typeof root?.["uri"] === "string" ? root["uri"] : view["uri"],
    parentUri: typeof parent?.["uri"] === "string" ? parent["uri"] : null,
    mentions,
    isBot:
      Array.isArray(author["labels"]) &&
      author["labels"].some((label: unknown) => object(label)?.["val"] === "bot"),
  };
}

async function query(
  config: Config,
  method: string,
  params: Record<string, string>,
  fetcher: typeof fetch = fetch,
): Promise<unknown> {
  const url = new URL(`/xrpc/${method}`, config.appviewUrl);
  url.search = new URLSearchParams(params).toString();
  const response = await fetcher(url, { method: "GET", redirect: "error", signal: AbortSignal.timeout(30_000) });
  if (!response.ok) {
    throw new Error(`Delve public read failed (HTTP ${response.status})`);
  }
  return response.json();
}

export async function fetchThread(
  uri: string,
  config: Config,
  fetcher: typeof fetch = fetch,
): Promise<Post[]> {
  const result = object(
    await query(
      config,
      "town.delve.feed.getPostThread",
      {
        uri,
        depth: "6",
        parentHeight: "80",
      },
      fetcher,
    ),
  );
  const posts = new Map<string, Post>();
  const pending: unknown[] = [result?.["thread"]];
  let visited = 0;
  while (pending.length > 0 && visited < 1000) {
    visited += 1;
    const node = object(pending.pop());
    if (!node) {
      continue;
    }
    const post = parsePostView(node["post"]);
    if (post) {
      posts.set(post.uri, post);
    }
    if (node["parent"]) {
      pending.push(node["parent"]);
    }
    if (Array.isArray(node["replies"])) {
      pending.push(...node["replies"]);
    }
  }
  return [...posts.values()].toSorted((left, right) =>
    left.createdAt.localeCompare(right.createdAt),
  );
}

export function fetchProfile(
  did: string,
  config: Config,
  fetcher: typeof fetch = fetch,
): Promise<unknown> {
  return query(config, "town.delve.actor.getProfile", { actor: did }, fetcher);
}

export async function fetchTown(config: Config, limit = 30, fetcher: typeof fetch = fetch): Promise<Post[]> {
  const result = object(
    await query(config, "town.delve.feed.getFeed", {
      feed: "at://did:plc:qzqct2rrq4u2gmy5g3mjxske/town.delve.feed.generator/town",
      limit: String(limit),
    }, fetcher),
  );
  if (!Array.isArray(result?.["feed"])) {
    throw new TypeError("Delve returned an invalid town feed");
  }
  const posts: Post[] = [];
  for (const item of result["feed"]) {
    const post = parsePostView(object(item)?.["post"]);
    if (post) {
      posts.push(post);
    }
  }
  return posts;
}
