/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/**
 * The simulator: owns the GL state textures, runs the pass sequence, and is
 * the only thing the UI talks to. One instance per sheet.
 *
 * A substep is: blur wetness → pipe flux → water/velocity → advect pigment →
 * transfer (deposit/lift, both sides from identical inputs) → capillary →
 * dry-fold → clear folded cells. Brush splats land additively between frames.
 */

import {
  bindTextures,
  clearTarget,
  copyTarget,
  createFullscreenVAO,
  createGL,
  createPingPong,
  createProgram,
  createTarget,
  type GL,
  type PingPong,
  type Program,
  type Target,
} from "./gl";
import {
  BLUR_WET_FRAG,
  CAPILLARY_FRAG,
  DRY_CLEAR_DEP_FRAG,
  DRY_CLEAR_SUS_FRAG,
  DRY_FOLD_FRAG,
  FLUX_FRAG,
  LIFT_FRAG,
  MOVE_PIGMENT_FRAG,
  RENDER_FRAG,
  XRAY_FRAG,
  SALT_FRAG,
  SPLAT_FRAG,
  SPLAT_VERT,
  TRANSFER_DEP_FRAG,
  TRANSFER_SUS_FRAG,
  WATER_FRAG,
} from "./shaders";
import { generatePaper, PAPERS, type PaperKind } from "./paper";
import { PAPER_REFLECTANCE } from "../paint/km";
import type { Pigment } from "../paint/pigments";
import type { Stamp } from "../paint/brush";

/** Every tunable in the physics, in one bag, so the whole feel of the medium
 * can be adjusted (or bisected, when a behavior goes wrong) without touching
 * a shader. These defaults are the result of the visual tuning passes logged
 * in docs/session-log.md. */
export interface Params {
  gravity: number;
  damp: number;
  slope: number;
  tension: number;
  saltPull: number;
  evap: number;
  evapEdge: number;
  absorb: number;
  capDiff: number;
  capDry: number;
  velScale: number;
  advect: number;
  edgeDrift: number;
  settle: number;
  granBias: number;
  lift: number;
  saltLift: number;
  /** Extra lift under the lift tool's scrub field. */
  scrubLift: number;
  /** Deposition multiplier at the drying front — the dried rim, the tide
   * line, the backrun's cauliflower edge. */
  edgeSettle: number;
  substeps: number;
  zoomGrain: number;
}

export const DEFAULT_PARAMS: Params = {
  gravity: 0.14,
  damp: 0.982,
  slope: 0.12,
  tension: 0.02,
  saltPull: 0.8,
  evap: 0.00016,
  evapEdge: 0.0016,
  absorb: 0.00045,
  capDiff: 0.04,
  capDry: 0.00025,
  velScale: 20,
  advect: 1.0,
  edgeDrift: 4.5,
  settle: 0.0018,
  granBias: 1.1,
  lift: 0.005,
  saltLift: 0.5,
  scrubLift: 0.9,
  edgeSettle: 14,
  substeps: 3,
  zoomGrain: 0.006,
}

const WATER_SCALE = 0.055;
const PIGMENT_SCALE = 0.13;
const MAX_STAMPS = 4096;
const UNDO_DEPTH = 3;

/** The X-ray views: which state texture, and the value at which its ramp
 * reaches 63% (1 - 1/e) of full ink. Scales were set once against measured
 * fields of a heavy mop wash on cold press (readFloat over the whole sheet at
 * 0, 1, 3 and 8 s) so a typical wash sits mid-ramp; they are constants, never
 * fitted to the frame being drawn. */
