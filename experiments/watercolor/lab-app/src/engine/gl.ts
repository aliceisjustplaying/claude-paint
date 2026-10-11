/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Copied from uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
 * Full license: LICENSE. */
/**
 * Thin WebGL2 plumbing: context, programs, float render targets, ping-pong
 * pairs, and a fullscreen triangle. No abstraction beyond what the simulator
 * actually calls — the passes in simulation.ts read better against a small
 * flat API than against a framework.
 */

export interface GL {
  gl: WebGL2RenderingContext;
  canvas: HTMLCanvasElement;
}

/* Source: src/engine/gl.ts:13-30 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function createGL(canvas: HTMLCanvasElement): GL {
  const gl = canvas.getContext("webgl2", {
    alpha: false,
    antialias: false,
    depth: false,
    stencil: false,
    // The paint must survive occasional missed frames in screenshots and
    // when the tab returns; the sim redraws every frame anyway.
    preserveDrawingBuffer: true,
  });
  if (!gl) throw new Error("WebGL2 is not available in this browser.");
  // Required to render into RGBA16F targets. Universal on 2020+ hardware.
  if (!gl.getExtension("EXT_color_buffer_float")) {
    throw new Error("EXT_color_buffer_float is not available; the simulation needs float render targets.");
  }
  gl.getExtension("OES_texture_float_linear");
  return { gl, canvas };
}

const VERT_FULLSCREEN = `#version 300 es
layout(location = 0) in vec2 aPos;
out vec2 vUV;
// Source: src/engine/gl.ts:35-38 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
void main() {
  vUV = aPos * 0.5 + 0.5;
  gl_Position = vec4(aPos, 0.0, 1.0);
}
`;

/* Source: src/engine/gl.ts:41-52 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function createFullscreenVAO({ gl }: GL): WebGLVertexArrayObject {
  const vao = gl.createVertexArray()!;
  gl.bindVertexArray(vao);
  const buf = gl.createBuffer()!;
  gl.bindBuffer(gl.ARRAY_BUFFER, buf);
  // One oversized triangle instead of a quad: no diagonal seam, one draw.
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3]), gl.STATIC_DRAW);
  gl.enableVertexAttribArray(0);
  gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
  gl.bindVertexArray(null);
  return vao;
}

export interface Program {
  program: WebGLProgram;
  uniforms: Record<string, WebGLUniformLocation | null>;
}

/* Source: src/engine/gl.ts:59-88 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function createProgram({ gl }: GL, fragSource: string, vertSource = VERT_FULLSCREEN): Program {
  const compile = /* Source: src/engine/gl.ts:60-73 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (type: number, source: string): WebGLShader => {
    const shader = gl.createShader(type)!;
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
      const log = gl.getShaderInfoLog(shader);
      const numbered = source
        .split("\n")
        .map(/* Source: src/engine/gl.ts:68-68 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (line, i) => `${i + 1}: ${line}`)
        .join("\n");
      throw new Error(`Shader compile failed:\n${log}\n${numbered}`);
    }
    return shader;
  };
  const program = gl.createProgram()!;
  gl.attachShader(program, compile(gl.VERTEX_SHADER, vertSource));
  gl.attachShader(program, compile(gl.FRAGMENT_SHADER, fragSource));
  gl.linkProgram(program);
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    throw new Error(`Program link failed: ${gl.getProgramInfoLog(program)}`);
  }
  const uniforms: Record<string, WebGLUniformLocation | null> = {};
  const count = gl.getProgramParameter(program, gl.ACTIVE_UNIFORMS) as number;
  for (let i = 0; i < count; i++) {
    const info = gl.getActiveUniform(program, i);
    if (info) uniforms[info.name.replace("[0]", "")] = gl.getUniformLocation(program, info.name);
  }
  return { program, uniforms };
}

export interface Target {
  texture: WebGLTexture;
  fbo: WebGLFramebuffer;
  width: number;
  height: number;
}

/* Source: src/engine/gl.ts:97-110 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function createTarget({ gl }: GL, width: number, height: number, filter: number = gl.LINEAR): Target {
  const texture = gl.createTexture()!;
  gl.bindTexture(gl.TEXTURE_2D, texture);
  gl.texStorage2D(gl.TEXTURE_2D, 1, gl.RGBA16F, width, height);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, filter);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, filter);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  const fbo = gl.createFramebuffer()!;
  gl.bindFramebuffer(gl.FRAMEBUFFER, fbo);
  gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0);
  gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  return { texture, fbo, width, height };
}

/** A read/write texture pair for iterative passes. */
export interface PingPong {
  read: Target;
  write: Target;
  swap(): void;
}

