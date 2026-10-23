//! Wet paint sitting on top of the dry picture.
//!
//! Every pixel holds a volume of wet paint (thickness, in "layer units": 1.0
//! is one normal coat), its pigment mixture as a latent vector of the
//! masstone (mixing is linear in latent space, weighted by volume) and its
//! Kubelka–Munk scattering per coat. Engine 9 retains the latent's measured
//! spectral K and S; the carried mean scattering records dilution and
//! drainage. Older engines derive RGB absorption from masstone and S.
//! Brushes exchange paint with this layer. It ages as the painting's clock
//! runs (`Canvas::wait`, see `drying`); `Canvas::dry` waits until it is
//! touch-dry, baking it into the dry picture with Kubelka–Munk.
//!
//! **What a paint's color means.** `Paint::color` is its *masstone*: the
//! color the paint has laid thick, which is also how it looks laid over
//! paint of that same color, at any thickness. Hiding decides how much of a
//! different underlayer shows through a thin coat. So a mark mixed to the
//! color of the field it sits in disappears into it, whether it is body
//! color or a thin scumble; a thin coat over a different color lands between
//! the two, as real paint does. To ask "what will this look like *here*",
//! use `Paint::over` and `Canvas::under`. Choose tube proportions explicitly
//! with `Palette::pile` and thin the resulting mixture with medium.
//!

use crate::canvas::Canvas;
use crate::color::{Rgb, luminance};
use crate::pigment::{Pigment, hiding_of, scatter_for};

pub const LAT: usize = crate::mixer::LATENT_SIZE;
#[derive(Clone, Default, PartialEq)]
#[cfg_attr(feature = "replay-effects", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Latent {
    pub color: [f32; LAT],
    pub parts: Vec<(usize, f32)>,
    #[cfg_attr(feature = "replay-effects", serde(skip))]
    pub(crate) layers: Vec<Layer>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "replay-effects", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct Layer {
    pub(crate) share: f32,
    pub(crate) lat: Latent,
    pub(crate) hide: Prop,
}

impl std::fmt::Debug for Latent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.parts.is_empty() { self.color.fmt(f) } else { f.debug_tuple("Latent").field(&self.color).field(&self.parts).finish() }
    }
}

impl std::ops::Deref for Latent {
    type Target = [f32; LAT];
    fn deref(&self) -> &Self::Target { &self.color }
}

impl std::ops::DerefMut for Latent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.color }
}

impl Latent {
    pub fn from_color(color: [f32; LAT]) -> Self { Self { color, parts: Vec::new(), layers: Vec::new() } }

    pub(crate) fn layers(&self, volume: f32, hide: Prop) -> Vec<Layer> {
        if self.layers.is_empty() {
            return vec![Layer { share: volume, lat: self.clone(), hide }];
        }
        let scatter: f32 = self.layers.iter().map(|l| l.share * l.hide[0]).sum();
        let scale = if scatter > 0.0 { hide[0] / scatter } else { 1.0 };
        self.layers.iter().cloned().map(|mut l| {
            l.share *= volume;
            l.hide[0] *= scale;
            l
        }).collect()
    }

    pub(crate) fn from_layers(mut layers: Vec<Layer>) -> (f32, Self, Prop) {
        layers.retain(|l| l.share > 0.0);
        let (mut volume, mut lat, mut hide) = (0.0, Self::default(), [0.0; 10]);
        for layer in &layers {
            mix_into(&mut volume, &mut lat, &mut hide, layer.share, &layer.lat, layer.hide);
        }
        if layers.len() > 1 {
            for layer in &mut layers { layer.share /= volume; }
            lat.layers = layers;
        }
        (volume, lat, hide)
    }

