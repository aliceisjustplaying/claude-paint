# External primary references

Accessed October 4, 2026. The evidence for the project's implementation is in EVIDENCE.md, not these references. No reference below calibrates this engine's rag coefficients.

**S1 — Robyn Palescandolo / R Palesca Fine Art, “How to Create a Wipe-Out Underpainting in Oil,” September 23, 2019.**
`https://rpalescafineart.com/blogs/in-the-studio/how-to-create-a-wipe-out-underpainting-in-oil`
Artist's own process: thin, smoothed raw-umber underpainting; wiping lights with cloth; solvent to strengthen lights, followed by a dry part; deliberately soft, general handling. Supports a qualitative test sequence, not film thicknesses, forces, cloth uptake rates, or a claim about every eighteenth-century painter. Its historical-origin aside is not used as evidence.

**S2 — Gamblin Artists Colors, “Sizes and Grounds.”**
`https://gamblincolors.com/sizes-and-grounds/`
Manufacturer's account of its grounds: oil ground is less absorbent than acrylic gesso and permits wiping back toward white. Supports making the ground part of the test specification. Does not establish that every historical oil ground is identical or completely nonabsorbent.

**S3 — George Field / Thomas W. Salter, _Field's Chromatography_, revised edition, section 155, Burnt Sienna; Project Gutenberg transcription.**
`https://www.gutenberg.org/files/20915/20915-h/20915-h.htm`
Historical qualitative description: burnt sienna is deeper and more transparent than the raw earth. It does not supply K/S values, film thicknesses, a modern reflectometry protocol, or a luminance-difference threshold.

**S4 — ASTM International, ASTM D2805-11(2023), “Standard Test Method for Hiding Power of Paints by Reflectometry,” public scope and significance page.**
`https://store.astm.org/d2805-11r23.html`
The public scope concerns air-dry coatings with Y tristimulus values above 15%. We did not obtain or claim compliance with the full paid standard. An RGB wet-paint diagnostic is not an ASTM-certified test. The numerical ratio remains useful outside a certification claim, but should be labeled as such.

**S5 — Miles Macklin, Matthias Müller, Nuttapong Chentanez, “XPBD: Position-Based Simulation of Compliant Constrained Dynamics,” 2016.**
`https://mmacklin.com/xpbd.pdf`
Original authors' paper; first and third PDF pages inspected. Addresses timestep/iteration-dependent stiffness and supplies constraint-force estimates. This is a numerical technique, not a calibrated cloth/paint model. Finite solver convergence, chosen compliance, boundary conditions, and friction still need testing.

**S6 — The Cargo Book, “Profiles,” official Rust documentation.**
`https://doc.rust-lang.org/cargo/reference/profiles.html`
Explains incremental compilation and code-generation-unit tradeoffs. The project already has an `iter` profile; the proposed improvement is to use it deliberately and remove source recompilation from coefficient experiments, not to rediscover that feature.

**S7 — Gamblin Artists Colors, “Gamsol.”**
`https://gamblincolors.com/oil-painting/gamsol/`
Manufacturer's handling guidance: solvent changes paint consistency; excessive thinning can compromise the paint film. Supports separating solvent from permanent paint rather than treating it as an arbitrary transparency slider. It does not calibrate the project's 6 µm ceiling or 2-minute evaporation constant.

Physical reference work should use modern safe materials, follow their safety data and oily-rag handling instructions, and avoid recreating historic toxic pigment recipes. No physical painting experiment was performed for this review.