export type XrayField = 0 | 1 | 2 | 3 | 4 | 5 | 6;
export const XRAY_FIELDS: Record<XrayField, { id: string; name: string; what: string; scale: number; unit: string }> = {
  0: { id: "paint", name: "Paint", what: "The painting as it looks.", scale: 1, unit: "" },
  1: { id: "water", name: "Water", what: "Standing water on the sheet: the film that flows and carries pigment.", scale: 0.12, unit: "depth" },
  2: { id: "wet", name: "Wet", what: "How wet the surface is, 0 to 1. The wash boundary is where it drops.", scale: 0.6, unit: "wetness" },
  3: { id: "suspended", name: "Suspended", what: "Pigment riding in the water, not yet settled.", scale: 0.15, unit: "concentration" },
  4: { id: "settled", name: "Settled", what: "Pigment settled on the sheet but not yet dried: what the lift tool can reach.", scale: 0.06, unit: "thickness" },
  5: { id: "paper", name: "Paper", what: "Water inside the paper's fibres, where backruns travel. Salt grains in vermillion.", scale: 0.03, unit: "saturation" },
  6: { id: "dried", name: "Dried", what: "Everything cured into the sheet, as optical depth. Permanent.", scale: 1.0, unit: "depth" },
};

export class Simulation {
  readonly simWidth: number;
  readonly simHeight: number;
  params: Params = { ...DEFAULT_PARAMS };

  private ctx: GL;
  private vao: WebGLVertexArrayObject;

  private flow: PingPong;
  private flux: PingPong;
  private susK: PingPong;
  private susS: PingPong;
  private susP: PingPong;
  private depK: PingPong;
  private depS: PingPong;
  private cap: PingPong;
  private dried: PingPong;
  private blurW: Target;
  private paper: Target;

  private progBlur: Program;
  private progFlux: Program;
  private progWater: Program;
  private progMove: Program;
  private progTransferSus: Program;
  private progTransferDep: Program;
  private progCapillary: Program;
  private progDryFold: Program;
  private progDryClearSus: Program;
  private progDryClearDep: Program;
  private progSplat: Program;
  private progSalt: Program;
  private progLift: Program;
  private progRender: Program;
  private progXray: Program;

  private mrtFbo: WebGLFramebuffer;
  private splatVao: WebGLVertexArrayObject;
  private splatInstanceBuf: WebGLBuffer;
  private splatData = new Float32Array(MAX_STAMPS * 8);

  private undoRing: Target[][] = [];
  private undoLive: number[] = [];
  private undoCursor = 0;

  private fastDrySteps = 0;
  /** Ambient mode: evaporation off, the sheet never dries. */
  foreverWet = false;
  paperKind: PaperKind = "cold-press";