    pub(crate) fn split_top(&self, volume: f32, hide: Prop, take: f32, drain: bool) -> (Vec<Layer>, Vec<Layer>) {
        let mut below = self.layers(volume, hide);
        let mut top = Vec::new();
        let mut left = take.min(volume).max(0.0);
        while left > 0.0 {
            let Some(mut layer) = below.pop() else { break };
            let amount = left.min(layer.share);
            left -= amount;
            let mut remaining = layer.hide;
            if drain {
                layer.hide = crate::bristle::take_from(&mut remaining, layer.share, amount);
            }
            if amount < layer.share {
                let mut rest = layer.clone();
                rest.hide = remaining;
                rest.share -= amount;
                below.push(rest);
            }
            layer.share = amount;
            top.push(layer);
        }
        top.reverse();
        (below, top)
    }

    pub(crate) fn concentrate(&mut self, factor: f32) {
        for c in &mut self.color { *c *= factor; }
        self.scale_parts(factor);
        for layer in &mut self.layers {
            layer.lat.concentrate(factor);
            layer.hide[0] *= factor;
        }
    }

    pub(crate) fn add_parts(&mut self, other: &Self, weight: f32) {
        for &(i, f) in &other.parts {
            match self.parts.iter_mut().find(|p| p.0 == i) {
                Some(p) => p.1 += f * weight,
                None => self.parts.push((i, f * weight)),
            }
        }
    }

    pub(crate) fn scale_parts(&mut self, weight: f32) {
        for p in &mut self.parts { p.1 *= weight; }
    }

    pub(crate) fn blend_parts(&mut self, other: &Self, weight: f32) {
        self.layers.clear();
        if self.parts.is_empty() && other.parts.is_empty() { return; }
        self.scale_parts(1.0 - weight);
        self.add_parts(other, weight);
        self.parts.retain(|p| p.1 > 0.0);
    }
}
/// Paint properties mixed by volume alongside the pigment: [KM scattering per
/// coat, stiffness, drying rate, solvent, oil, packed oil, drained floor,
/// packed share, oil volume, wax].
/// Stiffness 0 = fluid, medium-rich glaze; 1 = stiff tube paint. Drying rate
/// relative to average paint (see `drying::drier`). Solvent and oil are
/// engine 4's (see `Paint`; engine 3's thinner keeps its solvent beside the
/// paint, `crate::thinner`). The last five carry each paint's pigment
/// packing (a checkpoint keeps them only with the drain switch, version
/// 14; without it, the first five): the oil
/// left, relative to tube paint as `oil` is, when the pigment packs
/// (`Paint::packed`) and when an absorbent ground has drained all it can
/// (`Paint::floor`), the share of a film packed from the ground up as it
/// drained, of its thickness, the oil's share of the volume of its tube
/// paint (`Paint::oil_volume`), and the share of its oil from waxed tubes.
/// Only the drain switch reads them (`Canvas::with_drain`,
/// `bristle::ground_drain`), which alone sets the packed share; the wax
/// stays 0.
pub type Prop = [f32; 10];

/// The packed oil and drained floor of paint that names no tubes (relative
/// to tube paint): a fine pigment's, ground as a tube is
/// (`palette::tube_packing`).
pub const PACKED_OIL: f32 = 0.7;
pub const DRAINED_FLOOR: f32 = 0.6;
/// The oil share by volume of tube paint naming no tubes: Golden's
/// measured ultramarine tube, 54% oil, which keeps about `PACKED_OIL` of it
/// packed (notes/research/pigment_oil.md).
pub const OIL_VOLUME: f32 = 0.54;

#[inline]
fn lerp_prop(p: &mut Prop, q: Prop, a: f32) {
    for k in 0..p.len() {
        p[k] += (q[k] - p[k]) * a;
    }
}

