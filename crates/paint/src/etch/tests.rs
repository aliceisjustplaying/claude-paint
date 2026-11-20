//! The bath's, the dust box's and the films' physics against their formulas
//! (notes/research/etch_physics.md). Every expected number is computed here
//! from the formula, the CRC table's values and the inputs, not read from
//! the code.

use super::*;

/// A ferric bath made up at `molar`, face down at `temp_c`.
fn ferric(molar: f64, temp_c: f64) -> Bath {
    Bath { acid: Acid::Ferric, molar, molar0: molar, temp_c, face: Face::Down, rock: None, feather: None }
}

/// Face down at 2.189 M (the CRC row: 1.2679 g/cm³, 4.038 mPa·s) and 20 °C,
/// the rate is V_Cu·k·C/2 with k from Sh = 0.15 Ra^⅓, D from the
/// Nernst–Hartley diffusivity of FeCl₃ by Stokes–Einstein, and the dense
/// spent acid's Δρ = M_Cu·C/2. It lands in the workshops' still-bath
/// 0.5–2 µm/min (IP §3.1: a faint line in 5–15 min, black in 40).
#[test]
fn a_ferric_bite_follows_mass_transport() {
    let (c, rho, mu) = (2.189e3f64, 1267.9f64, 4.038f64);
    let d_nh = 4.0 * 0.604e-9 * 2.032e-9 / (0.604e-9 + 3.0 * 2.032e-9);
    let d = d_nh * (0.8900 / mu) * (293.15 / 298.15);
    let nu = mu * 1e-3 / rho;
    let (w, h) = (0.06f64, 0.04f64);
    let l = w * h / (2.0 * (w + h));
    let drho = 63.546e-3 * c / 2.0 / rho;
    let ra = 9.80665 * drho * l.powi(3) / (nu * d);
    assert!(ra > 1e7, "the turbulent branch: Ra {ra}");
    let k = 0.15 * ra.cbrt() * d / l;
    let want = 63.546e-3 / 8960.0 * k * c / 2.0 * 1e6 * 60.0;
    let t = transport(&ferric(2.189, 20.0), Metal::Copper, l, w, 600.0);
    assert!((t.rate_um_min / want - 1.0).abs() < 1e-9, "{} against {want}", t.rate_um_min);
    assert!((0.5..2.0).contains(&want), "{want} µm/min");
    assert!((t.delta_um - d / k * 1e6).abs() < 1e-6 * t.delta_um);
}

/// Ten degrees warmer bites e^(Ea/R·(1/T₁ − 1/T₂)) faster, 1.40× from 20 to
/// 30 °C with Ea = 24.9 kJ/mol (IP §3.1).
#[test]
fn warmth_follows_arrhenius() {
    let l = 0.012;
    let r20 = transport(&ferric(2.2, 20.0), Metal::Copper, l, l, 600.0).rate_um_min;
    let r30 = transport(&ferric(2.2, 30.0), Metal::Copper, l, l, 600.0).rate_um_min;
    let want = (24.9e3f64 / 8.314462618 * (1.0 / 293.15 - 1.0 / 303.15)).exp();
    assert!((r30 / r20 / want - 1.0).abs() < 1e-9, "{} against {want}", r30 / r20);
    assert!((want - 1.40).abs() < 0.01);
}

/// The rate against the bath's strength peaks where the CRC table's
/// viscosity rises as fast as the concentration (with its convection's own
/// powers): within the measured optimum's neighbourhood, 2–2.25 M [M], and a
/// 30 % bath (2.4 M) bites faster than a 40 % one (3.5 M), as the makers say
/// (IP §3.1). Rocked, the same.
#[test]
fn ferric_peaks_near_its_measured_optimum() {
    let l = 0.012;
    for rock in [None, Some((ROCK_TILT_DEG, ROCK_PERIOD_S))] {
        let rate = |m: f64| transport(&Bath { rock, face: if rock.is_some() { Face::Up } else { Face::Down }, ..ferric(m, 20.0) }, Metal::Copper, l, 0.06, 600.0).rate_um_min;
        let (mut best, mut at) = (0.0, 0.0);
        for i in 0..=300 {
            let m = 0.5 + i as f64 * 0.01;
            if rate(m) > best {
                (best, at) = (rate(m), m);
            }
        }
        assert!((1.8..=2.4).contains(&at), "{rock:?}: the peak at {at} M");
        assert!(rate(2.4) > rate(3.5), "{rock:?}");
    }
    // Baumé: 45 °Bé is SG 1.45, the 40 % stock (CRC: 3.5 M at 1.4176)
    assert!((ferric_molar_of_baume(145.0 - 145.0 / 1.4176) - 3.496).abs() < 1e-3);
}