  /* Source: src/engine/simulation.ts:173-231 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ constructor(canvas: HTMLCanvasElement, simWidth: number, simHeight: number) {
    this.simWidth = simWidth;
    this.simHeight = simHeight;
    this.ctx = createGL(canvas);
    const { gl } = this.ctx;
    this.vao = createFullscreenVAO(this.ctx);

    this.flow = createPingPong(this.ctx, simWidth, simHeight);
    this.flux = createPingPong(this.ctx, simWidth, simHeight);
    this.susK = createPingPong(this.ctx, simWidth, simHeight);
    this.susS = createPingPong(this.ctx, simWidth, simHeight);
    this.susP = createPingPong(this.ctx, simWidth, simHeight);
    this.depK = createPingPong(this.ctx, simWidth, simHeight);
    this.depS = createPingPong(this.ctx, simWidth, simHeight);
    this.cap = createPingPong(this.ctx, simWidth, simHeight);
    this.dried = createPingPong(this.ctx, simWidth, simHeight);
    this.blurW = createTarget(this.ctx, simWidth, simHeight);
    this.paper = createTarget(this.ctx, simWidth, simHeight);

    this.progBlur = createProgram(this.ctx, BLUR_WET_FRAG);
    this.progFlux = createProgram(this.ctx, FLUX_FRAG);
    this.progWater = createProgram(this.ctx, WATER_FRAG);
    this.progMove = createProgram(this.ctx, MOVE_PIGMENT_FRAG);
    this.progTransferSus = createProgram(this.ctx, TRANSFER_SUS_FRAG);
    this.progTransferDep = createProgram(this.ctx, TRANSFER_DEP_FRAG);
    this.progCapillary = createProgram(this.ctx, CAPILLARY_FRAG);
    this.progDryFold = createProgram(this.ctx, DRY_FOLD_FRAG);
    this.progDryClearSus = createProgram(this.ctx, DRY_CLEAR_SUS_FRAG);
    this.progDryClearDep = createProgram(this.ctx, DRY_CLEAR_DEP_FRAG);
    this.progSplat = createProgram(this.ctx, SPLAT_FRAG, SPLAT_VERT);
    this.progSalt = createProgram(this.ctx, SALT_FRAG, SPLAT_VERT);
    this.progLift = createProgram(this.ctx, LIFT_FRAG, SPLAT_VERT);
    this.progRender = createProgram(this.ctx, RENDER_FRAG);
    this.progXray = createProgram(this.ctx, XRAY_FRAG);

    this.mrtFbo = gl.createFramebuffer()!;

    // Instanced splat geometry: a unit quad, plus two vec4s per stamp.
    this.splatVao = gl.createVertexArray()!;
    gl.bindVertexArray(this.splatVao);
    const quad = gl.createBuffer()!;
    gl.bindBuffer(gl.ARRAY_BUFFER, quad);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]), gl.STATIC_DRAW);
    gl.enableVertexAttribArray(0);
    gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
    this.splatInstanceBuf = gl.createBuffer()!;
    gl.bindBuffer(gl.ARRAY_BUFFER, this.splatInstanceBuf);
    gl.bufferData(gl.ARRAY_BUFFER, this.splatData.byteLength, gl.DYNAMIC_DRAW);
    gl.enableVertexAttribArray(1);
    gl.vertexAttribPointer(1, 4, gl.FLOAT, false, 32, 0);
    gl.vertexAttribDivisor(1, 1);
    gl.enableVertexAttribArray(2);
    gl.vertexAttribPointer(2, 4, gl.FLOAT, false, 32, 16);
    gl.vertexAttribDivisor(2, 1);
    gl.bindVertexArray(null);

    this.setPaper("cold-press", false);
    this.resetState();
  }

  /** Every sheet is its own sheet: a new seed per instance unless one is
   * pinned (the scripted scenes pin theirs so a figure reproduces). */
  paperSeed = 7 + Math.floor(Math.random() * 1e6);

