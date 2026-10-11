/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Adapter for src/engine/simulation.ts at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: ../../LICENSE. */
import init, { WasmSimulation } from "../../wasm/watercolor_lab_wasm";
import { bindTextures, createFullscreenVAO, createGL, createProgram, createTarget, type GL, type Program, type Target } from "./gl";
import { DEFAULT_PARAMS, XRAY_FIELDS, type Params, type Simulation, type XrayField } from "./simulation";
import { RENDER_FRAG, XRAY_FRAG } from "./shaders";
import type { PaperKind } from "./paper";
import type { Stamp } from "../paint/brush";
import type { Pigment } from "../paint/pigments";

export const initializeCpu = init;
export type SheetSimulation = Pick<Simulation, "simWidth" | "simHeight" | "params" | "foreverWet" | "paperKind" | "paperSeed" | "setPaper" | "resetState" | "splat" | "addSalt" | "addScrub" | "dryFast" | "update" | "stepMany" | "render" | "snapshot" | "undo" | "clearSheet" | "probe" | "serializeDried" | "restoreDried">;

export class CpuSimulation implements SheetSimulation {
  params: Params = { ...DEFAULT_PARAMS };
  foreverWet = false;
  paperKind: PaperKind = "cold-press";
  paperSeed = 7 + Math.floor(Math.random() * 1e6);
  private core: WasmSimulation;
  private ctx: GL;
  private vao: WebGLVertexArrayObject;
  private paint: Program;
  private xray: Program;
  private targets: Record<string, Target> = {};
  private dirty = true;
  private configuration = "";

  // Adapter for src/engine/simulation.ts:173-233 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  constructor(canvas: HTMLCanvasElement, readonly simWidth: number, readonly simHeight: number) {
    this.core = new WasmSimulation(simWidth, simHeight, this.paperKind, this.paperSeed);
    this.ctx = createGL(canvas);
    this.vao = createFullscreenVAO(this.ctx);
    this.paint = createProgram(this.ctx, RENDER_FRAG);
    this.xray = createProgram(this.ctx, XRAY_FRAG);
    for (const name of ["flow", "susK", "susS", "susP", "depK", "depS", "cap", "dried", "paper"]) {
      this.targets[name] = createTarget(this.ctx, simWidth, simHeight);
    }
  }

  // Adapter for src/engine/simulation.ts:51-100,400-405 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  private configure(): void {
    const params = JSON.stringify(this.params);
    const key = `${params}:${this.foreverWet}`;
    if (key === this.configuration) return;
    const rustParams = Object.fromEntries(Object.entries(this.params).map(([name, value]) => [name.replace(/[A-Z]/g, (c) => `_${c.toLowerCase()}`), value]));
    this.core.configure(JSON.stringify(rustParams), this.foreverWet);
    this.configuration = key;
  }

  // Adapter for src/engine/simulation.ts:239-252 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  setPaper(kind: PaperKind, clear = true, seed = this.paperSeed): void {
    this.paperKind = kind;
    this.paperSeed = seed;
    this.core.set_paper(kind, clear, seed);
    this.dirty = true;
  }

  // Adapter for src/engine/simulation.ts:254-265 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  resetState(): void { this.core.reset_state(); this.dirty = true; }

  // Adapter for src/engine/simulation.ts:270-311 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  splat(stamps: Stamp[], pigment: Pigment | null): void {
    this.core.splat(packStamps(stamps), JSON.stringify(pigment));
    this.dirty = true;
  }

  // Adapter for src/engine/simulation.ts:316-354 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  addSalt(x: number, y: number, radius = 46, seed = Math.floor(Math.random() * 1e9)): void {
    this.core.add_salt(x, y, radius, seed);
    this.dirty = true;
  }

  // Adapter for src/engine/simulation.ts:359-387 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  addScrub(stamps: Stamp[]): void { this.core.add_scrub(packStamps(stamps)); this.dirty = true; }

  // Adapter for src/engine/simulation.ts:393-405 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  dryFast(): void { this.core.dry_fast(); }
  // Adapter for src/engine/simulation.ts:400-405 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  update(): void { this.configure(); this.core.update(); this.dirty = true; }
  // Adapter for src/engine/simulation.ts:408-410 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  stepMany(steps: number, evapMul = 1): void { this.configure(); this.core.step_many(steps, evapMul); this.dirty = true; }

  // Display-only adapter for src/engine/simulation.ts:579-619 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  render(backlight: number, field: XrayField = 0): void {
    const { gl, canvas } = this.ctx;
    if (this.dirty) {
      for (const [name, target] of Object.entries(this.targets)) {
        gl.bindTexture(gl.TEXTURE_2D, target.texture);
        gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, this.simWidth, this.simHeight, gl.RGBA, gl.FLOAT, this.core.field(name));
      }
      this.dirty = false;
    }
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, canvas.width, canvas.height);
    const program = field === 0 ? this.paint : this.xray;
    gl.useProgram(program.program);
    gl.uniform2f(program.uniforms.uTexel, 1 / this.simWidth, 1 / this.simHeight);
    if (field === 0) {
      gl.uniform1f(program.uniforms.uBacklight, backlight);
      gl.uniform1f(program.uniforms.uZoomGrain, this.params.zoomGrain);
    } else {
      gl.uniform1i(program.uniforms.uField, field);
      gl.uniform1f(program.uniforms.uScale, XRAY_FIELDS[field].scale);
    }
    const names = field === 0 ? ["flow", "susK", "susS", "depK", "depS", "dried", "paper", "susP"] : ["flow", "susK", "depK", "cap", "dried"];
    bindTextures(this.ctx, program, names.map((name) => [`u${name[0].toUpperCase()}${name.slice(1)}`, this.targets[name].texture]));
    gl.bindVertexArray(this.vao);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    gl.bindVertexArray(null);
  }

  // Adapter for src/engine/simulation.ts:637-653 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  snapshot(): void { this.core.snapshot(); }
  // Adapter for src/engine/simulation.ts:655-664 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  undo(): boolean { const changed = this.core.undo(); this.dirty = true; return changed; }
  // Adapter for src/engine/simulation.ts:666-676 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  clearSheet(): void { this.core.clear_sheet(); this.dirty = true; }
  // Adapter for src/engine/simulation.ts:682-704 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  probe(x: number, y: number, radius = 3): { dried: number; workable: number; wet: number } {
    const [dried, workable, wet] = this.core.probe(x, y, radius);
    return { dried, workable, wet };
  }
  // Adapter for src/engine/simulation.ts:708-714 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  serializeDried(): Float32Array { return this.core.field("dried"); }
  // Adapter for src/engine/simulation.ts:731-740 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
  restoreDried(data: Float32Array): void { this.core.restore_dried(data); this.dirty = true; }
}

// Adapter for src/engine/simulation.ts:270-288 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
function packStamps(stamps: Stamp[]): Float64Array {
  return Float64Array.from(stamps.slice(0, 4096).flatMap((s) => [s.x, s.y, s.radius, s.water, s.pigment, s.dryness]));
}
