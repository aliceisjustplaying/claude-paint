# Oil-film constitutive and knife-contact laws

Implementation: `crates/paint/src/oil/{constitutive,knife}.rs`, proposed material-law revision 1. These are local laws and accuracy diagnostics, not a fitted artist-oil model. No builds or tests ran on the implementation machine. The parent owns module registration, integrated checks, species transfers and mechanical evolution. No painter controls or stage labels are added.

## Evidence and its limits

Read Captain Drive `reports/STIL-312.md`, `handoffs/STIL-322/{calibration,mechanics,storage}-audit.md` and the relevant source entries in `handoffs/STIL-312/sources.md`. The following numeric experimental interpretation is inherited from those original-paper readings. Ranquet and Janas article introductions were also fetched here; the fetches were truncated before their numerical results, so they are not an independent recheck of the tables or figure labels.

**Ranquet et al.**, “A holistic view on the role of egg yolk in Old Masters’ oil paints,” *Nature Communications* 14, 1534 (2023), https://doi.org/10.1038/s41467-023-36859-5, full text https://pmc.ncbi.nlm.nih.gov/articles/PMC10050151/ . The handoff records original Fig.7b, “Wrinkling” and Methods. These are **1.0 mm lead-white/cold-pressed-linseed films**, not the separate 0.4 mm drying specimens. A, ordinary oil paint with 32 vol% pigment, has fresh bulk shear yield **75 ±10 Pa** and wrinkles. B, total solids 16 vol% including 2 vol% egg-yolk solids, has **58 ±4 Pa** and wrinkles. C, total solids 32 vol% including 4 vol% egg-yolk solids, has **3200 ±400 Pa** and suppressed wrinkles. Egg-yolk solids are included in total solids, not added on top. The figure does not establish a universal statistical interpretation of its printed ± values.

These numbers measure **fresh bulk shear yield**, not skin compression, skin modulus or a buckling threshold. Egg also changes oxidation and colloidal structure, so this is not a completely isolated rheological intervention. `measured_fresh_shear_yield_pa` permits reference specimens to carry their actual measured bulk yield while leaving their distinct chemistry to its own coefficients. It does not expose egg as a painter API. Changing that override alone is a matched numerical control, not a claim to reconstruct all of specimen C.

Ranquet's explanation involves oxidation-driven volume increase, a skin and a mobile interior. The experiment does **not** supply young-skin modulus, post-gel eigenstrain, relaxation time, cohesive strength, adhesion energy or knife force. Gravimetric oxygen uptake is not measured linear expansion. Its pigment series does not establish a universal critical pigment concentration.

**Janas et al.**, “Shrinkage and mechanical properties of drying oil paints,” *Heritage Science* 10 (2022), https://doi.org/10.1186/s40494-022-00814-2, https://www.nature.com/articles/s40494-022-00814-2 . The inherited reading of Tables 1–2 identifies 31-year lead-white films at PVC 44%, current thickness 130 µm, with Young's moduli 970 and 1080 MPa. These are **decades-old tensile films**, not a days-old skin. No 1 GPa value is transplanted into this law. The reported cumulative shrinkage is a long-term, curvature/mechanics-derived quantity, not a young-film height loss or an oxygen-to-volume conversion. This first-weeks law does not model decades of degradation.

**Pizzimenti et al.**, “Oxidation and Cross-Linking in the Curing of Air-Drying Artists’ Oil Paints,” *ACS Applied Polymer Materials* 3 (2021), 1912–1922, https://doi.org/10.1021/acsapm.0c01441, and **Sarah Sands**, “Weighing In on the Drying of Oils,” https://justpaint.org/weighing-in-on-the-drying-of-oils/ . Their source limitations and calendar fixtures are recorded in `oil-film-calibration.md`. Contact endpoints do not measure any mechanical coefficient below. Golden's stand-oil film can be touch-dry yet remain soft and tacky much later; no calendar stage supplies a modulus or an uncuttable flag.

The mechanics audit derives A/B/D integration and stress-free network joining. It calls for a coupled finite-depth yielding-flow solve rather than the original prototype's `N/H` gate. This module supplies coefficients to that solver and compares its supplied threshold diagnostics. It neither synthesizes wrinkles nor supplies a second instability/yield criterion. Source S6's aged-overpaint mechanical removal with solvent is not a calibration of young-film knife force. No direct young-impasto cutting-force data were found in the supplied record.

## Material evaluation

`evaluate(&Cell, &Formulation, &Parameters) -> Result<Material, Error>` reads actual reference pigment, binder and solvent inventories. It never reads legacy stiffness, elapsed time or a working-stage label. `Formulation::binders()` is normalized in the order **linseed, safflower, walnut, poppy, stand**. The current pigment formulation's packed-oil and tube-oil properties are used only as a packing descriptor, not as current oil inventory. Their accuracy is limited by their existing per-pigment measured/estimated provenance in `pigment_oil.md`.

