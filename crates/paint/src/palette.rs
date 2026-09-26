//! Palettes: a few tube paints, and mixing on the palette.
//!
//! `Palette::pile` mixes explicitly supplied tube proportions in Mixbox
//! latent space, weighted by each tube's tinting strength. The painter
//! chooses the recipe; the palette does not search for a target color.
//!
//! Masstone colors, hiding and tinting strength are approximations from
//! pigment knowledge (not measurements); each palette's tube list cites its
//! source.
//!
//! Color matching is not part of the physical paint API.
//!
//! No automatic `Palette::mix`:
//! ```compile_fail,E0599
//! use paint::Palette;
//! Palette::smalt_box().mix([0.5; 3]);
//! ```
//!
//! No automatic `Palette::aim`:
//! ```compile_fail,E0599
//! use paint::Palette;
//! Palette::smalt_box().aim([0.5; 3], [0.2; 3], 0.3, 1.0);
//! ```
//!
//! No automatic `Palette::aim_for`:
//! ```compile_fail,E0599
//! use paint::Palette;
//! Palette::smalt_box().aim_for([0.5; 3], [0.2; 3], 0.3, 1.0, unsafe { std::mem::zeroed() });
//! ```
//!
//! No automatic `Palette::paint`:
//! ```compile_fail,E0599
//! use paint::Palette;
//! Palette::smalt_box().paint([0.5; 3], 0.3);
//! ```
//!
//! No automatic `Palette::paint_for`:
//! ```compile_fail,E0599
//! use paint::Palette;
//! Palette::smalt_box().paint_for([0.5; 3], [0.2; 3], 0.3, 1.0);
//! ```
//!
//! No automatic `Canvas::aim`:
//! ```compile_fail,E0599
//! use paint::{Canvas, Palette};
//! Canvas::new(10, 1.0, [1.0; 3]).aim(&Palette::smalt_box(), [0.5; 3], (5.0, 5.0), 1.0, 0.3, 1.0);
//! ```

#[cfg(test)]
use crate::canvas::Canvas;
use crate::color::{Rgb, hex, luminance};
#[cfg(test)]
use crate::color::to_oklab;
use crate::drying::drier;
use crate::pigment::{hiding_of, scatter_for};
use crate::rng::Rng;
use crate::wet::Paint;

/// A tube (or hand-ground) paint.
#[derive(Clone, Debug)]
pub struct Tube {
    pub name: &'static str,
    /// Masstone color, linear RGB (the paint laid thick).
    pub color: Rgb,
    /// Hiding power of one coat of the tube paint (0 transparent .. 1
    /// opaque): contrast ratio, see `pigment::hiding_of`.
    pub hiding: f32,
    /// Stiffness straight from the tube (0 fluid .. 1 stiff).
    pub stiff: f32,
    /// Tinting strength relative to an average pigment (smalt is weak,
    /// Prussian blue very strong).
    pub strength: f32,
    /// How fast the paint dries in oil, relative to average paint (1):
    /// `drying::drier`.
    pub drying: f32,
}

fn tube(name: &'static str, color: &str, hiding: f32, stiff: f32, strength: f32, drying: f32) -> Tube {
    Tube { name, color: hex(color), hiding, stiff, strength, drying }
}

/// A mixture on the palette: parts of tubes.
#[derive(Clone, Debug)]
pub struct Mixture {
    /// (tube index, fraction by volume), fractions sum to 1.
    pub parts: Vec<(usize, f32)>,
    /// Masstone of the mixture (Mixbox mix of the tubes' masstones, weighted
    /// by volume × tinting strength).
    pub color: Rgb,
    /// Hiding of one coat of the unthinned mixture (derived from `scatter`).
    pub hiding: f32,
    /// Kubelka–Munk scattering per coat: the tubes' scattering mixed by
    /// volume (two-constant KM mixing).
    pub scatter: f32,
    pub stiff: f32,
    /// Drying rate of the pile: its tubes' rates (`Tube::drying`) mixed by
    /// volume. `Mixture::paint` leaves paint at the average rate (1); a pile
    /// laid as knifed carries this rate (`Mixture::laid`).
    pub drying: f32,
}

