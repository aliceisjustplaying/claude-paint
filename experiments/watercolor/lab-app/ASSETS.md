# Attribution

The source, styles, index page and assets were copied from Watercolor Lab by Ryan McCarty, pinned at `a19bfa2a026856a98ceb400a8eec41f091f04a11`. The MIT notice is retained in `LICENSE` and in each copied source file. Function comments cite the original file and line range, including callbacks and GLSL functions. JSON build metadata retains its MIT license and author fields; its upstream line references are `package.json:1-38`, `package-lock.json` and `tsconfig.json:1-30` at that same commit.

Binary assets cannot carry source comments. `public/fonts/figtree-normal.woff2`, `public/fonts/figtree-italic.woff2` and `public/opengraph.png` are copied unchanged from those exact upstream paths at the pinned commit. `public/robots.txt` and `public/sitemap.xml` are also copied unchanged. The full upstream MIT license applies to the copied material.

The GPU simulator remains at `src/engine/simulation.ts`. The scripted browser API remains `window.__wash`. The added CPU WASM adapter uses the original GLSL only for display; every simulation pass runs in Rust on the CPU. Both options use the same display shader, sheet settings and simulation resolution. Switching clears the sheet because wet-state transfer between engines is not implemented.