With total reference thickness h, pigment fraction p, solvent fraction s and nonvolatile binder fraction b:

```text
p = pigment/h; s = solvent/h; b = binder/(pigment+binder)
t = clamp((network - gel_network)/(1 - gel_network), 0, 1)
c = b t²(3 - 2t)
R = 1 + 2.5p + 8p²
L = exp(-6s)
packing = (1 - tube_oil)/(1 - tube_oil + tube_oil packed_oil)
P = 1 + p(1 - packing)

E = young_network_pa c network_factor R L
tau_fresh = fresh_yield_scale_pa (p/.32)² exp(4(p-.32)) oil_viscosity P L
tau_y = tau_fresh + network_yield_pa c network_factor
eta = plastic_viscosity_pa_s oil_viscosity (1+20p²) (1+100c²) L
tau_relax = relaxation_seconds (1+20c)
cohesive_strength = cohesive_strength_pa c network_factor
Gc = fracture_energy_j_m2 c network_factor
```

The measured-yield override replaces only `tau_fresh`, including its modeled composition factors. Its value is appropriate only for that named specimen's fresh composition. Reusing it after dilution is not a measured dilution law. Solvent dilutes and softens the proposed response; pigment loading affects bulk yield, viscosity and network reinforcement. Extra oil changes those quantities even if legacy stiffness stays fixed. Total thickness changes integrated rigidity and work, not local material constants at fixed composition. Pigment-specific chemistry already acts through retained oxidation/network and the recipe's kinetics; optical pigment IDs do not become arbitrary modulus IDs.

The oil-addition unit fixture mixes fresh, unoxidized/unnetworked binder into **pre-gel mobile** paint. Doubling binder halves the binder-weighted oxidation/network fractions and averages the old inhibitor reservoir with fresh inhibitor=1, preserving the original reacted/networked binder. Its suspension yield decreases under this proposed law. The separate equally cured, oil-rich comparison checks only a modulus difference: retaining network=.6 while doubling binder doubles networked-binder inventory, so it is not fresh-oil dilution. The sources do not establish universally decreasing yield for equally cured oil-rich films. Homogenizing fresh oil into an already coherent cell is not authorized by this unit fixture.

`c` and E are continuous with continuous first derivatives at the proposed gel onset. Coherence is a continuous load-bearing fraction, not the fraction of a cell that may be erased or mixed as liquid. An empty cell returns zero resistance and stiffness. With no binder the law creates no coherent binder skin; granular pigment has bulk yield but is not assigned a fictitious elastic oil network. A dry granular/frictional material model is outside this law.

Every coefficient in the following table is **proposed and unmeasured**, except the explicitly identified specimen yield observations above. No sensitivity range is a confidence interval.

| Parameter | Starting value | Status and role |
|---|---:|---|
| gel_network | .2 | Proposed smooth percolation onset; not an ASTM endpoint |
| young_network_pa | 200,000 Pa | Unfitted young-network scale, multiplied by composition and coherence; not a measured skin modulus |
| poisson | .35 | Unfitted isotropic plane-stress assumption |
| fresh_yield_scale_pa | 75 Pa | Seed scale motivated by Ranquet A; the general composition law is not fitted to that paper |
| measured_fresh_shear_yield_pa | None | Optional actual specimen observation, not a skin load |
| network_yield_pa | 2,000 Pa | Proposed network increase in bulk shear yield |
| plastic_viscosity_pa_s | 20 Pa·s | Proposed Bingham plastic viscosity scale, not a measured zero-shear viscosity |
| relaxation_seconds | 3,600 s | Proposed Maxwell scale before the coherence multiplier |
| oxidation_expansion | .02 | Proposed positive linear free strain per oxidation increment in reference binder |
| network_contraction | .005 | Proposed opposing free strain per network increment; not a measured cross-link shrinkage |
| cohesive_strength_pa | 20,000 Pa | Proposed full-network cutting traction scale |
| fracture_energy_j_m2 | .5 J/m² | Proposed full-network fracture energy scale |
| oil viscosity factors | 1, .85, .7, .75, 8 | Proposed relative factors in the binder order above; not measured rankings of particular commercial grades |
| network factors | 1, .9, .8, .8, 1.1 | Proposed composition sensitivity; not identified by calendar endpoints |
| shape coefficients | 2.5, 8, 6, 4, 20, 100, 20 | Proposed suspension, dilution and cure-response shapes in the equations above |