pub struct Palette {
    pub name: &'static str,
    pub tubes: Vec<Tube>,
    lat: Vec<[f32; mixbox::LATENT_SIZE]>,
    /// Scattering per coat of each tube paint.
    scat: Vec<f32>,
}

impl Clone for Palette {
    fn clone(&self) -> Self {
        Palette::new(self.name, self.tubes.clone())
    }
}

impl std::fmt::Debug for Palette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Palette").field("name", &self.name).field("tubes", &self.tubes.iter().map(|t| t.name).collect::<Vec<_>>()).finish()
    }
}

impl Palette {
    pub fn new(name: &'static str, tubes: Vec<Tube>) -> Self {
        let lat = tubes.iter().map(|t| mixbox::linear_float_rgb_to_latent(&t.color)).collect();
        let scat = tubes.iter().map(|t| scatter_for(luminance(t.color), t.hiding)).collect();
        Palette { name, tubes, lat, scat }
    }

    /// Return a palette restricted to the named tubes. Unknown names panic.
    pub fn only(&self, names: &[&str]) -> Palette {
        let tubes = names
            .iter()
            .map(|n| self.tubes.iter().find(|t| t.name == *n).unwrap_or_else(|| panic!("no tube {n:?} in palette {}", self.name)).clone())
            .collect();
        Palette::new(self.name, tubes)
    }

    /// Lead white, smalt (semi-transparent cobalt glass, weak) and pale
    /// smalt, yellow ochre, red earth, vermilion, raw umber, bone black.
    /// Naples yellow is not included. With green tubes added:
    /// `smalt_box_greens`.
    pub fn smalt_box() -> Self {
        Palette::new(
            "smalt box",
            vec![
                tube("lead white", "#efe9dc", 0.82, 0.8, 1.0, drier::LEAD_WHITE),
                tube("smalt", "#5a6e9e", 0.3, 0.55, 0.45, drier::SMALT),
                tube("pale smalt", "#8d9bb8", 0.35, 0.55, 0.35, drier::SMALT),
                tube("yellow ochre", "#b98a36", 0.8, 0.7, 0.8, drier::OCHRE),
                tube("red earth", "#9c4a30", 0.85, 0.7, 0.9, drier::RED_EARTH),
                tube("vermilion", "#cf3a24", 0.9, 0.75, 1.0, drier::VERMILION),
                tube("raw umber", "#5c4c3a", 0.8, 0.65, 0.9, drier::UMBER),
                tube("bone black", "#1e1b19", 0.9, 0.7, 1.1, drier::BONE_BLACK),
            ],
        )
    }

    /// `smalt_box` without smalt (pale smalt stays), plus cobalt blue
    /// and chrome yellow. With green tubes added: `cobalt_box_greens`.
    pub fn cobalt_box() -> Self {
        let mut t = Palette::smalt_box().tubes;
        t.retain(|t| t.name != "smalt");
        t.push(tube("cobalt blue", "#2f55a8", 0.55, 0.6, 0.8, drier::COBALT_BLUE));
        t.push(tube("chrome yellow", "#e8b21c", 0.9, 0.7, 1.0, drier::CHROME_YELLOW));
        Palette::new("cobalt box", t)
    }

    /// Two tubes: Prussian blue and green earth. Masstones and numbers are
    /// documented approximations: Prussian blue transparent and very strong
    /// [AP3 pp.196–197] (tinting strength 3, below the sourced "very high",
    /// because Mixbox's latent already carries some of a dark pigment's
    /// strength); green earth translucent, weak, short of body [AP1 p.146;
    /// FIELD p.129], its masstone from Munsell 7.5G/2.9/1.5 [AP1 Table 1].
    /// Green earth's drying rate is an estimate (an earth: medium).
    pub fn green_tubes() -> Vec<Tube> {
        vec![tube("Prussian blue", "#172440", 0.35, 0.45, 3.0, drier::PRUSSIAN_BLUE), tube("green earth", "#3a4843", 0.2, 0.35, 0.3, drier::OCHRE)]
    }

