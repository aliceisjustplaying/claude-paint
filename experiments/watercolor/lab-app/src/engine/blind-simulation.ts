/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * UI adapter for src/engine/simulation.ts at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: ../../LICENSE. */
import { Simulation } from "./simulation";
import { CpuSimulation, type SheetSimulation } from "./cpu-simulation";

// UI adapter for src/ui/app.ts:119-128 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
export function createBlindSimulation(canvas: HTMLCanvasElement, width: number, height: number): {
  sim: SheetSimulation; select(label: "A" | "B"): void; reveal(): void;
} {
  const gpuFirst = (crypto.getRandomValues(new Uint8Array(1))[0] & 1) === 0;
  let activeLabel: "A" | "B" = "A";
  const engines = new Map<"A" | "B", SheetSimulation>();
  // Adapter for src/engine/simulation.ts:173-233 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  const create = (label: "A" | "B"): SheetSimulation => {
    if ((label === "A") === gpuFirst) return new Simulation(canvas, width, height);
    return new CpuSimulation(canvas, width, height);
  };
  let active = create(activeLabel);
  engines.set(activeLabel, active);
  const sim = new Proxy({} as SheetSimulation, {
    // UI forwarding adapter for src/ui/app.ts:119-1045 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    get(_target, property) {
      const value = Reflect.get(active, property);
      return typeof value === "function" ? value.bind(active) : value;
    },
    // UI forwarding adapter for src/ui/app.ts:119-1045 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    set(_target, property, value) { return Reflect.set(active, property, value); },
  });
  return {
    sim,
    // Engine switch adapter for src/engine/simulation.ts:239-265 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
    select(label) {
      if (label === activeLabel) return;
      const prior = active;
      const next = engines.get(label) ?? create(label);
      next.params = { ...prior.params };
      next.foreverWet = prior.foreverWet;
      next.setPaper(prior.paperKind, true, prior.paperSeed);
      prior.resetState();
      engines.set(label, next);
      active = next;
      activeLabel = label;
    },
    // Reveal-only key for the blind switch, added around src/ui/app.ts:119-128 at the pinned commit.
    reveal() {
      console.info(`Engine A: ${gpuFirst ? "original WebGL simulation" : "Rust CPU WASM simulation"}; Engine B: ${gpuFirst ? "Rust CPU WASM simulation" : "original WebGL simulation"}`);
    },
  };
}