  /** Uploads a fresh sheet. Regenerating the height field is the slow part
   * (~60ms), so paper changes clear the painting with it. */
  /* Source: src/engine/simulation.ts:239-252 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ setPaper(kind: PaperKind, clear = true, seed = this.paperSeed): void {
    const { gl } = this.ctx;
    this.paperKind = kind;
    this.paperSeed = seed;
    const field = generatePaper(this.simWidth, this.simHeight, PAPERS[kind], seed);
    const rgba = new Float32Array(this.simWidth * this.simHeight * 4);
    for (let i = 0; i < this.simWidth * this.simHeight; i++) {
      rgba[i * 4] = field.data[i * 2];
      rgba[i * 4 + 1] = field.data[i * 2 + 1];
    }
    gl.bindTexture(gl.TEXTURE_2D, this.paper.texture);
    gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, this.simWidth, this.simHeight, gl.RGBA, gl.FLOAT, rgba);
    if (clear) this.resetState();
  }

  /* Source: src/engine/simulation.ts:254-264 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ resetState(): void {
    const [pr, pg, pb] = PAPER_REFLECTANCE;
    for (const pair of [this.flow, this.flux, this.susK, this.susS, this.susP, this.depK, this.depS, this.cap]) {
      clearTarget(this.ctx, pair.read);
      clearTarget(this.ctx, pair.write);
    }
    clearTarget(this.ctx, this.dried.read, pr, pg, pb, 0);
    clearTarget(this.ctx, this.dried.write, pr, pg, pb, 0);
    this.undoLive = [];
    this.fastDrySteps = 0;
  }

  // ---------------------------------------------------------------- painting

  /** Blends a batch of stamps into the live water and suspension textures.
   * All stamps in a call carry one pigment (the brush holds one mix). */
  /* Source: src/engine/simulation.ts:270-311 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ splat(stamps: Stamp[], pigment: Pigment | null): void {
    if (stamps.length === 0) return;
    const { gl } = this.ctx;
    const count = Math.min(stamps.length, MAX_STAMPS);
    for (let i = 0; i < count; i++) {
      const s = stamps[i];
      const base = i * 8;
      this.splatData[base] = s.x;
      this.splatData[base + 1] = this.simHeight - s.y; // pointer y is top-down
      this.splatData[base + 2] = s.radius;
      this.splatData[base + 3] = s.water * WATER_SCALE;
      this.splatData[base + 4] = (pigment ? s.pigment : 0) * PIGMENT_SCALE;
      this.splatData[base + 5] = s.dryness;
      this.splatData[base + 6] = i;
      this.splatData[base + 7] = 0;
    }
    gl.bindBuffer(gl.ARRAY_BUFFER, this.splatInstanceBuf);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, this.splatData, 0, count * 8);

    this.bindMRT([this.flow.read, this.susK.read, this.susS.read, this.susP.read]);
    gl.viewport(0, 0, this.simWidth, this.simHeight);
    gl.useProgram(this.progSplat.program);
    gl.uniform2f(this.progSplat.uniforms.uSimSize, this.simWidth, this.simHeight);
    const k = pigment ? pigment.k : [0, 0, 0];
    const s3 = pigment ? pigment.s : [0, 0, 0];
    gl.uniform3f(this.progSplat.uniforms.uPigK, k[0], k[1], k[2]);
    gl.uniform3f(this.progSplat.uniforms.uPigS, s3[0], s3[1], s3[2]);
    gl.uniform3f(
      this.progSplat.uniforms.uPigProps,
      pigment ? pigment.density : 0.4,
      pigment ? pigment.granulation : 0,
      pigment ? pigment.staining : 0.3
    );
    bindTextures(this.ctx, this.progSplat, [["uPaper", this.paper.texture]]);
    gl.bindVertexArray(this.splatVao);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.ONE, gl.ONE);
    gl.drawArraysInstanced(gl.TRIANGLES, 0, 6, count);
    gl.disable(gl.BLEND);
    gl.bindVertexArray(null);
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  }

  /** Scatters salt grains around (x, y). Each grain is a point of osmotic
   * pull and suppressed deposition in the capillary texture; the physics does
   * the rest while the wash dries. */
  /* Source: src/engine/simulation.ts:316-354 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ addSalt(x: number, y: number, radius = 46, seed = Math.floor(Math.random() * 1e9)): void {
    const { gl } = this.ctx;
    let state = seed | 0;
    const rand = /* Source: src/engine/simulation.ts:319-325 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => {
      state = (state + 0x6d2b79f5) | 0;
      let t = state;
      t = Math.imul(t ^ (t >>> 15), t | 1);
      t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
    const grains = 18 + Math.floor(rand() * 12);
    const count = Math.min(grains, MAX_STAMPS);
    for (let i = 0; i < count; i++) {
      const angle = rand() * Math.PI * 2;
      const dist = Math.sqrt(rand()) * radius;
      const base = i * 8;
      this.splatData[base] = x + Math.cos(angle) * dist;
      this.splatData[base + 1] = this.simHeight - (y + Math.sin(angle) * dist);
      this.splatData[base + 2] = 4.0 + rand() * 4.5;
      this.splatData[base + 3] = 0;
      this.splatData[base + 4] = 0.35 + rand() * 0.5; // salt amount
      this.splatData[base + 5] = 0;
      this.splatData[base + 6] = i;
      this.splatData[base + 7] = 0;
    }
    gl.bindBuffer(gl.ARRAY_BUFFER, this.splatInstanceBuf);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, this.splatData, 0, count * 8);
    this.bindMRT([this.cap.read]);
    gl.viewport(0, 0, this.simWidth, this.simHeight);
    gl.useProgram(this.progSalt.program);
    gl.uniform2f(this.progSalt.uniforms.uSimSize, this.simWidth, this.simHeight);
    gl.bindVertexArray(this.splatVao);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.ONE, gl.ONE);
    gl.drawArraysInstanced(gl.TRIANGLES, 0, 6, count);
    gl.disable(gl.BLEND);
    gl.bindVertexArray(null);
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  }

  /** Marks the lift tool's scrub field under a batch of stamps. The water
   * itself arrives through the normal splat; this only disturbs the sizing
   * so the transfer pass re-suspends deposit there. */
  /* Source: src/engine/simulation.ts:359-388 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ addScrub(stamps: Stamp[]): void {
    if (stamps.length === 0) return;
    const { gl } = this.ctx;
    const count = Math.min(stamps.length, MAX_STAMPS);
    for (let i = 0; i < count; i++) {
      const st = stamps[i];
      const base = i * 8;
      this.splatData[base] = st.x;
      this.splatData[base + 1] = this.simHeight - st.y;
      this.splatData[base + 2] = st.radius;
      this.splatData[base + 3] = 0;
      this.splatData[base + 4] = 0.35;
      this.splatData[base + 5] = 0;
      this.splatData[base + 6] = i;
      this.splatData[base + 7] = 0;
    }
    gl.bindBuffer(gl.ARRAY_BUFFER, this.splatInstanceBuf);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, this.splatData, 0, count * 8);
    this.bindMRT([this.cap.read]);
    gl.viewport(0, 0, this.simWidth, this.simHeight);
    gl.useProgram(this.progLift.program);
    gl.uniform2f(this.progLift.uniforms.uSimSize, this.simWidth, this.simHeight);
    gl.bindVertexArray(this.splatVao);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.ONE, gl.ONE);
    gl.drawArraysInstanced(gl.TRIANGLES, 0, 6, count);
    gl.disable(gl.BLEND);
    gl.bindVertexArray(null);
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  }

  /** The hairdryer: fast-forward evaporation over the next several frames,
   * so the drying is watched rather than instantaneous — the edges creep in
   * and the blooms freeze exactly as they would at 1×, just quicker. */
  /* Source: src/engine/simulation.ts:393-395 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ dryFast(): void {
    this.fastDrySteps = 480;
  }

  // -------------------------------------------------------------- simulation

  /** One frame's worth of physics. */
  /* Source: src/engine/simulation.ts:400-405 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ update(): void {
    const steps = this.params.substeps + (this.fastDrySteps > 0 ? 13 : 0);
    const evapMul = this.fastDrySteps > 0 ? 28 : this.foreverWet ? 0 : 1;
    for (let i = 0; i < steps; i++) this.substep(evapMul);
    if (this.fastDrySteps > 0) this.fastDrySteps -= steps;
  }

  /** Deterministic fast-forward, for scripted demos and figures. */
  /* Source: src/engine/simulation.ts:408-410 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ stepMany(steps: number, evapMul = 1): void {
    for (let i = 0; i < steps; i++) this.substep(evapMul);
  }

  /* Source: src/engine/simulation.ts:412-425 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ private bindMRT(targets: Target[]): void {
    const { gl } = this.ctx;
    gl.bindFramebuffer(gl.FRAMEBUFFER, this.mrtFbo);
    const attachments: number[] = [];
    for (let i = 0; i < targets.length; i++) {
      gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0 + i, gl.TEXTURE_2D, targets[i].texture, 0);
      attachments.push(gl.COLOR_ATTACHMENT0 + i);
    }
    // Detach any leftover attachment from a wider previous pass.
    for (let i = targets.length; i < 4; i++) {
      gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0 + i, gl.TEXTURE_2D, null, 0);
    }
    gl.drawBuffers(attachments);
  }

  /* Source: src/engine/simulation.ts:427-444 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ private runPass(program: Program, outputs: Target[], textures: Array<[string, WebGLTexture]>, uniforms?: Record<string, number>): void {
    const { gl } = this.ctx;
    this.bindMRT(outputs);
    gl.viewport(0, 0, this.simWidth, this.simHeight);
    gl.useProgram(program.program);
    gl.uniform2f(program.uniforms.uTexel, 1 / this.simWidth, 1 / this.simHeight);
    bindTextures(this.ctx, program, textures);
    if (uniforms) {
      for (const [name, value] of Object.entries(uniforms)) {
        const loc = program.uniforms[name];
        if (loc) gl.uniform1f(loc, value);
      }
    }
    gl.bindVertexArray(this.vao);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    gl.bindVertexArray(null);
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  }

  /* Source: src/engine/simulation.ts:446-575 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ private substep(evapMul: number): void {
    const p = this.params;

    this.runPass(this.progBlur, [this.blurW], [["uFlow", this.flow.read.texture]]);

    this.runPass(
      this.progFlux,
      [this.flux.write],
      [
        ["uFlow", this.flow.read.texture],
        ["uFlux", this.flux.read.texture],
        ["uPaper", this.paper.texture],
        ["uCap", this.cap.read.texture],
      ],
      { uGravity: p.gravity, uDamp: p.damp, uSlope: p.slope, uTension: p.tension, uSaltPull: p.saltPull }
    );
    this.flux.swap();

    this.runPass(
      this.progWater,
      [this.flow.write],
      [
        ["uFlow", this.flow.read.texture],
        ["uFlux", this.flux.read.texture],
        ["uBlurW", this.blurW.texture],
        ["uCap", this.cap.read.texture],
      ],
      { uEvap: p.evap, uEvapEdge: p.evapEdge, uEvapMul: evapMul, uAbsorb: p.absorb, uVelScale: p.velScale }
    );
    this.flow.swap();

    this.runPass(
      this.progMove,
      [this.susK.write, this.susS.write, this.susP.write],
      [
        ["uFlow", this.flow.read.texture],
        ["uBlurW", this.blurW.texture],
        ["uSusK", this.susK.read.texture],
        ["uSusS", this.susS.read.texture],
        ["uSusP", this.susP.read.texture],
      ],
      { uAdvect: p.advect, uEdgeDrift: p.edgeDrift }
    );
    this.susK.swap();
    this.susS.swap();
    this.susP.swap();

    const transferTextures: Array<[string, WebGLTexture]> = [
      ["uFlow", this.flow.read.texture],
      ["uPaper", this.paper.texture],
      ["uCap", this.cap.read.texture],
      ["uBlurW", this.blurW.texture],
      ["uSusK", this.susK.read.texture],
      ["uSusS", this.susS.read.texture],
      ["uSusP", this.susP.read.texture],
      ["uDepK", this.depK.read.texture],
      ["uDepS", this.depS.read.texture],
    ];
    const transferUniforms = {
      uSettle: p.settle,
      uGranBias: p.granBias,
      uLift: p.lift,
      uSaltLift: p.saltLift,
      uScrubLift: p.scrubLift,
      uEdgeSettle: p.edgeSettle,
    };
    // Both passes read the same pre-swap textures, so the two sides of the
    // exchange agree; only then do the pairs swap.
    this.runPass(this.progTransferDep, [this.depK.write, this.depS.write], transferTextures, transferUniforms);
    this.runPass(this.progTransferSus, [this.susK.write, this.susS.write, this.susP.write], transferTextures, transferUniforms);
    this.depK.swap();
    this.depS.swap();
    this.susK.swap();
    this.susS.swap();
    this.susP.swap();

    this.runPass(
      this.progCapillary,
      [this.cap.write],
      [
        ["uCap", this.cap.read.texture],
        ["uFlow", this.flow.read.texture],
      ],
      { uCapDiff: p.capDiff, uCapDry: p.capDry, uEvapMul: evapMul, uAbsorb: p.absorb }
    );
    this.cap.swap();

    this.runPass(
      this.progDryFold,
      [this.dried.write],
      [
        ["uFlow", this.flow.read.texture],
        ["uSusK", this.susK.read.texture],
        ["uSusS", this.susS.read.texture],
        ["uSusP", this.susP.read.texture],
        ["uDepK", this.depK.read.texture],
        ["uDepS", this.depS.read.texture],
        ["uDried", this.dried.read.texture],
        ["uPaper", this.paper.texture],
      ]
    );
    this.dried.swap();

    this.runPass(
      this.progDryClearSus,
      [this.susK.write, this.susS.write, this.susP.write],
      [
        ["uFlow", this.flow.read.texture],
        ["uSusK", this.susK.read.texture],
        ["uSusS", this.susS.read.texture],
        ["uSusP", this.susP.read.texture],
        ["uDepK", this.depK.read.texture],
      ]
    );
    this.runPass(
      this.progDryClearDep,
      [this.depK.write, this.depS.write],
      [
        ["uFlow", this.flow.read.texture],
        ["uSusK", this.susK.read.texture],
        ["uDepK", this.depK.read.texture],
        ["uDepS", this.depS.read.texture],
      ]
    );
    this.susK.swap();
    this.susS.swap();
    this.susP.swap();
    this.depK.swap();
    this.depS.swap();
  }

  // ----------------------------------------------------------------- render

  /* Source: src/engine/simulation.ts:579-617 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ render(backlight: number, xray: XrayField = 0): void {
    const { gl, canvas } = this.ctx;
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, canvas.width, canvas.height);
    if (xray !== 0) {
      gl.useProgram(this.progXray.program);
      gl.uniform2f(this.progXray.uniforms.uTexel, 1 / this.simWidth, 1 / this.simHeight);
      gl.uniform1i(this.progXray.uniforms.uField, xray);
      gl.uniform1f(this.progXray.uniforms.uScale, XRAY_FIELDS[xray].scale);
      bindTextures(this.ctx, this.progXray, [
        ["uFlow", this.flow.read.texture],
        ["uSusK", this.susK.read.texture],
        ["uDepK", this.depK.read.texture],
        ["uCap", this.cap.read.texture],
        ["uDried", this.dried.read.texture],
      ]);
      gl.bindVertexArray(this.vao);
      gl.drawArrays(gl.TRIANGLES, 0, 3);
      gl.bindVertexArray(null);
      return;
    }
    gl.useProgram(this.progRender.program);
    gl.uniform2f(this.progRender.uniforms.uTexel, 1 / this.simWidth, 1 / this.simHeight);
    gl.uniform1f(this.progRender.uniforms.uBacklight, backlight);
    gl.uniform1f(this.progRender.uniforms.uZoomGrain, this.params.zoomGrain);
    bindTextures(this.ctx, this.progRender, [
      ["uFlow", this.flow.read.texture],
      ["uSusK", this.susK.read.texture],
      ["uSusS", this.susS.read.texture],
      ["uDepK", this.depK.read.texture],
      ["uDepS", this.depS.read.texture],
      ["uDried", this.dried.read.texture],
      ["uPaper", this.paper.texture],
      ["uSusP", this.susP.read.texture],
    ]);
    gl.bindVertexArray(this.vao);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    gl.bindVertexArray(null);
  }

  // ------------------------------------------------------------------- undo

  /* Source: src/engine/simulation.ts:621-632 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ private stateTargets(): Target[] {
    return [
      this.flow.read,
      this.susK.read,
      this.susS.read,
      this.susP.read,
      this.depK.read,
      this.depS.read,
      this.cap.read,
      this.dried.read,
    ];
  }

  /** Called at the start of every stroke (and before salt/clear). Keeps the
   * last few states as GPU-side texture copies — VRAM is the budget, so the
   * ring is shallow, and allocation failure just means less undo. */
  /* Source: src/engine/simulation.ts:637-653 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ snapshot(): void {
    try {
      if (this.undoRing.length < UNDO_DEPTH) {
        while (this.undoRing.length < UNDO_DEPTH) {
          this.undoRing.push(this.stateTargets().map(/* Source: src/engine/simulation.ts:641-641 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ () => createTarget(this.ctx, this.simWidth, this.simHeight)));
        }
      }
    } catch {
      if (this.undoRing.length === 0) return;
    }
    const slot = this.undoCursor % this.undoRing.length;
    const targets = this.stateTargets();
    for (let i = 0; i < targets.length; i++) copyTarget(this.ctx, targets[i], this.undoRing[slot][i]);
    this.undoLive.push(slot);
    if (this.undoLive.length > this.undoRing.length) this.undoLive.shift();
    this.undoCursor++;
  }

  /* Source: src/engine/simulation.ts:655-664 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ undo(): boolean {
    const slot = this.undoLive.pop();
    if (slot === undefined) return false;
    const targets = this.stateTargets();
    for (let i = 0; i < targets.length; i++) copyTarget(this.ctx, this.undoRing[slot][i], targets[i]);
    clearTarget(this.ctx, this.flux.read);
    clearTarget(this.ctx, this.flux.write);
    this.fastDrySteps = 0;
    return true;
  }

  /* Source: src/engine/simulation.ts:666-676 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ clearSheet(): void {
    this.snapshot();
    const [pr, pg, pb] = PAPER_REFLECTANCE;
    for (const pair of [this.flow, this.flux, this.susK, this.susS, this.susP, this.depK, this.depS, this.cap]) {
      clearTarget(this.ctx, pair.read);
      clearTarget(this.ctx, pair.write);
    }
    clearTarget(this.ctx, this.dried.read, pr, pg, pb, 0);
    clearTarget(this.ctx, this.dried.write, pr, pg, pb, 0);
    this.fastDrySteps = 0;
  }

  /** Samples the sheet under a point: how much paint has cured into the
   * dried substrate, how much is still workable (settled or in suspension),
   * and how wet the surface is. A small block average, read back from the GPU,
   * so use it on discrete events (a pointer down), never per frame. */
  /* Source: src/engine/simulation.ts:682-704 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ probe(x: number, y: number, radius = 3): { dried: number; workable: number; wet: number } {
    const { gl } = this.ctx;
    const r = Math.max(1, Math.round(radius));
    const cx = Math.round(x);
    const cy = Math.round(this.simHeight - y); // pointer y is top-down
    const x0 = Math.max(0, cx - r);
    const y0 = Math.max(0, cy - r);
    const w = Math.min(this.simWidth, cx + r + 1) - x0;
    const h = Math.min(this.simHeight, cy + r + 1) - y0;
    if (w <= 0 || h <= 0) return { dried: 0, workable: 0, wet: 0 };
    const buf = new Float32Array(w * h * 4);
    const meanAlpha = /* Source: src/engine/simulation.ts:693-698 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (target: Target): number => {
      if (!this.readFloat(target, x0, y0, w, h, buf)) return 0;
      let sum = 0;
      for (let i = 3; i < buf.length; i += 4) sum += buf[i];
      return sum / (w * h);
    };
    const dried = meanAlpha(this.dried.read);
    const workable = meanAlpha(this.depK.read) + meanAlpha(this.susK.read);
    const wet = meanAlpha(this.flow.read);
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    return { dried, workable, wet };
  }

  /** Reads the dried layer back for persistence. Returns null when the
   * implementation cannot read float pixels (rare on WebGL2 hardware). */
  /* Source: src/engine/simulation.ts:708-714 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ serializeDried(): Float32Array | null {
    const { gl } = this.ctx;
    const out = new Float32Array(this.simWidth * this.simHeight * 4);
    const okay = this.readFloat(this.dried.read, 0, 0, this.simWidth, this.simHeight, out);
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    return okay ? out : null;
  }

  /** readPixels(RGBA, FLOAT) from a float target. WebGL2 guarantees this
   * combination for float-type color buffers even when the implementation
   * advertises HALF_FLOAT as its preferred read type for RGBA16F (ANGLE on
   * Metal does), so trust the call and check the error, not the parameter. */
  /* Source: src/engine/simulation.ts:720-728 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ private readFloat(target: Target, x: number, y: number, w: number, h: number, out: Float32Array): boolean {
    const { gl } = this.ctx;
    gl.bindFramebuffer(gl.FRAMEBUFFER, target.fbo);
    while (gl.getError() !== gl.NO_ERROR) {
      /* drain stale errors so the check below is ours */
    }
    gl.readPixels(x, y, w, h, gl.RGBA, gl.FLOAT, out);
    return gl.getError() === gl.NO_ERROR;
  }

  /** Restores a previously serialized dried layer onto a fresh sheet. */
  /* Source: src/engine/simulation.ts:731-738 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ restoreDried(data: Float32Array): void {
    if (data.length !== this.simWidth * this.simHeight * 4) return;
    const { gl } = this.ctx;
    this.resetState();
    gl.bindTexture(gl.TEXTURE_2D, this.dried.read.texture);
    gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, this.simWidth, this.simHeight, gl.RGBA, gl.FLOAT, data);
    copyTarget(this.ctx, this.dried.read, this.dried.write);
  }
}
