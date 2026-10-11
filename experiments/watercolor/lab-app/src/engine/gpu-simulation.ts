import init, { WasmSimulation } from "../../wasm/watercolor_lab_wasm";
import { DEFAULT_PARAMS, type Params, type Simulation, type XrayField } from "./simulation";
import type { PaperKind } from "./paper";
import type { Stamp } from "../paint/brush";
import type { Pigment } from "../paint/pigments";

export const initializeGpu = init;
export type SheetSimulation = Omit<Pick<Simulation, "simWidth" | "simHeight" | "params" | "foreverWet" | "paperKind" | "paperSeed" | "setPaper" | "resetState" | "splat" | "addSalt" | "addScrub" | "dryFast" | "update" | "stepMany" | "render" | "snapshot" | "undo" | "clearSheet" | "probe" | "serializeDried" | "restoreDried">, "probe" | "serializeDried"> & {
  probe(x: number, y: number, radius: number): Promise<{ dried: number; workable: number; wet: number }>;
  serializeDried(): Promise<Float32Array>;
};

export class GpuSimulation implements SheetSimulation {
  params: Params = { ...DEFAULT_PARAMS };
  foreverWet = false;
  paperKind: PaperKind = "cold-press";
  paperSeed: number;
  private configuration = "";

  private constructor(private core: WasmSimulation, readonly simWidth: number, readonly simHeight: number, seed: number) {
    this.paperSeed = seed;
  }

  static async create(canvas: HTMLCanvasElement, width: number, height: number): Promise<GpuSimulation> {
    const seed = 7 + Math.floor(Math.random() * 1e6);
    const core = await WasmSimulation.create(canvas, width, height, "cold-press", seed);
    return new GpuSimulation(core, width, height, seed);
  }

  private configure(): void {
    const key = `${JSON.stringify(this.params)}:${this.foreverWet}`;
    if (key === this.configuration) return;
    const params = Object.fromEntries(Object.entries(this.params).map(([name, value]) => [name.replace(/[A-Z]/g, (c) => `_${c.toLowerCase()}`), value]));
    this.core.configure(JSON.stringify(params), this.foreverWet);
    this.configuration = key;
  }

  setPaper(kind: PaperKind, clear = true, seed = this.paperSeed): void {
    this.paperKind = kind;
    this.paperSeed = seed;
    this.core.set_paper(kind, clear, seed);
  }
  resetState(): void { this.core.reset_state(); }
  splat(stamps: Stamp[], pigment: Pigment | null): void { this.core.splat(packStamps(stamps), JSON.stringify(pigment)); }
  addSalt(x: number, y: number, radius = 46, seed = Math.floor(Math.random() * 1e9)): void { this.core.add_salt(x, y, radius, seed); }
  addScrub(stamps: Stamp[]): void { this.core.add_scrub(packStamps(stamps)); }
  dryFast(): void { this.core.dry_fast(); }
  update(): void { this.configure(); this.core.update(); }
  stepMany(steps: number, evapMul = 1): void { this.configure(); this.core.step_many(steps, evapMul); }
  render(backlight: number, field: XrayField = 0): void { this.core.render(backlight, field); }
  snapshot(): void { this.core.snapshot(); }
  undo(): boolean { return this.core.undo(); }
  clearSheet(): void { this.core.clear_sheet(); }
  async probe(x: number, y: number, radius = 3): Promise<{ dried: number; workable: number; wet: number }> {
    const [dried, workable, wet] = await this.core.probe(x, y, radius);
    return { dried, workable, wet };
  }
  async serializeDried(): Promise<Float32Array> {
    return await this.core.serialize_dried();
  }
  restoreDried(data: Float32Array): void { this.core.restore_dried(data); }
}

function packStamps(stamps: Stamp[]): Float64Array {
  return Float64Array.from(stamps.slice(0, 4096).flatMap((s) => [s.x, s.y, s.radius, s.water, s.pigment, s.dryness]));
}