There is no identified Herschel–Bulkley exponent or thixotropic recovery state in the available production storage. The shared rheology is the simpler finite-yield Bingham law `tau=tau_y+eta gamma_dot` when yielded. Its applicability is a modeling limitation, not evidence that real oil paint is Newtonian after yield. A mechanics implementation can use the same `tau_y` and `eta` without importing a separate knife hardness table.

## Stress-free network birth and signed strain

`chemical_increment(old, new, formulation, parameters, dt_seconds)` requires the same pigment/binder inventories and recipe, with nondecreasing oxidation and network. Solvent can decrease without an invented extra chemical contraction; reference geometry already records its loss. Transfers and mixing are separate operations, not chemical increments.

```text
delta_free_strain = b [oxidation_expansion delta_oxidation
                     - network_contraction delta_network]
x = dt/tau_relax_old
retained_stress_fraction = exp(-x)
tension_increment_pa = -E_old/(1-nu_old) delta_free_strain (1-exp(-x))/x
```

The last factor is one at x=0. It is the exact constant-source Maxwell integration **for frozen old stiffness/relaxation**. The caller retains old tensile resultants multiplied by `retained_stress_fraction`, adds the new stress contribution integrated over its old material depth and preserves the old resultant when new modulus joins. A positive restrained free strain gives negative tension, hence compression; a negative free strain gives tension. No past free strain is multiplied by new E. A cell crossing from zero old E can join with positive new E yet contribute **zero birth stress**. Subsequent increments can stress it. The operator split does not resolve stress accumulated within the same timestep by cohorts born during that timestep; chemistry/mechanics time refinement remains necessary.

Free strain is not automatically a vertical inventory change. The mechanical solver owns constrained deformation, volume reconstruction and reference geometry. Different relaxation/strain histories through depth require retained cell/cohort contributions or an explicitly approximate packet reduction; one packet-average relaxation is not exact for arbitrary heterogeneous cells. These local rates must not reset stored membrane force, moment or chemistry on a render, cover or wait.

## Rigidity and adaptive depth accuracy

`integrate_depth(&[DepthCell], z_ref_m)` accepts ordered, nonoverlapping physical slices and integrates their local smooth-in-network E as piecewise constant within each finite volume. It does not invent an unresolved subcell cure front. A slice contributes exactly its within-cell cubic moment:

```text
q = E/(1-nu²)
Q = [[q, nu q, 0], [nu q, q, 0], [0, 0, q(1-nu)/2]]
A += Q h
B += Q h (z_mid-z_ref)
D += Q [h (z_mid-z_ref)² + h³/12]
z_neutral = z_ref + B11/A11
D_neutral_11 = D11 - B11²/A11
```

Matrix order is **xx, yy, engineering xy shear**, with A in N/m, B in N and D in N·m. The neutral scalar is accumulated using a weighted centered second moment, algebraically equivalent to the subtraction above but avoiding cancellation for a thin skin far from the reference plane. The scalar neutral-axis result is the 11-direction diagnostic; if nu varies with depth, there is generally no one scalar neutral axis that eliminates every laminate coupling. The returned full A/B/D is authoritative for the existing coupled solver. Equibiaxial stress uses E/(1-nu), not q. For a uniform slice, the scalar neutral rigidity is exactly `E h³/[12(1-nu²)]`. A one-cell skin does not have zero rigidity.

`integrate_column(&[Cell], &[Arc<Formulation>], &Parameters)` evaluates support-to-air chemistry cells at their actual reference depths, with `z_ref_m` at the geometric midplane. It uses no temporary heap depth vector. Packet-specific integration can call `integrate_depth` on the packet's own slices, or integrate a packet-local cell range and transform to the store's packet reference plane. Coverage remains an area sidecar; this local law does not silently spread a partially covered column over a whole pixel.

`refinement_accuracy(coarse, refined, thresholds: [coarse_value, refined_value], AccuracyFloors)` compares A, B, D and scalar neutral-axis rigidity. It first shifts the refined A/B/D to the coarse reference plane. Extension and bending use max-entry relative errors with caller-supplied positive dimensional floors. Coupling error is normalized by sqrt(extension scale × bending scale), not by a nearly zero B. The neutral-axis scalar has its own bending-relative comparison. The maximum of these errors and the supplied threshold-relative error is exposed as `maximum_relative()`. A separate `threshold_sign_changed` flags a change of stable/unstable classification even near a caller's absolute floor.