    /// `smalt_box` plus `green_tubes`: Prussian blue (hiding 0.35,
    /// stiffness 0.45, tinting strength 3) and green earth (hiding 0.2,
    /// stiffness 0.35, tinting strength 0.3).
    pub fn smalt_box_greens() -> Self {
        Palette::smalt_box().with(Palette::green_tubes()).named("smalt box, greens")
    }

    /// `cobalt_box` plus `green_tubes` and Rinmann's green (cobalt-zinc
    /// oxide: semi-transparent, weak, permanent [WEB-co]; hiding 0.35,
    /// stiffness 0.5, tinting strength 0.4; drying estimated as cobalt's).
    pub fn cobalt_box_greens() -> Self {
        let mut t = Palette::green_tubes();
        t.push(tube("Rinmann's green", "#5f8f76", 0.35, 0.5, 0.4, drier::COBALT_BLUE));
        Palette::cobalt_box().with(t).named("cobalt box, greens")
    }

    /// Every tube the engine knows, in one box: lead white, smalt, pale
    /// smalt, yellow ochre, red earth, vermilion, raw umber, bone black,
    /// cobalt blue, chrome yellow, Prussian blue, green earth, Rinmann's
    /// green and copper green. All were made by the 1820s (cobalt blue from
    /// 1802, chrome yellow sold in Germany from about 1820, Rinmann's green
    /// rare and costly [AP3; WEB-co]).
    pub fn tube_box() -> Self {
        let mut t = Palette::smalt_box().tubes;
        let later = Palette::cobalt_box_greens().tubes;
        t.extend(later.into_iter().filter(|x| !["lead white", "pale smalt", "yellow ochre", "red earth", "vermilion", "raw umber", "bone black"].contains(&x.name)));
        t.push(Palette::copper_green());
        Palette::new("tube box", t)
    }

    /// The same tubes under another name.
    pub fn named(self, name: &'static str) -> Palette {
        Palette { name, ..self }
    }

    /// A copper green (verdigris ground in oil); not in the standard
    /// palettes (add it with `with`). Masstone and numbers are assumptions;
    /// "poor hiding power in oil" [AP2 p.132]; copper is a drier (drying
    /// rate estimated as smalt's).
    pub fn copper_green() -> Tube {
        tube("copper green", "#3f7f6a", 0.25, 0.4, 1.0, drier::SMALT)
    }

    /// This palette with extra tubes appended.
    pub fn with(&self, extra: Vec<Tube>) -> Palette {
        let mut t = self.tubes.clone();
        t.extend(extra);
        Palette::new(self.name, t)
    }

    /// Masstone, scattering per coat and stiffness of a mixture.
    fn eval(&self, parts: &[(usize, f32)]) -> (Rgb, f32, f32) {
        let mut lat = [0.0f32; mixbox::LATENT_SIZE];
        let (mut wsum, mut sct, mut stf) = (0.0, 0.0, 0.0);
        for &(i, f) in parts {
            let w = f * self.tubes[i].strength;
            for k in 0..mixbox::LATENT_SIZE {
                lat[k] += self.lat[i][k] * w;
            }
            wsum += w;
            sct += self.scat[i] * f;
            stf += self.tubes[i].stiff * f;
        }
        for v in lat.iter_mut() {
            *v /= wsum.max(1e-9);
        }
        (mixbox::latent_to_linear_float_rgb(&lat), sct, stf)
    }