/// A paint as squeezed from the tube and thinned with medium.
///
/// It carries its Kubelka–Munk scattering `scatter` directly, so a mixture
/// handed to the brush keeps the S it was mixed with, however opaque it is
/// (hiding saturates near 1 and can't carry a large S). Name paints by
/// hiding with `Paint::new`/`with_hiding`; `hiding()` reports it back.
#[derive(Clone, Debug)]
pub struct Paint {
    pub pigment_latent: Option<[f32; LAT]>,
    pub spectral_films: bool,
    pub volume_latent: bool,
    pub parts: Vec<(usize, f32)>,
    /// Masstone, linear RGB: the paint's color laid thick (and over itself).
    pub color: Rgb,
    /// Kubelka–Munk scattering per coat: the spectral mean for engine 9's
    /// piles, flat across RGB channels for older paints.
    pub scatter: f32,
    /// Stiffness: 0 = fluid, rich in medium (a glaze), 1 = stiff tube paint.
    /// Sets how the paint levels as it dries (see `surface::settle`). How
    /// much paint goes on the brush is the separate `amount` of `Held::load`.
    pub stiff: f32,
    /// How fast it dries, relative to average paint (1): lead white and
    /// umber about 2, bone black and lakes 0.3–0.4 (`drying::drier`). With
    /// film thickness and fat (low `stiff`) it sets how long the paint stays
    /// open (see `Canvas::wait`).
    pub drying: f32,
    /// Share of solvent (turpentine, spirits) by volume in what the brush
    /// takes up, 0 (none) to 0.95 (`crate::thinner`; engine 3). The other
    /// fields stay the paint's own: solvent is never mixed into them.
    pub thinner: f32,
    /// The share of the paint's volume that is volatile solvent (turpentine,
    /// 0..0.9): it makes the paint flow on the brush and evaporates as it is
    /// laid, leaving a film of the paint's own body, that much thinner
    /// (engine 4; `pile{turps=}`).
    pub solvent: f32,
    /// How rich in oil the paint is, relative to tube paint (1): more with
    /// medium, less blotted or drawn into an absorbent ground. A film's gloss
    /// follows it (engine 4).
    pub oil: f32,
    /// The oil left, relative to tube paint (as `oil`), when the pigment
    /// packs at its critical pigment volume (`palette::Tube`'s oil
    /// absorption against the oil its tube holds).
    pub packed: f32,
    /// The oil an absorbent ground could leave at the least, below `packed`
    /// only for pigments coarser than the ground's pores (air enters the
    /// paint).
    pub floor: f32,
    /// The oil's share of the volume of the tube paint it is made of (as
    /// ground, before medium or blotting: `oil` counts those), from its
    /// tubes' oil by weight and their pigments' density
    /// (`palette::tube_packing`): about 0.5 for lead white, 0.85 for an oily
    /// tube of burnt sienna.
    pub oil_volume: f32,
}

