import { spawn } from "node:child_process";
import { setTimeout as delay } from "node:timers/promises";
import { chromium } from "playwright-core";
import type { Browser } from "playwright-core";
import type { Config } from "./types.ts";

export interface Capture {
  finished: Promise<void>;
  stop(): Promise<void>;
}

export function ffmpegArgs(config: Config, destination: string): string[] {
  const s = config.streaming;
  return ["-hide_banner", "-loglevel", "error", "-nostats", "-y",
    "-progress", "pipe:1", "-probesize", "32", "-analyzeduration", "0", "-fpsprobesize", "0",
    "-f", "image2pipe", "-framerate", String(s.fps), "-c:v", "mjpeg", "-i", "pipe:0",
    "-f", "lavfi", "-i", "anullsrc=r=48000:cl=stereo",
    "-c:v", "libx264", "-preset", "ultrafast", "-threads", "2", "-pix_fmt", "yuv420p",
    "-s", `${s.width}x${s.height}`, "-b:v", `${s.videoKbps}k`, "-maxrate", `${s.videoKbps}k`,
    "-bufsize", `${s.videoKbps * 2}k`, "-g", String(s.fps), "-bf", "0",
    "-c:a", "aac", "-b:a", `${s.audioKbps}k`, ...(s.maxSeconds == null ? [] : ["-t", String(s.maxSeconds)]), "-f", "flv", destination];
}

export async function capturePage(config: Config, viewerUrl: string, destination: string, signal: AbortSignal): Promise<Capture> {
  let browser: Browser | undefined;
  let child: ReturnType<typeof spawn> | undefined;
  let closing: Promise<void> | undefined;
  const closed = Promise.withResolvers<void>();
  const stop = (): Promise<void> => closing ??= (async () => {
    child?.stdin?.destroy();
    if (child && child.exitCode === null && child.signalCode === null) {
      child.kill("SIGTERM");
      const force = setTimeout(() => child?.kill("SIGKILL"), 1000);
      await closed.promise;
      clearTimeout(force);
    }
    await browser?.close().catch(() => {});
  })();
  const abort = () => { void stop(); };
  signal.addEventListener("abort", abort, { once: true });
  try {
    signal.throwIfAborted();
    browser = await chromium.launch({ executablePath: config.streaming.chromiumPath, headless: true,
      timeout: 10000, args: ["--disable-dev-shm-usage"] });
    if (signal.aborted) { await browser.close(); signal.throwIfAborted(); }
    const page = await browser.newPage({ viewport: { width: config.streaming.width, height: config.streaming.height } });
    const response = await page.goto(viewerUrl, { waitUntil: "load", timeout: 10000 });
    if (!response?.ok()) throw new Error("viewer_unavailable");
    signal.throwIfAborted();
    const first = await page.screenshot({ type: "jpeg", quality: 80, timeout: 3000 });
    signal.throwIfAborted();
    child = spawn("ffmpeg", ffmpegArgs(config, destination), { stdio: ["pipe", "pipe", "ignore"] });
    const ready = Promise.withResolvers<void>();
    child.stdout?.on("data", (data: Buffer) => {
      if (/frame=\s*[1-9][0-9]*/u.test(data.toString())) ready.resolve();
    });
    const exited = Promise.withResolvers<void>();
    child.once("error", () => { closed.resolve(); exited.reject(new Error("encoder_failed")); });
    child.once("close", code => {
      closed.resolve();
      if (signal.aborted || closing) exited.resolve();
      else exited.reject(new Error(code === 0 ? "encoder_ended" : "encoder_failed"));
    });
    child.stdin?.on("error", () => {});
    const input = child.stdin!;
    const write = (frame: Buffer) => new Promise<void>((resolve, reject) => {
      input.write(frame, error => error ? reject(new Error("encoder_input_failed")) : resolve());
    });
    const frames = (async () => {
      let frame = first;
      while (!signal.aborted && !closing) {
        const started = performance.now();
        await write(frame);
        frame = await page.screenshot({ type: "jpeg", quality: 80, timeout: 3000 });
        await delay(Math.max(0, 1000 / config.streaming.fps - (performance.now() - started)), undefined, { signal });
      }
    })();
    const finished = Promise.race([frames, exited.promise]).catch(() => { throw new Error("capture_ended"); });
    void finished.catch(() => {});
    await Promise.race([ready.promise, finished]);
    return { finished, async stop() {
      signal.removeEventListener("abort", abort);
      await stop();
      await frames.catch(() => {});
    } };
  } catch {
    signal.removeEventListener("abort", abort);
    await stop();
    throw new Error("capture_start_failed");
  }
}
