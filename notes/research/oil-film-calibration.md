# Finite-volume oil chemistry and calibration provenance

Implementation: `crates/paint/src/oil/chemistry.rs`. Material revision 1. This is a coupled depth solver with a reproducible calibration interface, not yet a fitted calendar parameter set. No build, test or fitting execution was performed on the implementation machine, by instruction. Fixture defaults are **proposed starting values**. The source calendar values are not drying schedules and are never consulted by `advance_column`.

## Evidence and limits

Read the Captain's `reports/STIL-312.md`, `handoffs/STIL-312/sources.md`, `handoffs/STIL-312/prototype/main.rs`, its README and `handoffs/STIL-322/calibration-audit.md`. The S2 original page was additionally fetched during implementation. The audit independently read the full S3 paper. These records distinguish experimental observations, derived composition conversions and unfitted constitutive assumptions.

**S3:** Pizzimenti et al., “Oxidation and Cross-Linking in the Curing of Air-Drying Artists’ Oil Paints,” *ACS Applied Polymer Materials* 3 (2021), 1912–1922. DOI: https://doi.org/10.1021/acsapm.0c01441 . Institutional full text: https://arpi.unipi.it/retrieve/e0d6c931-8304-fcf8-e053-d805fe0aa794/acsapm.0c01441.pdf . Locators: p.1913 §§2.2–2.3 and Table 1 for weighed formulations and 0.45 mm wet films on glass; p.1914 Table 2 for calendar endpoints. Numeric calendar temperature, RH, illumination, airflow and uncertainty are unreported. The separate §2.4 thermogravimetry at 80 °C is not the calendar experiment. No extra catalytic drier is listed, which does not mean chemically inert pigments.

| Fixture and provenance code | Pigment / binder | Pigment / binder mass fractions | Wet µm | Set-to-touch h | Dry-to-touch h | Dry-hard h |
|---|---|---|---:|---:|---:|---:|
| S3-LWLO-450 / 1 | Lead white / linseed | .66 / .34 | 450 | 63 | 90 | 134 |
| S3-UBLO-450 / 2 | Synthetic ultramarine / linseed | .61 / .39 | 450 | 213 | 242 | 284 |
| S3-LWSO-450 / 3 | Lead white / safflower | .77 / .23 | 450 | 112 | 157 | 207 |
| S3-UBSO-450 / 4 | Synthetic ultramarine / safflower | .49 / .51 | 450 | 234 | 261 | 305 |

Every number in this table is sourced to those two S3 tables except conversion of 0.45 mm to 450 µm. Reported uncertainty is not available. Each point target remains a point in `CALENDAR_TARGETS`; a fit's numerical residual is not an experimental error bar.

**S2:** Sarah Sands, “Weighing In on the Drying of Oils,” September 1, 2011, editorial correction June 6, 2023: https://justpaint.org/weighing-in-on-the-drying-of-oils/ . Manufacturer full mirror: https://goldenartistcolors.com/inspiration/justpaint/weighing-in-on-the-drying-of-oils . Locators: “Current Testing,” “Basic Graphs”/Fig.1 and “Film Thickness”/Fig.4.

| Fixture and provenance code | Nominal drawdown | Touch-dry constraint | Recipe completeness |
|---|---|---|---|
| S2-TiLO-3mil / 5 | 3 mil = 76.2 µm | By day 2: upper bound 48 h, no lower bound | Partial historical rutile titanium / linseed series |
| S2-TiLO-10mil / 5 | 10 mil = 254 µm | Around days 4–6: approximate interval 96–144 h | Partial, multiple samples averaged in Fig.4 |

The mil conversions use exactly 25.4 µm/mil and are nominal application thicknesses, not measured dry thicknesses. Clear polyester and uncontrolled 70–75 °F, 40–55% RH are reported in Current Testing. Complete pigment:oil fractions, densities and all additives are unknown. Both internal fixtures deliberately share the same proposed material; this is a modeling surrogate, not an assertion that a complete historical recipe was reconstructed. The 2023 correction explicitly prevents treating the current commercial Williamsburg formula as the historical no-drier reference.

S2's “Type of Oil”/Fig.3 describes stand oil touch-dry by the end of week two yet soft and substantially tacky after two months. This rejects identifying touch-dry with final hardness. Its net mass peak is not an expansion measurement. The 0/.005%/.011% cobalt–manganese labels in “Driers”/Fig.8 do not establish elemental-metal concentrations or a dose-response law. No numerical catalyst concentration is inferred from them.