/// Nitric 1:8 on zinc bites several times faster than ferric on copper (IP
/// §3.2: 5–10× [P]), and its bubbles slow it less when a feather sweeps them.
#[test]
fn nitric_on_zinc_is_faster_and_feathering_helps() {
    let l = 0.012;
    let rock = Some((ROCK_TILT_DEG, ROCK_PERIOD_S));
    let fe = transport(&Bath { face: Face::Up, rock, ..ferric(2.2, 20.0) }, Metal::Copper, l, 0.06, 600.0).rate_um_min;
    let m = NITRIC_CONC_M / 9.0;
    let nitric = |feather| transport(&Bath { acid: Acid::Nitric, molar: m, molar0: m, temp_c: 20.0, face: Face::Up, rock, feather }, Metal::Zinc, l, 0.06, 600.0);
    let (still, swept) = (nitric(None), nitric(Some(FEATHER_S)));
    assert!((still.bubbles - BUBBLE_COVER).abs() < 1e-12);
    assert!(swept.bubbles < still.bubbles && swept.rate_um_min > still.rate_um_min);
    let ratio = swept.rate_um_min / fe;
    assert!((3.0..15.0).contains(&ratio), "nitric/ferric {ratio}");
}

/// A dust box's largest grain after a wait is Stokes': √(18µH/(ρgt)), 61 µm
/// after 5 s in a 0.6 m box and 18 after 60 s (IP §4.1), and no grain larger
/// lands; the longer the wait, the finer the grain and the thinner the cover.
#[test]
fn the_dust_box_sorts_by_stokes() {
    for (wait, near) in [(5.0, 61.0), (60.0, 17.6)] {
        let want = (18.0 * 1.81e-5 * 0.6 / (1070.0 * 9.80665 * wait as f64)).sqrt() * 1e6;
        assert!((largest_grain(0.6, wait) - want).abs() < 1e-9);
        assert!((want - near).abs() < 0.5, "{want}");
        let n = dust_box(0.6, wait, 1.0);
        for (i, v) in n.iter().enumerate() {
            let (lo, _, _) = dust_bin(i);
            if dust_bin(i).2 > want {
                assert_eq!(*v, 0.0, "a {} µm grain fell before the plate went in", dust_bin(i).2);
            }
            let _ = lo;
        }
    }
    let c = |wait: f64| cover(&dust_box(0.6, wait, 1.0), 1.0);
    assert!(c(5.0) > c(30.0) && c(30.0) > c(60.0));
    assert!((c(30.0) - 0.5).abs() < 0.02, "the plan's example covers {}", c(30.0));
    // settling: v = ρgd²/(18µ)
    assert!((stokes(20.0) - 1070.0 * 9.80665 * 400e-12 / (18.0 * 1.81e-5)).abs() < 1e-12);
}

/// A line bitten z from its window widens by 2z/K, and its section is the
/// window's floor plus the two quarter-ellipses (IP §3.4).
#[test]
fn undercut_widens_a_line_with_its_depth() {
    for (z, k) in [(5.0, 1.2), (20.0, 1.2), (40.0, 1.0)] {
        let (a, w) = line_section(50.0, z, k);
        assert!((w - (50.0 + 2.0 * z / k)).abs() < 1e-12);
        assert!((a - (50.0 * z + PI * z * z / (2.0 * k))).abs() < 1e-9);
    }
    // a slot bites slower as it deepens: z + g·z²/(2δ) = r·t exactly
    let (delta, g) = (60.0, slot_share(50.0, 60.0));
    let z = advance(0.0, 30.0, delta, g, 0.0);
    assert!((z + g * z * z / (2.0 * delta) - 30.0).abs() < 1e-9 && z < 30.0);
    assert!((advance(0.0, 30.0, delta, 0.0, 0.0) - 30.0).abs() < 1e-12, "a flat face bites the dose");
    // in steps or at once, the same
    let steps = (0..6).fold(0.0, |z, _| advance(z, 5.0, delta, g, 4.0));
    assert!((steps - advance(0.0, 30.0, delta, g, 4.0)).abs() < 1e-9);
}

