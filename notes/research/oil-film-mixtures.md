# Homogeneous mobile paint mixtures

Explicit mean-field closure for genuinely stirred MOBILE paint only.

Sourced storage convention: IUPAC Henry solubility is linear in gas partial
pressure at equilibrium and infinite dilution, section 2.1,
https://doi.org/10.1515/pac-2020-0302 and
https://goldbook.iupac.org/terms/view/09024. This does not supply a mixed-oil
solubility law. Pizzimenti et al., sections 2.2-2.4 and Tables 1-2,
https://doi.org/10.1021/acsapm.0c01441, supply separate pigment/oil specimens,
not measured mixture coefficients. Lead-ion catalysis in their introduction
also limits the independent-material approximation below.

Derived conservation projection, using the existing chemistry.rs units:
C_j=B_j s_b,j+S_j s_s,j, M=sum C_j a_j, C=sum C_j.
P, B and S add. x=sum B_j x_j/B, n=sum B_j n_j/B,
I=sum B_j I_j/B. Named binder fractions are also binder-volume weighted.
s_b=sum B_j s_b,j/B and s_s=sum S_j s_s,j/S give C_new=C.
Set a_new=M/C_new, NOT a volume/binder average of the old activities.
Thus both dissolved oxygen amount and a common equilibrium activity survive
composition changes. Zero solvent uses the binder-weighted nominal s_s.
I remains the engine's recipe-normalized inhibitor inventory, not a measured
universal molecular concentration. Only floating-point roundoff is clipped.

Explicit constitutive MODELING ASSUMPTIONS, not sourced material constants:
homogenization replaces constituent cure histories with the conserved means.
For w_j=B_j/B, set k_x=sum w_j k_x,j and k_I=sum w_j k_I,j.
A, inhibition strength H and oxidation demand q_x use weights w_j k_x,j/k_x;
inhibitor demand q_I uses w_j k_I,j/k_I. These match the corresponding
zero-inhibitor/low-conversion rate moments and the first inhibition slope,
not every cross-moment or an unmixed population's subsequent evolution.
Network rate is binder weighted. Evaporation rate is solvent weighted
(binder weighted when S=0). Inactive-rate coefficients use binder weights.

Assume subcell oxygen equilibrium and parallel local activity gradients:
D0=sum C_j D0_j/C and air_transfer=sum C_j h_air,j/C.
log(r)=sum C_j D0_j log(r_j)/sum C_j D0_j fits the parallel permeability's
log-slope at n=0; it is not exact at arbitrary n. If D0=0, r is capacity
weighted and unused. The unchanged column solver then uses D0*r^n.
This declares a homogeneous morphology, not the harmonic resistance of
alternating slabs. Solubilities and primitive constitutive laws are retained
as independent contributions: nonideal solvent effects, catalyst exchange,
new cross-reactions and percolation are uncalibrated. No constants are copied
from an unrelated fixture. Like recipes::kinetics_for, composites carry
EXTRAPOLATED_RECIPE provenance, never an original specimen's measured label.

Optical latent coordinates, tube-part identities and effective native Prop
values use native geometric-volume weights. The oil ratio is reanchored to
the actual binder/pigment amounts so Fragment::properties does not apply
dilution twice. Named binders retain all five fractions. Current drier codes
0 (absent) and one nonzero family combine without losing that family; two
distinct nonzero families are rejected because Recipe has no identity list.
The mixed Prop retains the existing relative-drier scalar too.

Integration: scale normalized source payloads by the physical volumes
actually entrained, then call mix_mobile and replace that stirred parcel by
its ONE result. Do not mix an unworked overcoat, intact coherent fragments
or unentrained material. The caller decides mobility/contact; no arbitrary
conversion threshold or history cap is imposed here. Geometry and the new
mechanical history are explicit caller inputs, not silently reset stress.
A mixture needs positive binder; solvent-only transport stays a separate
operation. This is a conservative coarse law, not an exact population solver
or measured calendar/transport calibration. Cell remains 32 bytes and no
engine dispatch, legacy serialization or other source file changes here.
