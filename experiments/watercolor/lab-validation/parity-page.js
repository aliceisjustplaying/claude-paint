// MIT License, Copyright (c) 2026 Ryan McCarty.
// Instrumented pinned Watercolor Lab harness: a19bfa2a026856a98ceb400a8eec41f091f04a11.
// Full license: ../THIRD_PARTY_NOTICES.md.
import { Simulation, DEFAULT_PARAMS } from "./original/src/engine/simulation.ts";
import { beginStroke, strokeTo, TOOLS } from "./original/src/paint/brush.ts";
import { PIGMENT_BY_ID } from "./original/src/paint/pigments.ts";

const fields = { flow: "flow", flux: "flux", sus_k: "susK", sus_s: "susS", sus_p: "susP", dep_k: "depK", dep_s: "depS", cap: "cap", dried: "dried", blur_w: "blurW", paper: "paper" };
const passes = [
  ["progBlur", "blur_wet", ["blur_w"]],
  ["progFlux", "flux", ["flux"]],
  ["progWater", "water", ["flow"]],
  ["progMove", "move_pigment", ["sus_k", "sus_s", "sus_p"]],
  ["progTransferDep", "transfer_dep", ["dep_k", "dep_s"]],
  ["progTransferSus", "transfer_sus", ["sus_k", "sus_s", "sus_p"]],
  ["progCapillary", "capillary", ["cap"]],
  ["progDryFold", "dry_fold", ["dried"]],
  ["progDryClearSus", "dry_clear_sus", ["sus_k", "sus_s", "sus_p"]],
  ["progDryClearDep", "dry_clear_dep", ["dep_k", "dep_s"]],
];

// Source: src/engine/simulation.ts:720–727 at the pinned commit.
function read(sim, target) {
  const out = new Float32Array(sim.simWidth * sim.simHeight * 4);
  if (!sim.readFloat(target, 0, 0, sim.simWidth, sim.simHeight, out)) throw new Error("readPixels failed");
  return Array.from({ length: out.length / 4 }, (_, i) => Array.from(out.slice(i * 4, i * 4 + 4)));
}

// Source: src/engine/simulation.ts:621–632 and 427–444 at the pinned commit.
function capture(sim, overrides = {}) {
  const state = {};
  for (const [name, key] of Object.entries(fields)) {
    const value = sim[key];
    state[name] = read(sim, overrides[name] ?? value.read ?? value);
  }
  sim.ctx.gl.bindFramebuffer(sim.ctx.gl.FRAMEBUFFER, null);
  return state;
}

// Source: src/engine/simulation.ts:243–250 and 733–737 at the pinned commit.
function upload(sim, name, pixels) {
  const gl = sim.ctx.gl;
  const value = sim[fields[name]];
  for (const target of value.read ? [value.read, value.write] : [value]) {
    gl.bindTexture(gl.TEXTURE_2D, target.texture);
    gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, sim.simWidth, sim.simHeight, gl.RGBA, gl.FLOAT, new Float32Array(pixels.flat()));
  }
}