**S13 and observation interpretation:** S3 names ASTM D1640 without its edition. STIL-312 read D1640–95 (1999 reapproval), §§7.2, 7.5–7.7, printed pp.2–3 through an unofficial transcription: https://pdfcoffee.com/astm-d1640pdf-pdf-free.html . Official scope: https://www.astm.org/d1640_d1640m-14r22.html . Set-to-touch tests light fingertip transfer, dry-to-touch adds resistance to light rubbing and dry-hard uses downward thumb pressure without twisting followed by polishing. The separate dry-through test includes a 90° thumb rotation and is **not** S3's dry-hard measurement. No force or modulus was measured by these observations.

**Depth evidence, not transferred numerical coefficients:** STIL-312 sources S4/S5 report alkyd depth-dependent mobility and an oxygen-limited front. S5 DOI https://doi.org/10.1063/1.1886913, pp.134105-1–2, Figs.1–3. Its fitted front slope identifies a transport/solubility/stoichiometry combination, not artist-oil diffusivity alone. The prototype's D₀=2×10⁻¹² m²/s and reaction k=10⁻⁴ s⁻¹ are illustrative. They are not sourced artist-oil material measurements.

## Inventory, units and oxygen convention

`Cell` is `repr(C)` with seven f32 fields and one u32 recipe index, 32 bytes at 4-byte alignment. Cells run **support to air**, unlike the prototype's air-first grid. `pigment_um`, `binder_um` and `solvent_um` are reference volume per area: one µm is one µm³/µm². Chemical evolution never changes pigment or binder inventory. Geometry here is their sum plus remaining solvent, with no oxidative volume strain applied. Mechanics can use its own explicitly proposed specific-volume law; gravimetric gain is not such a law.

An oxygen inventory unit is oxygen mass per area divided by a fixed reference binder density, expressed as an equivalent µm. This is a bookkeeping mass equivalent, not the volume of oxygen gas and not an oil expansion. `oxygen` is dimensionless dissolved **activity** `a`, between 0 and 1. `air_activity=1` is the formulation's equilibrium ambient-oxygen exposure. At activity a, binder and solvent dissolved concentrations are respectively `s_b a` and `s_s a`, with solubilities in oxygen-equivalent mass per reference liquid volume. The storage capacity is

`C_i = binder_um_i s_b,i + solvent_um_i s_s,i`

and dissolved inventory is `M_i=C_i a_i`. Different materials at equilibrium share activity, not concentration: the binder concentration ratio is their solubility ratio. Solvent shares local activity but can have a different solubility. Pigment stores no dissolved oxygen. A pigment-only cell is an impermeable barrier; a zero-thickness cell is skipped without adding transport resistance. A cell with no liquid cannot carry nonzero activity at input. The support is impervious with zero oxygen flux.

Oxidation fraction `x`, network fraction `n` and remaining inhibitor fraction `I` are bounded independently in [0,1]. Initial n may exceed x, allowing a prepolymerized imported material. Network does not decrease when x<n. Fully oxidized material can continue forming a network without further oxygen uptake. Covering simply adds a transport resistance above retained cells; there is no surface-stage branch or cure reset.

The two demands `q_x` and `q_I` are oxygen-equivalent mass per reference binder volume for complete oxidation and complete depletion of the normalized initial inhibitor reservoir. Each formulation fixes what I=1 means; no universal molecular inhibitor concentration is claimed. Chemical oxygen consumption is `binder_um (q_x Δx + q_I (I_old−I_new))`. This is not net gravimetric uptake once degradation or volatile organic loss matters. Those long-term losses are not modeled.

## Transport and bounded integration

At each internal step, with current network and geometry,

`P_i = D0_i cured_diffusivity_ratio_i ^ n_i C_i / h_i`

`G_i,i+1 = 1 / [h_i/(2P_i) + h_i+1/(2P_i+1)]`

where P has oxygen-equivalent conductance units µm²/s and G has µm/s. This is harmonic interface **resistance**, including solubility/partitioning. D0 is in µm²/s. A zero permeability blocks transport. At the air face `k_s = air_transfer_um_s C_top/h_top`, and `G_air = 1/[1/k_s + h_top/(2P_top)]`. The finite transfer and exposed half-cell resistance are both present. There is no prescribed concentration or infinite oxygen reservoir inside a new paint cell.

Backward Euler solves the tridiagonal finite-volume mass equations

`C_i(a_new−a_old) = Δt Σ G_ij(a_j,new−a_i,new) + Δt G_air(a_air−a_top,new)`.

The no-flux bottom has no external term. An M-matrix tridiagonal elimination/back-substitution gives a nonnegative, maximum-principle-preserving diffusion update. Summed interface terms cancel. Air exchange uses the same implicit top activity and can be negative under an oxygen-poor atmosphere. Coefficients are lagged, so the transport/reaction split is first order, not an exact semigroup under arbitrary caller partitions.

The local reaction uses a bounded exponential extent at unknown final activity:

