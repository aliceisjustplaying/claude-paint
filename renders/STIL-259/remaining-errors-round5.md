# Remaining measured-row errors, round 5

08F and 13G are the revised-test rows above 0.03. The old 17-row report also includes 16F above 0.03, now explicitly fitted. Other displayed rows above 0.03 are included for completeness. These are measured model-level failure explanations; no unsupported laboratory cause or visual acceptance is claimed.

- **08F, 0.036714:** Cobalt-to-lead-white transfer is too dark (ΔL −0.03362); cobalt is calibrated on its zinc-white ladder and lead white on pure/black data. Actual-backing error is still 0.036718, so the fixed backing is not the cause.
- **13G, 0.041499:** The Prussian constants fitted to 13B/13F absorb too much in the 1.1% tint (ΔL −0.04019); this is a concentration-transfer residual, not backing error. The laboratory cause is not established.
- **16F, 0.045676:** The joint barium fit compromises across its pure film and red/green/blue mixtures, leaving the cobalt row too dark (ΔL −0.04080); fitted raw spectral RMS is 0.030535/0.037641 over black/white. This row is now fit, not held out.
- **16E, 0.042591:** The same fitted barium stock leaves its viridian mixture too light (ΔL +0.03854); raw black/white spectral RMS remains 0.017655/0.037563. This is also fit, not an independent test.
- **01C, 0.065510:** The 50% wax-eosin tint stays too light and less red (ΔL +0.04802, Δa −0.03351) when transferring from pure/10% calibration; the physical cause is unproved. This row was selection-touched, not a test.
- **11C, 0.030287:** The 51% ultramarine tint is too dark and less blue (ΔL −0.02055, Δb +0.02221). The source paper itself reports needing 39% rather than weighed 51% for this mixture; the measurement is not rewritten here. This row was selection-touched.
- **13A, 0.052241:** Pure Prussian is excluded from calibration because its red reflectance conflicts with the opaque mixture ladder; the replacement 13B/13F fit leaves its predicted masstone too dark. Main’s zero is reconstruction of its input RGB, not independent validation.

Counterfactual backing values in remaining-errors-round5.json use the same frozen coefficients and constants, with no new fit. They show backing changes below 0.001 OKLab for every listed row. The shipped decoder always uses the one fixed 07A white field.

Source for the ultramarine statement: Geldof et al. (2018), Figure 9 discussion, https://doi.org/10.1186/s40494-018-0181-6. The paper gives no conclusive explanation for the discrepancy and says it cut illumination below 400 nm to avoid fluorescence problems; fluorescence is not asserted as the cause here.