window.runParity = () => {
  const width = 16, height = 12;
  const canvas = document.querySelector("canvas");
  const sim = new Simulation(canvas, width, height);
  sim.setPaper("cold-press", true, 71);
  const gl = sim.ctx.gl;
  const debug = gl.getExtension("WEBGL_debug_renderer_info");
  const renderer = debug ? gl.getParameter(debug.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER);
  const base = capture(sim);
  const blank = structuredClone(base);
  for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
    const i = y * width + x;
    const inside = x > 1 && x < width - 2 && y > 1 && y < height - 2;
    const c = inside ? 0.03 + ((x + y * 3) % 7) * 0.007 : 0;
    base.flow[i] = [0, 0, inside ? 0.018 + ((x * 3 + y) % 9) * 0.014 : 0, inside ? 0.6 + ((x + y) % 4) * 0.1 : 0];
    base.sus_k[i] = [c * 1.1, c * 0.9, c * 0.3, c];
    base.sus_s[i] = [c * 0.22, c * 0.25, c * 0.28, 0];
    base.sus_p[i] = [c * 0.65, c * 0.8, c * 0.35, 0];
    base.dep_k[i] = [c * 0.21, c * 0.17, c * 0.06, c * 0.2];
    base.dep_s[i] = [c * 0.044, c * 0.05, c * 0.056, c * 0.07];
    base.cap[i] = [inside ? 0.08 + ((x + y) % 3) * 0.025 : 0, inside && x === 8 ? 0.03 : 0, inside && y === 6 ? 0.02 : 0, 0];
  }
  const result = { upstreamCommit: "a19bfa2a026856a98ceb400a8eec41f091f04a11", width, height, renderer, subpixelBits: gl.getParameter(gl.SUBPIXEL_BITS), format: "RGBA16F", filtering: "LINEAR", wrap: "CLAMP_TO_EDGE", orientation: "GL-bottom-up", cases: [] };
  for (const name of ["wet-transport", "dry-fold", "closed-system", "brush-input"]) {
    sim.params = { ...DEFAULT_PARAMS };
    const input = structuredClone(name === "brush-input" ? blank : base);
    if (name === "dry-fold") for (const pixel of input.flow) { pixel[2] = 0; pixel[3] = 0; }
    if (name === "closed-system") {
      Object.assign(sim.params, { evap: 0, evapEdge: 0, absorb: 0, capDry: 0 });
      for (const pixel of input.flow) { pixel[2] = Math.max(pixel[2], 0.02); pixel[3] = 1; }
      for (const pixel of input.cap) { pixel[1] = 0; pixel[2] = 0; }
    }
    for (const [key, value] of Object.entries(input)) upload(sim, key, value);
    const initial = capture(sim);
    const captures = [];
    const original = sim.runPass;
    sim.runPass = function(program, outputs, textures, uniforms) {
      const input = capture(this);
      original.call(this, program, outputs, textures, uniforms);
      const [, pass, names] = passes.find(([key]) => this[key] === program);
      captures.push({ pass, input, fields: capture(this, Object.fromEntries(names.map((key, i) => [key, outputs[i]]))) });
    };
    let operations = Array.from({ length: name === "dry-fold" ? 1 : 3 }, () => ({ kind: "substep", evap_mul: name === "closed-system" ? 0 : 1 }));
    if (name === "brush-input") {
      operations = [];
      for (const [pigmentId, y] of [["french-ultramarine", 5], ["burnt-sienna", 7]]) {
        const pigment = PIGMENT_BY_ID.get(pigmentId);
        const brush = beginStroke(TOOLS.round, 0.5, 0.7, 0.8, 3, y, 12345, 0.08);
        for (const [x, pressure] of [[6, 0.4], [10, 0.8], [13, 0.55]]) {
          operations.push({ kind: "splat", stamps: strokeTo(brush, x, y, pressure), pigment });
          operations.push({ kind: "substep", evap_mul: 1 });
        }
      }
      operations.push({ kind: "salt", x: 7, y: 6, radius: 2, seed: 424242 });
      operations.push({ kind: "scrub", stamps: [{ x: 9, y: 6, radius: 2, water: 0, pigment: 0, dryness: 0 }] });
      operations.push({ kind: "substep", evap_mul: 1 });
    }
    for (const op of operations) {
      if (op.kind === "substep") sim.substep(op.evap_mul);
      else {
        const input = capture(sim);
        if (op.kind === "splat") sim.splat(op.stamps, op.pigment);
        else if (op.kind === "salt") sim.addSalt(op.x, op.y, op.radius, op.seed);
        else if (op.kind === "scrub") sim.addScrub(op.stamps);
        captures.push({ pass: op.kind, input, fields: capture(sim) });
      }
    }
    sim.runPass = original;
    const params = Object.fromEntries(Object.entries(sim.params).map(([key, value]) => [key.replace(/[A-Z]/g, (c) => `_${c.toLowerCase()}`), value]));
    result.cases.push({ name, params, initial, operations, captures, final: capture(sim) });
  }
  return result;
};