impl Paint {
    /// Its properties as they mix in a brush and on the canvas (`Prop`).
    pub fn prop(&self) -> Prop {
        [self.scatter(), self.stiff, self.drying, self.solvent, self.oil, self.packed, self.floor, 0.0, self.oil_volume, 0.0]
    }
    /// A paint of masstone `color` whose one coat hides `hiding` (contrast
    /// ratio: over black ÷ over white; 0.05 = glaze, 0.5 = scumble,
    /// 0.92 = body).
    pub fn new(color: Rgb, hiding: f32, stiff: f32) -> Self {
        Paint { pigment_latent: None, spectral_films: false, volume_latent: false, parts: Vec::new(), color, scatter: scatter_for(luminance(color), hiding), stiff, drying: 1.0, thinner: 0.0, solvent: 0.0, oil: 1.0, packed: PACKED_OIL, floor: DRAINED_FLOOR, oil_volume: OIL_VOLUME }
    }
    /// A paint of masstone `color` that scatters `scatter` per coat.
    pub fn km(color: Rgb, scatter: f32, stiff: f32) -> Self {
        Paint { pigment_latent: None, spectral_films: false, volume_latent: false, parts: Vec::new(), color, scatter, stiff, drying: 1.0, thinner: 0.0, solvent: 0.0, oil: 1.0, packed: PACKED_OIL, floor: DRAINED_FLOOR, oil_volume: OIL_VOLUME }
    }
    pub fn body(color: Rgb) -> Self {
        Paint::new(color, 0.92, 1.0)
    }
    pub fn scumble(color: Rgb) -> Self {
        Paint::new(color, 0.5, 0.6)
    }
    /// This paint with its scattering set so one coat hides `hiding`
    /// (the masstone, stiffness and drying rate stay).
    pub fn with_hiding(self, hiding: f32) -> Self {
        Paint { scatter: scatter_for(luminance(self.color), hiding), ..self }
    }
    /// This paint with stiffness `stiff`.
    pub fn with_stiff(self, stiff: f32) -> Self {
        Paint { stiff, ..self }
    }
    /// This paint drying at `rate` relative to average paint (see
    /// `drying::drier`: lead white 2, bone black 0.4).
    pub fn with_drying(self, rate: f32) -> Self {
        Paint { drying: rate, ..self }
    }
    /// This paint thinned: `t` (0..0.95) of what the brush takes up is
    /// solvent, the rest this paint, unchanged (`crate::thinner`; engine 3).
    pub fn with_thinner(self, t: f32) -> Self {
        Paint { thinner: if t.is_nan() { 0.0 } else { t.clamp(0.0, 0.95) }, ..self }
    }
    /// The share of solvent by volume (`with_thinner`).
    pub fn thinner(&self) -> f32 {
        self.thinner
    }
    /// Hiding power of one coat (contrast ratio), derived from the
    /// scattering; for reporting (it rounds to 1 for strong scatterers).
    pub fn hiding(&self) -> f32 {
        hiding_of(luminance(self.color), self.scatter)
    }
    pub fn latent(&self) -> Latent {
        Latent { color: self.pigment_latent.unwrap_or_else(|| if self.volume_latent { crate::mixer::encode9(&self.color) } else { crate::mixer::encode(&self.color) }), parts: self.parts.clone(), layers: Vec::new() }
    }
    /// Kubelka–Munk scattering per coat (the `scatter` field).
    pub fn scatter(&self) -> f32 {
        self.scatter
    }
    /// The paint as a Kubelka–Munk layer (per coat).
    pub fn pigment(&self) -> Pigment {
        Pigment::masstone(self.color, self.scatter)
    }
    /// What `coats` of this paint look like laid over `under`.
    pub fn over(&self, under: Rgb, coats: f32) -> Rgb {
        if self.spectral_films {
            if self.volume_latent { return crate::mixer::over9(&self.latent(), self.scatter, under, coats); }
            return crate::mixer::over(&self.latent(), self.scatter, under, coats);
        }
        self.pigment().over(under, coats)
    }
}

#[derive(Clone)]
pub(crate) struct Wet {
    pub(crate) vol: Vec<f32>,
    pub(crate) lat: Vec<Latent>,
    /// [scattering, stiffness, drying rate, solvent, oil] of the wet paint.
    pub(crate) hide: Vec<Prop>,
    /// Which stroke last laid paint here (a stroke barely re-picks its own paint).
    pub(crate) stroke: Vec<u32>,
    /// Stroke that last touched a pixel, and the film floor that stroke may
    /// not lift below (one pass lifts only part of the film).
    pub(crate) touched: Vec<u32>,
    pub(crate) floor: Vec<f32>,
    /// Share of the pixel the wet paint covers (1 = all of it). Only the
    /// hairs of a pointed tool, finer than a pixel, lay paint over part of
    /// one; `dry` composites that paint at its real thickness over that
    /// share, so a hairline looks alike at any resolution.
    pub(crate) cover: Vec<f32>,
    /// Id of the stroke being painted.
    pub(crate) current: u32,
    /// Dirty bounding box in pixels (x0, y0, x1, y1), if any paint is wet.
    pub(crate) dirty: Option<(usize, usize, usize, usize)>,
    /// The painting's clock and how far each film has dried (`drying`).
    pub(crate) clock: crate::drying::Clock,
    /// Solvent in the open film, µm (not coats: it is saved as µm, and a
    /// save must restore it bit for bit), engine 3: empty before engine 3
    /// (`crate::thinner`). Saved (PAINTCK9).
    pub(crate) solv: Vec<f32>,
    /// The wet film (paint + solvent, coats) the stroke `laid_id` has added
    /// to each pixel, against its ceiling (`thinner::stroke_limit_um`).
    /// Transient: an entry counts only for the stroke `laid_id`, and when
    /// the stroke ids wrap (`Canvas::next_stroke_ids`) every entry is
    /// forgotten, so a stale entry is never read; not saved. Empty before
    /// engine 3 (allocated by `Canvas::surf`).
    pub(crate) laid: Vec<f32>,
    pub(crate) laid_id: Vec<u32>,
}