/* Source: src/engine/gl.ts:119-130 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function createPingPong(ctx: GL, width: number, height: number): PingPong {
  const pair = {
    read: createTarget(ctx, width, height),
    write: createTarget(ctx, width, height),
    /* Source: src/engine/gl.ts:123-127 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ swap() {
      const t = pair.read;
      pair.read = pair.write;
      pair.write = t;
    },
  };
  return pair;
}

/** A framebuffer with several color attachments, for the passes that write
 * more than one state texture in a single deterministic step. */
/* Source: src/engine/gl.ts:134-148 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function createMRT({ gl }: GL, targets: Target[]): WebGLFramebuffer {
  const fbo = gl.createFramebuffer()!;
  gl.bindFramebuffer(gl.FRAMEBUFFER, fbo);
  const attachments: number[] = [];
  targets.forEach(/* Source: src/engine/gl.ts:138-141 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ (t, i) => {
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0 + i, gl.TEXTURE_2D, t.texture, 0);
    attachments.push(gl.COLOR_ATTACHMENT0 + i);
  });
  gl.drawBuffers(attachments);
  if (gl.checkFramebufferStatus(gl.FRAMEBUFFER) !== gl.FRAMEBUFFER_COMPLETE) {
    throw new Error("MRT framebuffer incomplete");
  }
  gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  return fbo;
}

/* Source: src/engine/gl.ts:150-157 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function bindTextures({ gl }: GL, program: Program, bindings: Array<[string, WebGLTexture]>): void {
  bindings.forEach(/* Source: src/engine/gl.ts:151-156 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ ([name, texture], unit) => {
    gl.activeTexture(gl.TEXTURE0 + unit);
    gl.bindTexture(gl.TEXTURE_2D, texture);
    const loc = program.uniforms[name];
    if (loc) gl.uniform1i(loc, unit);
  });
}

/** Copies one state texture into another of the same size (undo snapshots). */
/* Source: src/engine/gl.ts:160-166 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function copyTarget({ gl }: GL, from: Target, to: Target): void {
  gl.bindFramebuffer(gl.READ_FRAMEBUFFER, from.fbo);
  gl.bindFramebuffer(gl.DRAW_FRAMEBUFFER, to.fbo);
  gl.blitFramebuffer(0, 0, from.width, from.height, 0, 0, to.width, to.height, gl.COLOR_BUFFER_BIT, gl.NEAREST);
  gl.bindFramebuffer(gl.READ_FRAMEBUFFER, null);
  gl.bindFramebuffer(gl.DRAW_FRAMEBUFFER, null);
}

/* Source: src/engine/gl.ts:168-174 at a19bfa2a026856a98ceb400a8eec41f091f04a11. */ export function clearTarget({ gl }: GL, target: Target, r = 0, g = 0, b = 0, a = 0): void {
  gl.bindFramebuffer(gl.FRAMEBUFFER, target.fbo);
  gl.viewport(0, 0, target.width, target.height);
  gl.clearColor(r, g, b, a);
  gl.clear(gl.COLOR_BUFFER_BIT);
  gl.bindFramebuffer(gl.FRAMEBUFFER, null);
}