    /// The pile mixed from these parts (tube index, fraction by volume;
    /// fractions sum to 1), mixed the way the palette mixes: its masstone,
    /// scattering and stiffness.
    pub fn pile(&self, parts: Vec<(usize, f32)>) -> Mixture {
        self.mixture(parts)
    }

    fn mixture(&self, parts: Vec<(usize, f32)>) -> Mixture {
        let (color, scatter, stiff) = self.eval(&parts);
        let drying = parts.iter().map(|&(i, f)| self.tubes[i].drying * f).sum::<f32>() / parts.iter().map(|p| p.1).sum::<f32>().max(1e-9);
        Mixture { hiding: hiding_of(luminance(color), scatter), parts, color, scatter, stiff, drying }
    }

    /// Jitter the proportions (relative sd `amount`) and remix, so repeated
    /// piles of one recipe vary.
    pub fn remix(&self, m: &Mixture, amount: f32, rng: &mut Rng) -> Mixture {
        if amount <= 0.0 || m.parts.len() < 2 {
            return m.clone();
        }
        let mut parts: Vec<(usize, f32)> = m.parts.iter().map(|&(i, f)| (i, (f * (1.0 + rng.normal() * amount)).max(0.0))).collect();
        let s: f32 = parts.iter().map(|p| p.1).sum();
        parts.iter_mut().for_each(|p| p.1 /= s.max(1e-9));
        self.mixture(parts)
    }

    /// Human-readable recipe, e.g. "lead white 0.72 + yellow ochre 0.20 + raw umber 0.08".
    pub fn recipe(&self, m: &Mixture) -> String {
        m.parts.iter().map(|&(i, f)| format!("{} {:.2}", self.tubes[i].name, f)).collect::<Vec<_>>().join(" + ")
    }
}

impl Mixture {
    /// This mixture as paint on the brush, thinned with `medium` (0..1).
    pub fn paint(&self, medium: f32) -> Paint {
        let k = (1.0 - medium).clamp(0.0, 1.0);
        // medium dilutes the pigment: K and S per coat fall with the pigment
        // concentration, the masstone stays; the paint flows (stiffness
        // falls faster than hiding). The paint carries S itself: hiding
        // rounds to 1 for strong scatterers and would lose it.
        Paint::km(self.color, self.scatter * k.max(1e-3), self.stiff * k * k)
    }