The **caller supplies the actual coupled finite-depth solver diagnostic** on each trial column, in the same units and sign convention, with the same physical domain, seed/perturbation, boundary conditions and finite yielding model. It can be a critical load or a signed energy/stability margin; the `threshold` floor has those same units. There is deliberately no generic default value, fake Winkler spring, `N/H` gate or claimed finite positive threshold for every viscous support. A near-zero signed margin calls for the sign check as well as a numerical relative tolerance. Mean cure and even matching A can conceal a materially wrong D. Rigidity acceptance supplements, rather than replaces, conservative species/oxygen and chemistry error checks. The parent decides tolerances and refinement/coarsening; this module never merges recipes or packet interfaces.

## Knife contact and receipts

`knife::reach(&[KnifeCell], Contact)` accepts **air-to-support order**, the reverse of chemistry storage. Each cell carries evaluated shared Material, actual physical thickness, prior cut/interface damage and an optional explicitly modeled `ReleaseInterface`. The function does not mutate any cells or transfer species. It prevalidates the whole input and returns per-cell removed fractions, reached depth, a thickness-weighted whole-column removed fraction and work receipts. Lower cells remain inaccessible until every cell above them has been fully reached. `max_depth_m` is the geometric blade-depth limit, not a pressure-dependent cure cutoff.

Normalized pressure is dimensionless in [0,1]. `ContactCalibration::resolved_stress_at_unit_pressure_pa` defaults to **20,000 Pa**, an explicitly **empirical proposed conversion** from the existing control to equivalent resolved cutting traction. It is not a measured normal contact pressure, force, force per blade width or number of newtons. Blade angle, friction and contact-area calibration are not resolved. The caller supplies available **blade work per actual contacted plan area in J/m²** independently; pressure is not allowed to invent unlimited work. `shear_rate_s` and positive `shear_strain` describe the proposed local shearing exposure. `cut_spacing_m` is a positive **physical cut-face area/volume length**, independent of chemistry grid size, not a pixel count.

For undamaged fraction u=1-damage:

```text
flow_stress = tau_y + eta shear_rate
required_traction = max(flow_stress + u cohesive_strength, u interface_strength)
rheology_work_per_depth = flow_stress shear_strain
fracture_work_per_depth = u Gc / cut_spacing
release_work = u interface_separation_work
```

The blade advances only above the finite required traction and with positive available work. An explicitly supplied release interface consumes its release work before any of that cell can be lifted. Remaining work bounds penetration by `(work-release_work)/(rheology_work_per_depth+fracture_work_per_depth)`, as well as thickness and geometric depth. No release entry means **no adhesion traction or work**. A release entry describes a real interface that the caller's proposed lift path requires to separate; it is not automatically attached to every numerical chemistry cell. Refining a cell must not duplicate a release interface. A cut/peel geometry where the lower interface is not released must omit that entry. This is a local work-limited cutting/lifting approximation, not a solved blade-contact/friction/delamination problem.

Receipts separate rheological dissipation, cohesive fracture work and explicit adhesion work. `spent_work + remaining_work` equals supplied blade work to floating-point rounding. `coherent_removed_fraction` records the removed fraction times the continuous load-bearing fraction; it is **not permission to convert its complement to fresh liquid**. `fragment_damage=1` for a removed coherent slice denotes a fully separated **new cut surface**, not destroyed binder chemistry or bulk rejuvenation. For noncoherent material it retains prior damage. The caller maps cut-surface damage to its mechanical interface/fragment record; it must not overwrite all remaining packet interfaces or set the fragment's network to zero. No retained-layer crack pattern is generated.

Low pressure can fail even with substantial work. Higher pressure can reach setting/tacky material and some touch-dry films, provided work is available. An input material whose cohesive resistance exceeds the proposed unit-pressure calibration remains uncut at pressure=1. The code does not pretend to predict decades-old hardness from the first-weeks law; harder measured/reference material can supply its own Material values. Fractional top-down removal, actual species division, packet N/M transport, debris collection, interface placement and coverage/reference-area handling remain store/Canvas responsibilities.

## Compact verification supplied, not executed here

The added tests are `oil_constitutive_depth_integration_and_refinement`, `oil_constitutive_stress_free_birth_and_signed_strain`, `oil_constitutive_inventory_recipe_and_smoothness`, `oil_knife_pressure_monotonicity_and_finite_set_reach`, `oil_knife_work_receipts_preserve_top_down_order` and `oil_knife_only_explicit_interfaces_cost_adhesion_work`. They cover the analytic uniform slice, neutral-axis/layered contrast at equal A, coupled diagnostic sign, birth/sign, inventory/packing effects, measured-yield override, pressure/work monotonicity, finite coherent reach, an uncuttable hard control, cut damage, no input mutation and work accounting. These unit checks do not establish calibrated knife force or actual Canvas exposure/conservation behavior. Existing tests and tolerances are untouched. The parent owns the build/test run and routine-gate registration.
