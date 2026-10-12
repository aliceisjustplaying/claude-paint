/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/**
 * Sheet persistence: one saved painting in IndexedDB (localStorage cannot
 * hold a float layer this size). The dried layer plus the paper's identity
 * is the whole document — wet state dies with the tab, like real water.
 */

import type { PaperKind } from "../engine/paper";

export interface SavedSheet {
  width: number;
  height: number;
  paperKind: PaperKind;
  paperSeed: number;
  /** The dried reflectance layer, RGBA float32. */
  dried: Float32Array;
  savedAt: number;
}

const DB_NAME = "watercolor-lab";
const STORE = "sheets";
const KEY = "current";

/* Source: src/data/store.ts:23-32 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ function openDB(): Promise<IDBDatabase> {
  return new Promise(/* Source: src/data/store.ts:24-31 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (resolve, reject) => {
    const req = indexedDB.open(DB_NAME, 1);
    req.onupgradeneeded = /* Source: src/data/store.ts:26-28 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
      if (!req.result.objectStoreNames.contains(STORE)) req.result.createObjectStore(STORE);
    };
    req.onsuccess = /* Source: src/data/store.ts:29-29 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => resolve(req.result);
    req.onerror = /* Source: src/data/store.ts:30-30 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => reject(req.error);
  });
}

/* Source: src/data/store.ts:34-44 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export async function saveSheet(sheet: SavedSheet): Promise<void> {
  const db = await openDB();
  await new Promise<void>(/* Source: src/data/store.ts:36-42 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (resolve, reject) => {
    const tx = db.transaction(STORE, "readwrite");
    // Store the buffer, not the typed array, so structured clone stays lean.
    tx.objectStore(STORE).put({ ...sheet, dried: sheet.dried.buffer }, KEY);
    tx.oncomplete = /* Source: src/data/store.ts:40-40 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => resolve();
    tx.onerror = /* Source: src/data/store.ts:41-41 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => reject(tx.error);
  });
  db.close();
}

/* Source: src/data/store.ts:46-62 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export async function loadSheet(): Promise<SavedSheet | null> {
  try {
    const db = await openDB();
    const record = await new Promise<unknown>(/* Source: src/data/store.ts:49-54 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (resolve, reject) => {
      const tx = db.transaction(STORE, "readonly");
      const req = tx.objectStore(STORE).get(KEY);
      req.onsuccess = /* Source: src/data/store.ts:52-52 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => resolve(req.result);
      req.onerror = /* Source: src/data/store.ts:53-53 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => reject(req.error);
    });
    db.close();
    if (!record || typeof record !== "object") return null;
    const raw = record as Omit<SavedSheet, "dried"> & { dried: ArrayBuffer };
    return { ...raw, dried: new Float32Array(raw.dried) };
  } catch {
    return null;
  }
}