/// The pits between dots: their cover at the face is the dots' Boolean cover
/// until the undercut eats them; a dot of radius R is lost once z > K·R, its
/// pillar's tip at √(z² − K²R²); the ink the yarns leave rises with the bite,
/// then falls once the dots go (IP §4.2: "overbitten aquatints print
/// lighter").
#[test]
fn an_aquatints_pits_rise_and_merge() {
    let (n, r, k) = (0.0022, 10.0, 1.2);
    let dots = [(n, r)];
    let c0 = 1.0 - (-PI * n * r * r).exp();
    let net = network(&dots, 2.0, k);
    let c_top = 1.0 - (-PI * n * (r - 2.0 / k).powi(2)).exp();
    assert!((net.share - (1.0 - c_top)).abs() < 1e-9 && net.share > 1.0 - c0);
    assert_eq!(net.level, 0.0);
    assert!((net.width - 1.0 / (2.0 * n * (r - 2.0 / k))).abs() < 1e-9);
    // lost: the tip of the pillar at √(z² − K²R²)
    let z = 15.0;
    let lost = network(&dots, z, k);
    assert!((lost.level - (z * z - k * k * r * r).sqrt()).abs() < 1e-9);
    assert!((lost.share - 1.0).abs() < 1e-9, "the pits have merged");
    // the tone: share × (depth − the yarn's dip) by the formula, rising then gone
    let tone = |z: f64| {
        let g = network(&dots, z, k);
        if g.vol <= 0.0 { 0.0 } else { held_um(g.share, g.width, g.vol / g.share) }
    };
    let dip = |w: f64| 75.0 - (75.0f64 * 75.0 - w * w / 4.0).sqrt();
    let g = network(&dots, 6.0, k);
    assert!((tone(6.0) - g.share * (g.vol / g.share - dip(g.width))).abs() < 1e-9);
    // it rises while the pits deepen between standing dots, peaks before the dots go (their
    // shrinking widens the gaps the yarns reach into), and is nothing once they are lost (z > K·R)
    assert!(tone(1.0) < tone(2.0) && tone(2.0) < tone(6.0), "{} {} {}", tone(1.0), tone(2.0), tone(6.0));
    let zs: Vec<f64> = (1..=150).map(|i| i as f64 * 0.1).collect();
    let peak = zs.iter().copied().max_by(|a, b| tone(*a).total_cmp(&tone(*b))).unwrap();
    assert!(peak < k * r, "the peak at {peak} µm, before the dots go at {}", k * r);
    assert_eq!(tone(1.01 * k * r), 0.0, "over-bitten: an open bite holds nothing the cloth leaves");
    // merged: one opening over the whole face, between the pillars' remnants far apart
    let gone = network(&dots, 40.0, k);
    assert!((gone.share - 1.0).abs() < 1e-9 && gone.width > 2.5 * net.width, "{gone:?} against {net:?}");
    // a fin bitten from both faces goes by its inradius: (1 − z/r)
    let (area, height) = (40.0f64, 4.0f64);
    let base = 2.0 * area / height;
    let rin = area / (0.5 * (base + 2.0 * (height * height + base * base / 4.0f64).sqrt()));
    assert!((fin_left(area, height, 0.5 * rin) - 0.5).abs() < 1e-12 && fin_left(area, height, rin) == 0.0);
}

/// A film over the metal: a bare film (t = 0) is the face's reflection plus
/// the metal's through it; bitumen darkens the plate as it thickens, more in
/// the blue (an Urbach edge); the "strong tea" thickness passes 0.45 at 550 nm.
#[test]
fn a_ground_darkens_the_plate() {
    let bare = spectral::to_rgb(&crate::plate::reflectance(Metal::Copper, 0.0));
    let thin = spectral::to_rgb(&film_over_metal(Metal::Copper, GROUND_N, ground_alpha, 1.0));
    let ground = spectral::to_rgb(&film_over_metal(Metal::Copper, GROUND_N, ground_alpha, GROUND_UM));
    let varnish = spectral::to_rgb(&film_over_metal(Metal::Copper, GROUND_N, ground_alpha, STOP_UM));
    let lum = |c: crate::Rgb| 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
    assert!(lum(bare) > lum(thin) && lum(thin) > lum(ground) && lum(ground) > lum(varnish));
    assert!((-2.0 * GROUND_ALPHA550 * GROUND_UM).exp() - 0.45 < 0.01);
    assert!(ground_alpha(450.0) > ground_alpha(550.0) && ground_alpha(550.0) > ground_alpha(650.0));
    assert!((ground_alpha(550.0) - GROUND_ALPHA550).abs() < 1e-12);
}