impl Wet {
    pub fn new(n: usize) -> Self {
        Wet { vol: vec![0.0; n], lat: vec![Latent::default(); n], hide: vec![[0.0, 0.5, 1.0, 0.0, 1.0, PACKED_OIL, DRAINED_FLOOR, 0.0, OIL_VOLUME, 0.0]; n], stroke: vec![0; n], touched: vec![0; n], floor: vec![0.0; n], cover: vec![1.0; n], current: 0, dirty: None, clock: Default::default(), solv: Vec::new(), laid: Vec::new(), laid_id: Vec::new() }
    }

    /// Allocate the solvent and the strokes' ceilings (a thinned paint is
    /// about to be laid).
    pub(crate) fn ensure_solvent(&mut self) {
        let n = self.vol.len();
        if self.solv.len() != n {
            self.solv = vec![0.0; n];
        }
        if self.laid.len() != n {
            self.laid = vec![0.0; n];
            self.laid_id = vec![0; n];
        }
    }

    /// Forget every stroke's ceiling (the stroke ids wrapped). 0 is no
    /// stroke's id.
    pub(crate) fn forget_laid(&mut self) {
        self.laid.fill(0.0);
        self.laid_id.fill(0);
    }

    /// Whether any open film holds solvent (within the dirty box; `w` is
    /// the buffer's width).
    pub(crate) fn has_solvent(&self, w: usize) -> bool {
        let Some((x0, y0, x1, y1)) = self.dirty else { return false };
        if self.solv.is_empty() {
            return false;
        }
        let x1 = x1.min(w);
        let h = self.solv.len() / w.max(1);
        (y0..y1.min(h)).any(|y| self.solv[y * w + x0..y * w + x1].iter().any(|&s| s > 0.0))
    }

    pub fn touch(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        self.dirty = Some(match self.dirty {
            None => (x0, y0, x1, y1),
            Some((a, b, c, d)) => (a.min(x0), b.min(y0), c.max(x1), d.max(y1)),
        });
    }
}

/// Mix `v` of (`lat`, `hide`) into a reservoir (`rv`, `rl`, `rh`).
#[inline]
pub fn mix_into(rv: &mut f32, rl: &mut Latent, rh: &mut Prop, v: f32, lat: &Latent, hide: Prop) {
    if v <= 0.0 {
        return;
    }
    let t = *rv + v;
    let a = v / t;
    for k in 0..LAT {
        rl[k] += (lat[k] - rl[k]) * a;
    }
    rl.blend_parts(lat, a);
    lerp_prop(rh, hide, a);
    *rv = t;
}


/// `coats` of `pig` over `under`, laid over the share `cover` of a pixel:
/// the paint sits there `coats / cover` thick and the rest of the pixel
/// shows `under` (area-weighted in linear light, as the eye averages it).
/// However small the share, it weighs what it covers: a hair's edge over a
/// thousandth of a pixel darkens it by a thousandth. (The paint's thickness
/// there is capped far beyond hiding, so a vanishing share stays finite.)
pub(crate) fn over_share(pig: Pigment, under: Rgb, coats: f32, cover: f32) -> Rgb {
    if cover >= 1.0 {
        return pig.over(under, coats);
    }
    if cover.is_nan() || cover <= 0.0 {
        return under;
    }
    let c = cover;
    let o = pig.over(under, (coats / c).min(1e4));
    [under[0] + (o[0] - under[0]) * c, under[1] + (o[1] - under[1]) * c, under[2] + (o[2] - under[2]) * c]
}

