# Finite-volume oil chemistry and calibration provenance

Implementation: `crates/paint/src/oil/chemistry.rs`. Material revision 1. Named reference formulations are fitted to the fourteen source observations below; generic defaults and extrapolated recipes remain explicitly proposed. The source calendar values are not drying schedules and are never consulted by `advance_column`.

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

Elimination retains a positive effective capacity and its weighted source activity instead of subtracting nearly equal diagonal terms. If L is the time-integrated interface conductance and E is the previous effective capacity, its contribution is the series capacity LE/(L+E), evaluated without an overflowing product. Back-substitution is a convex activity combination. Air exchange uses the algebraically equivalent effective-capacity flux, avoiding subtraction of nearly equal atmospheric and surface activities. The equations are unchanged. A regression covers open and closed two-cell columns from 0.01 to 1e-30 µm with relative oxygen-balance error at most 1e-12; the previous subtraction failed for valid ultrathin cells.

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

The generic `Kinetics::default` coefficients below remain proposed numerical reference values. Named fixtures instead use the fitted rows recorded below. A fitted coefficient is an effective model parameter, not a physical quantity measured by the source. Binder codes are 0 unspecified, 1 linseed, 2 safflower, 3 walnut, 4 poppy, 5 stand and 6 an explicit mixture; the formulation separately retains all five binder fractions. Drier codes distinguish the encoded native lead-white action from no such action, not measured catalyst concentrations. Provenance codes 1–4 identify the S3 rows in table order, 5 identifies the partial historical S2 reference and100 explicitly marks an extrapolated fresh recipe. Recipe indices align with the supplied kinetics table and are independent of optical recipe indices.

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

All fixtures start with I=1 and x=n=a=0. The solver has no legacy stiffness input. Binder inventory changes storage, oxygen demand and transport independently; oxidation and network rates allow catalyst effects to differ. No monotonic real-dose drier response is asserted.

## Fitted reference formulations, material revision 1

The following effective parameters were fit jointly to the source endpoint observations with the actual transport/oxidation/network solver. S3 is Pizzimenti et al. (2021), §§2.2–2.3 and Tables1–2, DOI `10.1021/acsapm.0c01441`; S2 is Golden's historical 2011 study, “Film Thickness,” Figs.1 and4, with its 2023 formula correction. The two Golden thicknesses share one kinetic row. No fit changes the source observations, density assumptions or missing environmental data.

| Source formulation | oxidation s⁻¹ | inhibitor s⁻¹ | network s⁻¹ | D µm²/s | air transfer µm/s | inhibition | autocatalysis |
|---|---:|---:|---:|---:|---:|---:|---:|
| S3 LWLO,450 µm | 4.306142155211445e-5 | 2.025911623753712e-5 | 8.796022631963491e-6 | 95.61929404305099 | .06856045066079364 | 9.190589849379837 | 2.768390985499804 |
| S3 UBLO,450 µm | 1.4485206489848462e-5 | 2.962892878743327e-6 | 5.207092180596602e-6 | 869.9592501011125 | 1.4354844820983492 | 16.487780016955842 | 4.474785809855917 |
| S3 LWSO,450 µm | 1.5071014856830334e-5 | 6.9528774377365555e-6 | 1.5082654585657824e-5 | 277.4917035053666 | .07276096051644453 | 12.243391343452837 | 4.416371811763764 |
| S3 UBSO,450 µm | 7.816050765845935e-6 | 2.7470804768659995e-6 | 6.695107571826018e-6 | 398.2989372249973 | .40915139839049286 | 15.975922002279807 | 9.597091559423877 |
| S2 titanium/linseed,76.2 and254 µm | .00015971157038422205 | 8.238538109961529e-5 | 8.434462438911919e-6 | 4857.715032816826 | .030165193668497542 | 1.8136077753332913 | 1.1225900793684678 |

The contact operators are also fitted model parameters. Network conversion at a transfer, rub or thumb-pressure endpoint was not measured by S3, so a universal conversion cutoff would be an additional unsupported assumption. After the joint kinetic fit, the surface/pressure averages at each source endpoint were evaluated on96 depth cells and used to refine these formulation-specific operators:

| Source | set-to-touch surface n | dry-to-touch surface n | dry-hard pressure n |
|---|---:|---:|---:|
| S3 LWLO | .15187306555858643 | .35523510332021857 | .6522796714339786 |
| S3 UBLO | .15461218217387795 | .3389227809384465 | .6494597736746073 |
| S3 LWSO | .14292350225150585 | .3710772218182683 | .6446165964007378 |
| S3 UBSO | .16028567031025887 | .32477723341435194 | .6562748597934842 |
| S2 partial reference | .15 (not fit) | .35 | .65 (not fit) |

The verification uses600-second observations on96 depth cells. All twelve S3 times are reproduced within that numerical observation interval; both Golden observations satisfy their unchanged bounds (≤48 h and96–144 h). These are fitted observations, not out-of-sample predictions. The degrees of freedom exceed the experimental information: induction, transport, reaction and observation response are not uniquely identified. Calendar agreement does not validate the proposed oxygen solubility, demand, young-skin modulus, expansion strain or knife-force law.

Fresh named recipes use these parameter families without claiming that a current tube is the experimental recipe. Named pigment components are combined deterministically; effective coefficients are volume-weighted as an explicitly empirical mixture rule. Actual chemical inventories remain conserved separately. Unsupported pigment recipes and mixtures carry provenance100. Walnut/poppy activity estimates .8/.6 are inherited from the existing oil selector, not Pizzimenti measurements; only oxidation and induction are adjusted, not every endpoint by one clock multiplier. Stand oil has no fitted resolver and is refused there. Added drier dose response is not inferred from the ambiguous Golden product percentages.

Fresh bound-solvent release uses the existing estimated two-minute thin-film scale (`thinner::TAU_MIN`) with the existing thickness correction. Its source basis is Jennings' turpentine purity observation and the qualitative diffusion argument already documented in `thinner.rs`, not an artist-oil solvent-release calibration. The dry reference fixtures contain no solvent. An inactive evaporation coefficient does not reduce the chemical timestep for a cell containing no solvent.

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

The reference fits vary oxidation/induction/network rates, diffusivity, finite air transfer and observation parameters. Each S3 formulation is fitted independently; both Golden thicknesses share one formulation. These are in-sample fits, not held-out validation. Search ranges and effective coefficients are modeling choices, not identified physical parameter intervals.

`cargo test -p paint --lib oil::` checks the source observations, fitted calendar, conservative transport, bounded reaction and adaptive depth integration. The 96- and 192-cell reference predictions agree at the 600-second sampling resolution. All fourteen adaptive predictions also agree within one sample; measured grids, convergence and limitations are recorded in `oil-adaptive-depth.md`. The positive-elimination chemistry solver passes all seventeen standalone checks, including the existing calendar tolerances.

The six specimens cannot uniquely identify solubility, diffusivity, oxygen stoichiometry, reaction induction and network/contact response. S3's binder and pigment loading both change between rows. Golden has an incomplete recipe and bounded observations. Unknown mechanical moduli, free strain, adhesion, yield laws and cutting forces remain proposed outside this module; a calendar fit does not validate wrinkle mechanics or a young-film knife force model.