`I_new = I_old exp(−k_I a_new Δt)`

`r_x = k_x (1+A x_old) exp(−inhibition_strength I_new)`

`x_new = 1−(1−x_old) exp(−r_x a_new Δt)`

`C(a_old−a_new) = binder_um [q_x(x_new−x_old)+q_I(I_old−I_new)]`.

The scalar oxygen residual is monotone on [0,a_old]. Forty-eight bisections retain the feasible lower-activity bracket; consumed inventory is then subtracted directly from dissolved inventory. This prevents an extent from consuming unavailable oxygen. No oxygen clipping hides an unbalanced reaction. Network follows `n_new=n_old+max(x_new−n_old,0)[1−exp(−k_n Δt)]` and consumes no additional oxygen beyond the oxidation reservoir. This is an effective phenomenological mechanism, not an identified radical reaction scheme.

The step is at most 600 s and at most .05 divided by the largest effective oxidation, inhibitor, network or evaporation rate present. These are engineering integration controls, not source measurements. Diffusion is implicit, so the bound is not a depth-cell Courant condition. Production acceptance still requires time/depth refinement of derived skin mechanics, not just mean cure. Calendar drivers return first passing sample times, with up to one sample interval of temporal quantization. The caller owns committed/partial calendar scheduling and deterministic replay.

Optional exposed-top solvent evaporation uses `s_new=s_old exp(−k_evap Δt)`. It releases `Δs s_solvent a` dissolved oxygen to an explicitly separate outflow receipt. It is off in every fixture. Existing integration that already evaporates solvent leaves this coefficient zero to avoid double counting. No solvent below a covering cell evaporates directly in this one-dimensional surface approximation.

`OxygenBudget` inventories, integrated fluxes and chemical consumption use the same oxygen-equivalent µm. Its receipt is

`final−initial−air_exchange+oxidation_consumed+inhibitor_consumed+evaporation_oxygen_outflow−rounding_delta = balance_error`.

State is integrated in f64 scratch and rounded to f32 only on return. `rounding_delta` isolates dissolved inventory changes from that storage conversion, including remaining-solvent rounding. The final clamp removes floating-point excursions only after internal validity checks; its inventory effect is included in rounding_delta. No material inventory is clamped or renormalized. The routine validates supplied cells/kinetics before committing any changes and leaves input unchanged on error. Scratch capacity can be retained with `advance_column_with_scratch`; the convenience function uses one thread-local scratch. There is no heap allocation per internal step.

## Proposed coefficients and stable identities

No coefficient below is currently calibrated. Provenance code identifies the **target observation source**, not a claim that the coefficient was measured there. Revision 1 is a proposed model family. Binder codes: 0 unspecified, 1 linseed, 2 safflower. Drier codes: 0 unspecified/no additional drier encoded, 1 native lead-white chemical action. These are serializable numeric identities, not concentrations. Provenance codes: 0 unsourced generic proposal, 1–4 S3 rows in table order, 5 S2 historical surrogate. Recipe indices align directly to the supplied kinetics slice and are independent of optical recipes.

| Field | Proposed default | Provenance/status |
|---|---:|---|
| oxygen_solubility | .002 | Normalization proposal; no measured solubility inferred from calendar data |
| solvent_solubility | .002 | Proposal, no solvent fixture calibration |
| diffusivity_um2_s | 2 | Prototype D₀ converted from 2×10⁻¹² m²/s; illustrative only, prototype README §1 |
| cured_diffusivity_ratio | .1 | Proposed transport reduction, not prototype's exponential coefficient |
| air_transfer_um_s | .1 | Proposed effective transfer, unknown source airflow |
| oxidation_rate_s | 2×10⁻⁵ | Proposed generic seed |
| autocatalysis | 2 | Proposed effective kinetic shape |
| oxygen_demand | .16 | Proposed capacity, **not** a measured total uptake; S2 Fig.1's nearly16% is net mass and only motivates scale |
| inhibitor_rate_s | 10⁻⁵ | Proposed induction seed |
| inhibitor_oxygen_demand | .001 | Proposed normalized inhibitor capacity |
| inhibition_strength | 12 | Proposed smooth induction suppression |
| network_rate_s | 2×10⁻⁵ | Proposed oxidation/network lag |
| solvent_evaporation_rate_s | 0 | Deliberately disabled, no evaporation fit |

Fixture oxidation/induction seeds in s⁻¹ are respectively LWLO 3×10⁻⁵/3×10⁻⁵, UBLO 3×10⁻⁵/8×10⁻⁶, LWSO 2×10⁻⁵/2×10⁻⁵, UBSO 3×10⁻⁵/8×10⁻⁶ and both Golden cases 4×10⁻⁵/4×10⁻⁵. All start with I=1, x=n=a=0. These differing seeds are search initialization, not a fitted binder comparison. The solver has no legacy stiffness input. Binder inventory changes storage, oxygen demand and transport independently; oxidation and network rates allow drier effects to differ. No monotonic real-dose drier response is asserted.