pub(crate) fn over_latent(lat: &Latent, scatter: f32, under: Rgb, coats: f32, cover: f32, spectral: bool, volume_latent: bool) -> Rgb {
    if !spectral && lat.layers.is_empty() {
        let color = if volume_latent { crate::mixer::decode9(lat) } else { crate::mixer::decode(lat) };
        return over_share(Pigment::masstone(color, scatter), under, coats, cover);
    }
    if cover.is_nan() || cover <= 0.0 {
        return under;
    }
    let cover = cover.min(1.0);
    let over = if lat.layers.is_empty() {
        if volume_latent { crate::mixer::over9(lat, scatter, under, (coats / cover).min(1e4)) } else { crate::mixer::over(lat, scatter, under, (coats / cover).min(1e4)) }
    } else {
        let mean: f32 = lat.layers.iter().map(|l| l.share * l.hide[0]).sum();
        let scale = if mean > 0.0 { scatter / mean } else { 1.0 };
        lat.layers.iter().fold(under, |rgb, l| {
            let thickness = (coats * l.share / cover).min(1e4);
            if spectral {
                if volume_latent { crate::mixer::over9(&l.lat, l.hide[0] * scale, rgb, thickness) } else { crate::mixer::over(&l.lat, l.hide[0] * scale, rgb, thickness) }
            } else {
                let color = if volume_latent { crate::mixer::decode9(&l.lat) } else { crate::mixer::decode(&l.lat) };
                Pigment::masstone(color, l.hide[0] * scale).over(rgb, thickness)
            }
        })
    };
    std::array::from_fn(|k| under[k] + (over[k] - under[k]) * cover)
}

/// Paste stands at most this tall for its width (a bead of oil paint: stroke
/// ridges of 0.1-0.3 mm on strokes 1-2 mm wide are 0.1-0.3; 0.5 is stiff
/// impasto).
const BEAD_ASPECT: f32 = 0.5;

/// The share of a pixel (`px_um` wide) that `coats` of wet paint in it
/// cover: at least `cover` (what the hairs touched), and at least enough
/// that the paint on it stands no taller than `BEAD_ASPECT` times the
/// pixel's width. A sliver of a pixel can't hold a bead taller than that:
/// the paint has slumped across it (c ≥ coats·COAT_UM / (BEAD_ASPECT·px_um)).
/// Linear in the paint, so a hairline's fringe keeps its share wherever the
/// line falls between pixel centers; at 1000px (pixels of 0.2-0.9 mm) it
/// takes a film over 100 µm on the sliver to act. At 3200 (0.1 mm pixels)
/// it spreads the tens of µm that leveling pours into a pixel a hair only
/// grazed over the pixel, instead of leaving them on a sliver with the rest
/// of the pixel bare.
pub(crate) fn bead_cover(cover: f32, coats: f32, px_um: f32) -> f32 {
    if cover >= 1.0 || coats.is_nan() || coats <= 0.0 || px_um.is_nan() || px_um <= 0.0 {
        return cover;
    }
    let need = coats * crate::surface::COAT_UM / (BEAD_ASPECT * px_um);
    cover.max(need).min(1.0)
}

