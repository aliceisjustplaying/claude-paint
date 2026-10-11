# Knife round 4 developer experiment

Default-off on `capy/knife-4`, based on `75e7121`. Build with `--features knife-4` and select `knife{width=30, model="knife-4", flex=0.65}`. Earlier explicit models and ordinary knives retain their paths. The engine and canvas checkpoint format are unchanged.

Scrape contact ramps on and off at varying physical distances across the blade. Heel/edge contact changes along the pass, with bounded path and angle wobble. The actual set support stays protected; its physical contact footprint still avoids the per-pixel weave cutoff. Body pressure crests occur in localized patches, so most of the face stays smooth. Adjacent wet blade faces exchange small conservative portions during body movement, carrying their paint/thinner, latent mixtures, properties and cure together. This gives lateral color memory a bounded smear instead of eight isolated color stripes. Fresh fluid veils retain the earlier path.

All noise is counter-based on stroke and physical position. No image noise, final-height overlay or new painter controls are added. The face inventories remain authoritative; clone rollback and debug digests include the fourth-model choice.

Run the focused knife tests with `cargo test --workspace --lib --bins --features knife-4 knife`. The existing `knife_probe` and `knife_demo` examples render all enabled models once per state. Their sheet scripts label the new model; `knife_demo_sheet.py --reference knife-3` compares the two rounds from native views. This remains Alice-only engine evidence. Broader paintings, Mac validation and convergence remain uncertified.
