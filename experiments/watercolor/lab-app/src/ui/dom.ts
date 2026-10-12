/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/** Minimal DOM helpers, shared voice with the other Lab experiments. The app
 * is one screen of controls around a canvas; a framework would mostly get in
 * the way of the part that matters. */

type Child = Node | string | null | undefined | false;

type AttrValue = string | number | boolean | null | undefined;

export type Attrs = Record<string, AttrValue | EventListener>;

/* Source: src/ui/dom.ts:11-43 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function el<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  attrs: Attrs = {},
  ...children: Child[]
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);

  for (const [key, value] of Object.entries(attrs)) {
    if (value === null || value === undefined || value === false) continue;

    if (typeof value === "function") {
      // Listener-ness is decided by the key, never inferred from the value's
      // runtime type — an inferred listener is a failure with nothing to see.
      if (!key.startsWith("on")) {
        throw new TypeError(`el(): "${key}" was given a function; event keys must start with "on"`);
      }
      node.addEventListener(key.slice(2).toLowerCase(), value);
    } else if (key === "class") {
      node.className = String(value);
    } else if (value === true) {
      node.setAttribute(key, "");
    } else {
      node.setAttribute(key, String(value));
    }
  }

  for (const child of children) {
    if (child === null || child === undefined || child === false) continue;
    node.append(typeof child === "string" ? document.createTextNode(child) : child);
  }

  return node;
}

/**
 * A two-step destructive button, same rationale as everywhere else in the
 * Lab: clearing the sheet has no undo past the snapshot ring, so it should
 * not fire on a single click, and a native confirm() would break the page's
 * voice to ask. Both labels are always in the button, stacked in one grid
 * cell, so arming cannot reflow the toolbar.
 */
/* Source: src/ui/dom.ts:52-76 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function confirmButton(
  label: string,
  armedLabel: string,
  onConfirm: () => void,
  className = "btn btn-ghost"
): HTMLButtonElement {
  const btn = el(
    "button",
    { class: `${className} confirm-btn`, type: "button" },
    el("span", { class: "confirm-rest" }, label),
    el("span", { class: "confirm-armed" }, armedLabel)
  );
  let disarm = 0;
  btn.addEventListener("click", /* Source: src/ui/dom.ts:65-74 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
    if (btn.classList.contains("is-armed")) {
      window.clearTimeout(disarm);
      btn.classList.remove("is-armed");
      onConfirm();
      return;
    }
    btn.classList.add("is-armed");
    disarm = window.setTimeout(/* Source: src/ui/dom.ts:73-73 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => btn.classList.remove("is-armed"), 3000);
  });
  return btn;
}