impl Canvas {
    /// The visible color at pixel `i`: the dry picture with any wet paint
    /// on it (at its laid thickness, before it levels).
    pub(crate) fn look_px(&self, i: usize) -> Rgb {
        let v = self.wet.vol[i];
        if v < 1e-5 {
            return self.px[i];
        }
        let cover = bead_cover(self.wet.cover[i], v, self.px_mm() * 1000.0);
        if self.spectral_films() {
            return self.backing.get(i).copied().unwrap_or_default().under(self.px[i])
                .over_wet(&self.wet.lat[i], self.wet.hide[i][0], v, cover).color;
        }
        over_latent(&self.wet.lat[i], self.wet.hide[i][0], self.px[i], v, cover, false, self.engine >= crate::PIGMENT_MIXER)
    }

    /// The visible colors, pixel by pixel over the window: the dry
    /// picture with any wet paint on it (at its laid thickness).
    pub fn seen(&self) -> Vec<Rgb> {
        use rayon::prelude::*;
        if self.engine >= 4 {
            // engine 4: a matte surface veils its colors (wet paint is oily, glossy)
            return (0..self.px.len())
                .into_par_iter()
                .map(|i| {
                    if let Some(c) = self.sheet_seen(i) {
                        return c;
                    }
                    // (the thinnest wet film lets the surface under it show: no edge where it ends)
                    let v = self.wet.vol[i];
                    let cover = bead_cover(self.wet.cover[i], v, self.px_mm() * 1000.0);
                    let gloss_share = if cover > 0.0 { cover * crate::smoothstep(0.0, 0.1, v / cover) } else { 0.0 };
                    let g = crate::lerp(self.gloss[i], 1.0, gloss_share);
                    crate::canvas::haze(self.look_px(i), g)
                })
                .collect();
        }
        (0..self.px.len()).into_par_iter().map(|i| self.look_px(i)).collect()
    }

    /// What is on the canvas around (`x`, `y`) within radius `r` (units),
    /// dry picture plus wet paint, averaged in linear light: the underlayer a
    /// new mark there will sit on.
    pub fn under(&self, x: f32, y: f32, r: f32) -> Rgb {
        let f = self.f;
        let n = if r * f.scale < 1.5 { 0 } else { 2 };
        let (mut acc, mut k) = ([0.0f32; 3], 0.0f32);
        for j in -n..=n {
            for i in -n..=n {
                let (dx, dy) = (i as f32 / 2.0 * r, j as f32 / 2.0 * r);
                if n > 0 && dx * dx + dy * dy > r * r * 1.01 {
                    continue;
                }
                let (px, py) = (x + dx, y + dy);
                // (a crop render holds only its window: look where it can)
                if px < 0.0 || py < 0.0 || px >= f.width() || py >= f.height() || !f.holds(px, py) {
                    continue;
                }
                let c = self.look_px(f.index(px, py));
                for q in 0..3 {
                    acc[q] += c[q];
                }
                k += 1.0;
            }
        }
        if k == 0.0 {
            return self.look_px(f.index(x, y));
        }
        [acc[0] / k, acc[1] / k, acc[2] / k]
    }

    /// Wet paint film at a point (units), µm: 0 where the paint is dry.
    pub fn wet_um(&self, x: f32, y: f32) -> f32 {
        self.wet.vol[self.f.index(x, y)] * crate::surface::COAT_UM
    }

    /// Total wet paint on the canvas (for tests / debugging).
    pub fn wet_total(&self) -> f64 {
        self.wet.vol.iter().map(|&v| v as f64).sum::<f64>() / (self.f.scale as f64 * self.f.scale as f64)
    }

    /// Solvent in the open film at a point (units), µm: 0 where there is
    /// none (`crate::thinner`). `wet_um` is the paint alone.
    pub fn solvent_um(&self, x: f32, y: f32) -> f32 {
        self.wet.solv.get(self.f.index(x, y)).copied().unwrap_or(0.0)
    }

    /// Total solvent on the canvas, in `wet_total`'s units (coats × square
    /// units).
    pub fn solvent_total(&self) -> f64 {
        self.wet.solv.iter().map(|&v| v as f64).sum::<f64>() / crate::surface::COAT_UM as f64 / (self.f.scale as f64 * self.f.scale as f64)
    }
}