S3 volume conversions use assumed pigment densities 6.81 g/cm³ for lead white and 2.35 for ultramarine from the existing `palette.rs` density table, with the existing oil reference density .93 g/cm³. These are **not S3 measurements**. Applying the same oil density to safflower is an additional proposal. Given pigment/oil mass fractions `w_p,w_b`, `f_b=(w_b/.93)/[(w_b/.93)+(w_p/ρ_p)]`. Derived binder fractions are approximately .790455, .617671, .686251 and .724519. Golden uses **assumed .65** binder volume fraction because composition is unknown. It is not a sourced titanium recipe.

## Empirical observation model

`observe` computes the volume-weighted network in the exposed contact depth and in a deeper pressure-sampling depth, plus minimum network anywhere. Cells straddling assay depths contribute only their intersected thickness. No modulus, adhesion or finger force is inferred from these averages. The operators are empirical proxies that can be refit independently of chemical coefficients:

| Observation-model parameter | Proposed value | Meaning |
|---|---:|---|
| contact_depth_um | 10 | Surface sampling depth, not a sourced skin thickness |
| pressure_depth_um | 100 | Thumb-pressure proxy depth, not dry-through |
| set_to_touch_network | .15 | Proposed no-transfer cutoff |
| dry_to_touch_network | .35 | Proposed light-rub cutoff |
| dry_hard_network | .65 | Proposed pressure resistance cutoff |
| golden_touch_network | .35 | Separate empirical cutoff, not proof that Golden's procedure equals S3 TDTT |
| maximum_solvent_fraction | .05 | Proposed surface solvent exclusion |

Set-to-touch and dry-to-touch use different contact cutoffs. Dry-hard requires the dry-to-touch surface cutoff **and** the deeper pressure cutoff, so a dry surface over a mobile core is permitted. The network minimum remains an independent bulk diagnostic. S3 dry-hard cannot be called through-dry. Neither chemistry nor these observations contains an elapsed-time stage transition.

## Fitting and verification interface

`FixtureId::fixture()` constructs six named starting candidates. `Fixture::column(count)` creates a uniform reference-volume column, recipe index 0. `predict_calendar(&fixture,count,sample_seconds,horizon_hours)` runs the actual coupled solver and returns separately sampled endpoint crossings. Change any public `Kinetics`, initial inhibitor, composition or `ObservationModel` field to perform a parameter sweep. `CalendarPrediction::loss(id)` sums squared hour residuals against only that fixture's source observations. Point residuals are signed relative to the source time, upper-bound residuals penalize only late results and interval residuals are zero within the interval. Missing crossings produce infinite loss rather than a fabricated date. These interfaces are explicit and dependency-free; no fitting optimizer is hidden in the production advance path.

An optional standalone coordinate-search driver was handed to the parent as `/home/user/.capy/work/oil-chemistry-fit.rs`, outside the repository. It imports this module by path, varies log oxidation/induction/network rates, diffusivity and finite air transfer, uses the source loss and prints both search-grid and refined-grid predictions. It fits each S3 formulation independently and one shared candidate across both Golden thicknesses. Those are in-sample fits, not held-out validation. Its finite search ranges are optimizer proposals, not identified physical parameter intervals. The parent can compile it with `rustc --edition=2024 -O` on its build machine or sakaki and record the resulting parameters, all fourteen residuals, refinement errors and exact invocation here before calling the defaults calibrated.

Recommended parent checks are standalone `rustc --edition=2024 --test crates/paint/src/oil/chemistry.rs` then the resulting test binary, or the integrated paint library chemistry filter once the parent adds the module. Source-table tests verify the fourteen observations and independent endpoint semantics; they intentionally do **not** pretend unfitted coefficients reproduce the calendar. Solver tests cover the ABI, finite surface resistance, harmonic resistance, solubility partitioning, zero substrate flux, signed air exchange, bounded reaction, induction, continued network formation, depth gradients, oil demand, distinct drier rates, solvent oxygen outflow, scratch reuse, invalid-input atomicity and caller-rounding sensitivity. None was run on this machine.

Calibration remains unfinished until those predictions have been measured and coefficients accepted. The six specimens cannot uniquely identify solubility, diffusivity, oxygen stoichiometry, reaction induction and network/contact response. S3's binder and pigment loading both change between rows. Golden has an incomplete recipe and bounded observations. Unknown mechanical moduli, free strain, adhesion, yield laws and cutting forces remain proposed outside this module; a calendar fit does not validate wrinkle mechanics or a young-film knife force model.
