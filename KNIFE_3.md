# Knife round 3 developer experiment

Default-off, branched from `capy/knife-2`. Build with `--features knife-3` and explicitly select `knife{width=30, model="knife-3", flex=0.65}`. Knife-1, knife-2 and ordinary knives remain independently selectable on their existing paths. The engine number and canvas checkpoint format are unchanged.

The crossed-slab defect came from scrape-back using raw per-pixel set height, even though body laying already used a physical contact footprint. Its cut left thread-scale paint remnants over set blue. Knife-3 extends the 1.2 mm support footprint to scrape contact. The actual set height, color and film remain protected; the footprint changes the blade's material contact.

The eight local paint/thinner faces from knife-2 remain authoritative, including their latent mixtures, properties and cure. Clone rollback and debug digests retain the selected third model and face inventory. No painter controls are added. Fresh fluid veils retain the earlier pooled path.

## Reproduce

```sh
CARGO_TARGET_DIR=<private-target> cargo test --workspace --lib --bins --features knife-3 knife
CARGO_TARGET_DIR=<private-target> cargo run --profile test -p paint --features knife-3 --example knife_probe -- <probe-directory>
uv run --script scripts/knife_probe_sheet.py <probe-directory>
CARGO_TARGET_DIR=<private-target> cargo run --profile test -p paint --features knife-3 --example knife_demo -- <demo-directory>
uv run --script scripts/knife_demo_sheet.py <demo-directory>
```

The probe compares baseline, knife-1, knife-2 and knife-3 from identical initial states. The demo uses the unchanged six-pass study and seeds for baseline, knife-2 and knife-3. Each model is simulated once and all its views read that final state. The sheet script compares baseline to the newest model by default; `--reference knife-2` gives a direct round-2 comparison from the same native views. Details are unscaled Gallery crops. The demo is for Alice, never painter guidance.

This remains a bounded material-contact experiment. It is not a metal beam solver or continuous wet-body model. The pressure/contact scales, wider painting behavior and spatial/time convergence need validation before promotion.
