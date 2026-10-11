/** MIT License. Copyright (c) 2026 Ryan McCarty.
 * Source: vite.config.ts at a19bfa2a026856a98ceb400a8eec41f091f04a11. Full license: LICENSE. */
import { defineConfig } from "vite";

export default defineConfig({
  base: "./",
  server: {
    port: 5173,
    host: true,
  },
  build: {
    target: "es2022",
    sourcemap: true,
  },
});
