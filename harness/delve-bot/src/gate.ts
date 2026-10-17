import { existsSync } from "node:fs";
import type { Config, Post } from "./types.ts";

export function postAuthor(uri: string): string | null {
  const match = /^at:\/\/(?<did>did:[a-z]+:[A-Za-z0-9._:%-]*[A-Za-z0-9._-])\/town\.delve\.feed\.post\/(?<key>[A-Za-z0-9._~:-]{1,512})$/u.exec(uri);
  if (match === null || match.groups?.["key"] === "." || match.groups?.["key"] === ".." || /%(?![0-9A-Fa-f]{2})/u.test(uri)) return null;
  return match.groups?.["did"] ?? null;
}

export function addressed(post: Post, config: Config): boolean {
  return post.mentions.includes(config.did) || (post.parentUri !== null && postAuthor(post.parentUri) === config.did);
}

export function stopped(config: Config, env: NodeJS.ProcessEnv = process.env): boolean {
  return env["STILLWET_KILL_SWITCH"] !== undefined || existsSync(config.killFile);
}

export function instruction(post: Post, config: Config): string {
  return post.text.replaceAll(new RegExp(`@${config.handle.replaceAll(".", "\\.")}(?=\\s|$)`, "giu"), "").trim();
}

export function optOut(post: Post, config: Config): boolean {
  return addressed(post, config) && /^(?:stop|opt[ -]?out|do not reply|don't reply|stop replying|leave me alone)[.!]?$/iu.test(instruction(post, config));
}