    /// This pile as paint on the brush, thinned with `medium` (0..1), drying
    /// at its tubes' rate (`drying`). Medium adds oil, which the drying
    /// model already counts (a fat film stays open longer).
    pub fn laid(&self, medium: f32) -> Paint {
        self.paint(medium).with_drying(self.drying)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A pile laid as knifed dries at its tubes' rate, mixed by volume: lead
    /// white fast, bone black slow, half and half in between.
    #[test]
    fn a_pile_dries_at_its_tubes_rate() {
        let pal = Palette::tube_box();
        let at = |n: &str| pal.tubes.iter().position(|t| t.name == n).unwrap();
        let (w, k) = (at("lead white"), at("bone black"));
        let white = pal.pile(vec![(w, 1.0)]).laid(0.2);
        let black = pal.pile(vec![(k, 1.0)]).laid(0.2);
        let half = pal.pile(vec![(w, 0.5), (k, 0.5)]).laid(0.2);
        assert_eq!(white.drying, drier::LEAD_WHITE);
        assert_eq!(black.drying, drier::BONE_BLACK);
        assert!((half.drying - 0.5 * (drier::LEAD_WHITE + drier::BONE_BLACK)).abs() < 1e-6, "{}", half.drying);
        // the paint is otherwise the pile's own
        let p = pal.pile(vec![(w, 0.5), (k, 0.5)]);
        assert_eq!((half.color, half.scatter, half.stiff), (p.paint(0.2).color, p.paint(0.2).scatter, p.paint(0.2).stiff));
    }

    /// The tube box holds every tube the engine knows, each once.
    #[test]
    fn the_tube_box_holds_every_tube_once() {
        let names: Vec<&str> = Palette::tube_box().tubes.iter().map(|t| t.name).collect();
        let mut known: Vec<&str> = Palette::smalt_box().tubes.iter().chain(Palette::cobalt_box_greens().tubes.iter()).map(|t| t.name).collect();
        known.push(Palette::copper_green().name);
        known.sort();
        known.dedup();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(sorted, known);
        assert_eq!(names.len(), 14);
        assert!(Palette::tube_box().tubes.iter().all(|t| t.drying > 0.0));
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::*;
    use crate::pigment::Pigment;

    /// A prescribed dark pile deepens a light ground more than a dark one,
    /// with more darkening as the film thickens.
    #[test]
    fn glazes_stay_glazes() {
        let pal = Palette::tube_box().only(&["bone black"]);
        let glaze = pal.pile(vec![(0, 1.0)]).paint(0.9);
        let (light, dark) = (hex("#d8d0bc"), hex("#2a2622"));
        let l = |c| to_oklab(c)[0];
        let light_drop = l(light) - l(glaze.over(light, 1.0));
        let dark_drop = l(dark) - l(glaze.over(dark, 1.0));
        assert!(light_drop > 0.05, "light ground darkens: {light_drop}");
        assert!(dark_drop.abs() < 0.3 * light_drop, "dark ground changes less: {dark_drop} vs {light_drop}");
        assert!(l(glaze.over(light, 2.0)) < l(glaze.over(light, 1.0)));
    }

    /// Brush paint keeps a mixture's scattering, even when it rounds to
    /// hiding 1 (e.g. opaque neutral tubes mixed in explicit proportions).
    #[test]
    fn mixture_to_paint_preserves_scattering() {
        let pal = Palette::new("opaque neutral tubes", vec![
            Tube { name: "white", color: [0.99; 3], hiding: 0.99, stiff: 0.5, strength: 1.0, drying: 1.0 },
            Tube { name: "black", color: [0.01; 3], hiding: 0.99, stiff: 0.5, strength: 1.0, drying: 1.0 },
        ]);
        for (white, medium) in [(0.1, 0.0), (0.1, 0.5), (0.6, 0.0), (0.6, 0.9)] {
            let m = pal.pile(vec![(0, white), (1, 1.0 - white)]);
            let p = m.paint(medium);
            let s = m.scatter * (1.0 - medium);
            assert!((p.scatter() - s).abs() <= 1e-4 * s, "S {} thinned {s} → paint S {}", m.scatter, p.scatter());
            let expected = Pigment::masstone(m.color, s).over([1.0; 3], 0.1);
            let got = p.over([1.0; 3], 0.1);
            assert!((expected[0] - got[0]).abs() < 1e-3, "white fraction {white} medium {medium}: expected {expected:?} got {got:?}");
            assert!((p.hiding() - hiding_of(luminance(m.color), s)).abs() < 1e-4, "hiding is reported from S");
        }
        // and the brush lays that scattering into the wet layer
        let p = pal.pile(vec![(0, 0.1), (1, 0.9)]).paint(0.0);
        let mut c = Canvas::new(100, 1.0, [1.0; 3]);
        let mut b = crate::bristle::Held::new(crate::bristle::Tool::round_sable(14.0), 1);
        b.load(p, 0.7);
        c.drag(&mut b, &crate::bristle::Gesture::new(vec![(50.0, 50.0), (53.0, 52.0)]).pressure(0.8, 0.6), None);
        let i = (0..c.wet.vol.len()).max_by(|&a, &b| c.wet.vol[a].total_cmp(&c.wet.vol[b])).unwrap();
        assert!(c.wet.vol[i] > 0.0);
        let laid = c.wet.hide[i][0];
        assert!((laid - p.scatter()).abs() <= 1e-3 * p.scatter(), "wet S {laid} vs paint S {}", p.scatter());
    }
}
