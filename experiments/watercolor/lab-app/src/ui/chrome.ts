/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
import { el } from "./dom";

/**
 * The site chrome. This tool is EXP-040 in the Lab on ryankm.com and is
 * hosted off-site, so the bar is the only thing carrying the brand and the
 * only route home — same shape as every other experiment served from its own
 * subdomain.
 */

export const SITE_URL = "https://ryankm.com";
export const LAB_URL = `${SITE_URL}/lab`;

/* Source: src/ui/chrome.ts:13-29 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function appBar(): HTMLElement {
  return el(
    "div",
    { class: "site-bar" },
    el(
      "div",
      { class: "container bar-inner" },
      el("a", { class: "wordmark", href: SITE_URL }, "Ryan McCarty", el("span", { class: "dot" }, ".")),
      el(
        "a",
        { class: "bar-back", href: LAB_URL },
        "Back to the Lab",
        el("span", { "aria-hidden": "true" }, "↗")
      )
    )
  );
}
