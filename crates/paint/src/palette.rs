//! Palettes: a few tube paints, and mixing on the palette.
//!
//! `Palette::pile` mixes explicitly supplied tube proportions in Mixbox
//! latent space, weighted by each tube's tinting strength. The painter
//! chooses the proportions.
//!
//! Masstone colors, hiding and tinting strength are approximations from
//! pigment knowledge (not measurements); each palette's tube list cites its
//! source.
//!

#[cfg(test)]
use crate::canvas::Canvas;
use crate::color::{Rgb, hex, luminance};
#[cfg(all(test, tube_box))]
use crate::color::to_oklab;
use crate::drying::drier;
use crate::pigment::{hiding_of, scatter_for};
use crate::rng::Rng;
use crate::wet::{DRAINED_FLOOR, OIL_VOLUME, PACKED_OIL, Paint};

/// A tube (or hand-ground) paint.
#[derive(Clone, Debug)]
pub struct Tube {
    pub name: &'static str,
    /// What the paint is made of, in a few words (the guide's tube table).
    pub pigment: &'static str,
    /// Masstone color, linear RGB (the paint laid thick).
    pub color: Rgb,
    /// Hiding power of one coat of the tube paint (0 transparent .. 1
    /// opaque): contrast ratio, see `pigment::hiding_of`.
    ///
    /// An input calibration scalar, not a measurement of the rendered
    /// film: `scatter_for` turns it into the paint's scattering through a
    /// grayscale surrogate (a gray paint of the masstone's luminance, one
    /// coat over black and white 1.0). The renderer then absorbs per RGB
    /// channel, so a rendered coat's luminance contrast ratio differs from
    /// this number and two tubes can rank differently by the two (raw and
    /// burnt sienna: 0.40 and 0.45 here, about 0.45 and 0.44 rendered over a
    /// black/80% white chart; thinner check 13, notes/thinner/ACCEPTANCE.md).
    pub hiding: f32,
    /// Stiffness straight from the tube (0 fluid .. 1 stiff).
    pub stiff: f32,
    /// Tinting strength relative to an average pigment (smalt is weak,
    /// Prussian blue very strong).
    pub strength: f32,
    /// How fast the paint dries in oil, relative to average paint (1):
    /// `drying::drier`. Engines 1 and 2.
    pub drying: f32,
    /// The same in engine 3 (`drying::drier::engine3`): `drying` unless the
    /// tube's own source range called for another.
    pub drying_3: f32,
    /// Engine 6: the pigment's oil absorption (g of oil per 100 g of pigment,
    /// the stiff paste at its critical pigment volume), its density (g/cm³)
    /// and the share of that packed oil it keeps when a ground has drained
    /// all it can (`drain`: about 0.9 for pigments finer than a chalk
    /// ground's pores, which stay saturated, 0.4–0.9 for those about 1 µm,
    /// 0.1–0.3 for coarse ones, which let air in). notes/research/
    /// pigment_oil.md gives each one's sources (`PIGMENT_OIL`).
    pub oa: f32,
    pub density: f32,
    pub drain: f32,
}

fn tube(name: &'static str, pigment: &'static str, color: &str, hiding: f32, stiff: f32, strength: f32, drying: f32) -> Tube {
    let (oa, density, drain) = pigment_oil(name);
    Tube { name, pigment, color: hex(color), hiding, stiff, strength, drying, drying_3: drying, oa, density, drain }
}

/// Each pigment's oil absorption (g/100 g), density (g/cm³) and drained
/// share (`Tube::drain`); notes/research/pigment_oil.md has the sources and
/// which are estimates. A pigment not listed takes a fine one's.
const PIGMENT_OIL: &[(&str, f32, f32, f32)] = &[
    ("lead white", 10.0, 6.81, 0.6),
    ("smalt", 25.0, 2.5, 0.15),
    ("pale smalt", 30.0, 2.5, 0.25),
    ("yellow ochre", 25.0, 2.8, 0.6),
    ("red earth", 18.0, 4.0, 0.7),
    ("vermilion", 10.0, 8.1, 0.5),
    ("raw umber", 35.0, 2.68, 0.8),
    ("bone black", 43.0, 2.64, 0.9),
    ("cobalt blue", 22.0, 4.2, 0.6),
    ("chrome yellow", 20.0, 6.0, 0.65),
    ("Prussian blue", 45.0, 1.78, 0.95),
    ("green earth", 50.0, 2.75, 0.6),
    ("Rinmann's green", 28.0, 5.5, 0.6),
    ("copper green", 25.0, 1.88, 0.3),
    ("zinc white", 18.0, 5.66, 0.85),
    ("lead-tin yellow", 20.0, 8.0, 0.5),
    ("Naples yellow", 25.0, 6.6, 0.5),
    ("lemon chrome", 24.0, 6.1, 0.85),
    ("pale cadmium", 24.0, 4.6, 0.85),
    ("deep cadmium", 20.0, 4.82, 0.6),
    ("cadmium yellow", 22.0, 4.82, 0.75),
    ("Indian yellow", 40.0, 1.7, 0.6),
    ("Mars yellow", 32.5, 4.0, 0.9),
    ("transparent oxide yellow", 48.0, 4.0, 0.95),
    ("brown ochre", 25.0, 3.0, 0.6),
    ("raw sienna", 46.0, 3.27, 0.9),
    ("orange chrome", 11.0, 6.9, 0.4),
    ("Mars orange", 42.0, 4.0, 0.9),
    ("red lead", 9.0, 8.8, 0.5),
    ("orange vermilion", 10.0, 8.1, 0.55),
    ("Chinese vermilion", 10.0, 8.1, 0.3),
    ("cadmium red", 20.0, 5.1, 0.9),
    ("Mars red", 27.0, 5.0, 0.92),
    ("Indian red", 20.0, 5.0, 0.75),
    ("rose madder", 70.0, 2.0, 0.92),
    ("permanent alizarin", 60.0, 1.5, 0.95),
    ("magenta", 70.0, 2.0, 0.9),
    ("burnt sienna", 28.0, 3.95, 0.8),
    ("Mars brown", 24.0, 4.5, 0.9),
    ("bone brown", 43.0, 2.6, 0.65),
    // (asphaltum dissolves in the oil: no pigment bed to pack, the ground
    // drinks it as it drinks oil)
    ("bitumen", 0.0, 1.1, 0.0),
    ("cerulean blue", 28.0, 4.7, 0.6),
    ("ultramarine blue", 35.0, 2.35, 0.6),
    ("ultramarine ash", 30.0, 2.6, 0.2),
    ("Antwerp blue", 55.0, 1.95, 0.92),
    ("viridian", 80.0, 3.2, 0.6),
    ("emerald green", 13.0, 3.26, 0.2),
    ("cobalt violet", 25.0, 3.8, 0.3),
    ("strontium yellow", 22.0, 3.7, 0.55),
    ("barium yellow", 18.0, 4.5, 0.75),
    ("zinc yellow", 26.0, 3.45, 0.75),
    ("carmine lake", 70.0, 2.0, 0.92),
    ("yellow lake", 50.0, 2.4, 0.8),
    ("vine black", 30.0, 1.4, 0.3),
];

fn pigment_oil(name: &str) -> (f32, f32, f32) {
    PIGMENT_OIL.iter().find(|p| p.0 == name).map_or((25.0, 3.0, 0.9), |p| (p.1, p.2, p.3))
}

/// The oil a tube is ground in (engine 6). Its drying rate against
/// linseed's (`Oil::rate`) sets how fast the tube's paint dries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oil {
    Linseed,
    Walnut,
    Poppy,
}

/// Engine 6: how fast paint ground in walnut and in poppy oil dries against
/// linseed's, and the slowing of a tube's few per cent of wax
/// (notes/research/pigment_oil.md §5). Poppy: titanium white in safflower
/// oil (poppy's fatty acids) touch-dry in 8–10 days for 4–5 in refined
/// linseed (Golden 2016), thin oil films 5 days for 3–4 (Eibner 1909
/// p. 308), the bis-allylic sites of each oil 75 for 125. Walnut: Eibner's
/// 5–6 days, its bis-allylic sites 87, its cure after gelation 0.64 of
/// linseed's (DePolo et al. 2024). Wax: a linseed film with 5% wax dried in
/// 8 days for 4, with 10% in 11 (Eibner p. 415), so 3–6% of the oil about
/// 1.7×.
pub const WALNUT_RATE: f32 = 0.70;
pub const POPPY_RATE: f32 = 0.55;
pub const WAX_RATE: f32 = 0.60;
/// Engine 6: the share of its gloss a paint's wax takes away: beeswax gels
/// oil at 3–5% and its crystallites roughen the surface ("stumpfes,
/// speckiges Aussehen", Eibner p. 414; a "satiny sheen", Golden); no gloss
/// measurement exists, about 0.8 of the gloss at 60° is an estimate below
/// purpose-made matting waxes'.
pub const WAX_MATTE: f32 = 0.2;

impl Oil {
    /// Drying rate against linseed's (engine 6).
    pub fn rate(self) -> f32 {
        match self {
            Oil::Linseed => 1.0,
            Oil::Walnut => WALNUT_RATE,
            Oil::Poppy => POPPY_RATE,
        }
    }
}

/// A painter's box as its colourmen ground it (engine 6), where the
/// research found its own paint (notes/research/pigment_oil.md §5): the
/// oil its tubes are ground in unless a tube says otherwise, whether they
/// hold wax, and tubes whose oil content (w) or oil differ from the box's
/// period (`period_of`).
struct BoxGrind {
    name: &'static str,
    oil: Oil,
    wax: bool,
    tubes: &'static [(&'static str, Option<f32>, Option<Oil>)],
}

const BOX_GRIND: &[BoxGrind] = &[
    // Friedrich bought his colours ground, in bladders, and mixed the blues
    // he bought dry himself (Most et al. 2024 pp. 89–90; letter of 1821);
    // walnut oil, no resin (Mills & White 1988). Bladder colours took more
    // oil (Fernbach 1834): lead white 0.14 est., the others bought ground
    // at its ratio, 1.63 × their oil absorption est.; cobalt blue mixed by
    // him, 1.2 × est.
    BoxGrind {
        name: "tube box",
        oil: Oil::Walnut,
        wax: false,
        tubes: &[
            ("lead white", Some(0.14), None),
            ("yellow ochre", Some(0.29), None),
            ("vermilion", Some(0.14), None),
            ("raw umber", Some(0.36), None),
            ("bone black", Some(0.41), None),
            ("cobalt blue", Some(0.21), None),
            ("chrome yellow", Some(0.25), None),
            ("Prussian blue", Some(0.42), None),
            ("green earth", Some(0.45), None),
        ],
    },
    // Inness's red earth is Venetian red (Uebele 1913 p. 231: 75 : 25);
    // Antwerp blue as Church's Prussian blue est. (Uebele's 0.35 is a
    // blanc-fixe imitation, too stiff for the alumina pigment)
    BoxGrind { name: "inness", oil: Oil::Linseed, wax: false, tubes: &[("red earth", Some(0.25), None), ("Antwerp blue", Some(0.43), None)] },
    // Sargent's surviving box (about 1884–88): British tubes, mostly
    // Winsor & Newton (Hellen & Kilmurray 2016): W&N's 1901 figures (Church
    // p. 66); viridian Parry & Coste's analysed British tube; poppy in the
    // white, linseed in the darks (Ridge & Townsend 1998); magenta and bone
    // brown Roberson's (bought 1888, 1899) est.
    BoxGrind {
        name: "sargent",
        oil: Oil::Linseed,
        wax: false,
        tubes: &[
            ("lead white", None, Some(Oil::Poppy)),
            ("zinc white", Some(0.187), Some(Oil::Poppy)),
            ("lemon chrome", Some(0.359), None),
            ("chrome yellow", Some(0.359), None),
            ("cadmium yellow", Some(0.401), None),
            ("yellow ochre", Some(0.387), None),
            ("raw sienna", Some(0.706), None),
            ("burnt sienna", Some(0.60), None),
            ("bone black", Some(0.528), None),
            ("cobalt blue", Some(0.474), None),
            ("ultramarine blue", Some(0.301), None),
            ("viridian", Some(0.50), None),
            ("magenta", Some(0.50), None),
            ("bone brown", Some(0.47), None),
        ],
    },
    // Alma-Tadema's colours partly Belgian (Mommen, Blockx), no figures:
    // the British average; Blockx ground whites, blues and pale lakes in
    // poppy (Blockx 1881 pp. 11–12)
    BoxGrind {
        name: "alma-tadema",
        oil: Oil::Linseed,
        wax: false,
        tubes: &[("lead white", None, Some(Oil::Poppy)), ("cobalt blue", None, Some(Oil::Poppy)), ("rose madder", None, Some(Oil::Poppy)), ("viridian", Some(0.50), None)],
    },
    // French tubes, 1869–1890s: true oil, a third more than British for most
    // colours (Vibert 1891 pp. 116–117; Wurm and Horadam's waxed colours,
    // Eibner 1909 pp. 403–406; three measured tubes, Salvant 2012), wax in
    // all three tubes measured; poppy for whites, blues, violets and
    // greens, linseed for darks, lakes, vermilion and the chromes (Vibert;
    // Moreau-Vauthier 1912; Seurat's paints, NG Technical Bulletin 24)
    BoxGrind {
        name: "impressionist",
        oil: Oil::Linseed,
        wax: true,
        tubes: &[
            ("lead white", Some(0.13), Some(Oil::Poppy)),
            ("zinc white", Some(0.19), Some(Oil::Poppy)),
            ("yellow ochre", Some(0.43), Some(Oil::Poppy)),
            ("raw sienna", Some(0.68), Some(Oil::Poppy)),
            ("burnt sienna", Some(0.63), None),
            ("red earth", Some(0.45), Some(Oil::Poppy)),
            ("vermilion", Some(0.22), None),
            ("red lead", Some(0.12), None),
            ("bone black", Some(0.55), None),
            ("vine black", Some(0.45), None),
            ("cobalt blue", Some(0.55), Some(Oil::Poppy)),
            ("ultramarine blue", Some(0.34), Some(Oil::Poppy)),
            ("Prussian blue", Some(0.50), Some(Oil::Poppy)),
            ("cerulean blue", Some(0.50), Some(Oil::Poppy)),
            ("viridian", Some(0.50), Some(Oil::Poppy)),
            ("emerald green", Some(0.19), Some(Oil::Poppy)),
            ("cobalt violet", Some(0.36), Some(Oil::Poppy)),
            ("chrome yellow", Some(0.33), None),
            ("orange chrome", Some(0.30), None),
            ("pale cadmium", Some(0.40), None),
            ("cadmium yellow", Some(0.40), None),
            ("deep cadmium", Some(0.40), None),
            ("barium yellow", Some(0.34), None),
            ("strontium yellow", Some(0.34), None),
            ("zinc yellow", Some(0.34), None),
            ("Naples yellow", Some(0.22), Some(Oil::Poppy)),
            ("Indian yellow", Some(0.50), None),
            ("yellow lake", Some(0.49), None),
            ("rose madder", Some(0.57), None),
            ("carmine lake", Some(0.50), None),
        ],
    },
    // Late Monet's colours, hand-ground by Edouard: the French oil, no wax
    // found; poppy in a green, a yellow, a red and a lilac, linseed in a
    // white (Roy, NG Technical Bulletin 28, 2007)
    BoxGrind {
        name: "giverny",
        oil: Oil::Poppy,
        wax: false,
        tubes: &[
            ("lead white", Some(0.13), Some(Oil::Linseed)),
            ("zinc white", Some(0.19), None),
            ("yellow ochre", Some(0.43), None),
            ("vermilion", Some(0.22), None),
            ("cobalt blue", Some(0.55), None),
            ("ultramarine blue", Some(0.34), None),
            ("cobalt violet", Some(0.36), None),
            ("viridian", Some(0.50), None),
            ("pale cadmium", Some(0.40), None),
            ("cadmium yellow", Some(0.40), None),
            ("deep cadmium", Some(0.40), None),
            ("barium yellow", Some(0.34), None),
            ("zinc yellow", Some(0.34), None),
            ("rose madder", Some(0.57), Some(Oil::Linseed)),
            ("carmine lake", Some(0.50), Some(Oil::Linseed)),
        ],
    },
    // Hopper's Winsor & Newton (his ledgers from 1945; "the maker is
    // Winsor and Newton", 1959): W&N's 1901 figures (Church p. 66;
    // Stockmeier's W&N light red 41.9%); its whites poppy-rich (a 1957 W&N
    // flake white, Tate 2016)
    BoxGrind {
        name: "hopper",
        oil: Oil::Linseed,
        wax: false,
        tubes: &[
            ("lead white", Some(0.13), Some(Oil::Poppy)),
            ("zinc white", None, Some(Oil::Poppy)),
            ("yellow ochre", Some(0.39), None),
            ("red earth", Some(0.41), None),
            ("burnt sienna", Some(0.60), None),
            ("bone black", Some(0.53), None),
            ("cobalt blue", Some(0.47), None),
        ],
    },
    // Tonn's Williamsburg, Old Holland and Michael Harding (his own posts):
    // Williamsburg's flake white "moderate", its cerulean "moderate"
    // (beeswax in all Williamsburg colours, amount unknown: not counted)
    BoxGrind { name: "tonn", oil: Oil::Linseed, wax: false, tubes: &[("lead white", Some(0.15), None), ("cerulean blue", Some(0.28), None)] },
];

fn box_grind(box_name: &str) -> Option<&'static BoxGrind> {
    BOX_GRIND.iter().find(|b| b.name == box_name)
}

/// The oil a tube of `name` is ground in in the box `box_name`, and
/// whether it holds wax: the box's, or linseed without wax.
pub fn grind_of(box_name: &str, name: &str) -> (Oil, bool) {
    box_grind(box_name).map_or((Oil::Linseed, false), |b| (b.tubes.iter().find(|t| t.0 == name).and_then(|t| t.2).unwrap_or(b.oil), b.wax))
}

/// The most of its oil a tube's paint keeps when its pigment packs: tube
/// paint is a workable paste, so richer than its critical pigment volume,
/// and makers grind it just richer (Golden's measured ultramarine keeps
/// 0.75). Where a pigment's modern oil absorption against a period's oil
/// says less (viridian, Naples yellow, carmine and Antwerp blue in the 19th
/// century's tubes), the period's pigment took less oil than today's.
const MAX_PACKED: f32 = 0.9;

/// When a box's paint was made, which sets how much oil its tubes hold
/// (`tube_oil`): ground by hand or bought in bladders before tubes (about
/// 1800–1840), the 19th century's tube colours (about 1850–1925), the
/// 20th century's (about 1925–1965), and today's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Period {
    Bladder,
    Tube19,
    Tube20,
    Modern,
}

/// A box's period: the default box is the early 19th century's (smalt,
/// verdigris, Rinmann's green); the painters' boxes their painters' working
/// years. A palette of another name is a 19th-century tube box.
pub fn period_of(box_name: &str) -> Period {
    match box_name {
        _ if Some(box_name) == default_box() => Period::Bladder,
        "hopper" => Period::Tube20,
        "tonn" => Period::Modern,
        _ => Period::Tube19,
    }
}

/// The oil a tube of `name` holds, as a share of its weight, in `period`,
/// and the oil absorption of what it is ground from (the pigment's, or an
/// extended pigment's: `TUBE_OIL_20`'s cadmiums): notes/research/
/// pigment_oil.md gives each one's source. One not listed is ground stiff,
/// its oil 1.45 times its pigment's oil absorption (the early 19th
/// century's rule for hand-ground paint, which Watin's lead white sets).
fn tube_oil(box_name: &str, period: Period, name: &str, oa: f32) -> (f32, f32) {
    if let Some(w) = box_grind(box_name).and_then(|b| b.tubes.iter().find(|t| t.0 == name)).and_then(|t| t.1) {
        return (w, oa);
    }
    let table: &[(&str, f32)] = match period {
        Period::Bladder => TUBE_OIL_BLADDER,
        Period::Tube19 => TUBE_OIL_19,
        Period::Tube20 => TUBE_OIL_20,
        Period::Modern => TUBE_OIL_MODERN,
    };
    let w = table.iter().find(|t| t.0 == name).map_or_else(|| 1.45 * oa / (100.0 + 1.45 * oa), |t| t.1);
    let oa = match period {
        Period::Tube20 => EXTENDED_20.iter().find(|t| t.0 == name).map_or(oa, |t| t.1),
        _ => oa,
    };
    (w, oa)
}

/// A tube's oil when its pigment packs and when an absorbent ground has
/// drained it, relative to the tube's own oil (`Paint::packed`,
/// `Paint::floor`), and its oil's share of its volume. The pigment packs
/// (its critical pigment volume) holding its oil absorption: `oa` g per
/// 100 g against the tube's `w` (oil by weight), so the packed share is
/// oa·(1 − w)/(100·w), at most `MAX_PACKED`. A pigment finer than the
/// ground's pores keeps its packed layer saturated; a coarser one lets air
/// in and drains further (`Tube::drain`).
pub fn tube_packing(t: &Tube, box_name: &str) -> (f32, f32, f32) {
    let (w, oa) = tube_oil(box_name, period_of(box_name), t.name, t.oa);
    let packed = (oa * (1.0 - w) / (100.0 * w)).min(MAX_PACKED);
    // oil (0.93 g/cm³) per volume of pigment, and its share of the paint
    let r = w / (1.0 - w) * t.density / 0.93;
    (packed, t.drain * packed, r / (1.0 + r))
}

/// About 1800–1840, ground by hand or in bladders (Watin 1823, Bouvier 1827,
/// Field 1835; most from the rule above, which Watin's lead white sets).
const TUBE_OIL_BLADDER: &[(&str, f32)] = &[
    ("lead white", 0.125),
    ("smalt", 0.23),
    ("pale smalt", 0.265),
    ("yellow ochre", 0.266),
    ("red earth", 0.25),
    ("vermilion", 0.107),
    ("raw umber", 0.337),
    ("bone black", 0.384),
    ("cobalt blue", 0.242),
    ("chrome yellow", 0.225),
    ("Prussian blue", 0.395),
    ("green earth", 0.42),
    ("Rinmann's green", 0.289),
    ("copper green", 0.266),
];

/// About 1850–1925, artists' tube colours (Church 1915, Roberson's and
/// Winsor & Newton's of 1901; Uebele 1913; Stockmeier's and Parry & Coste's
/// analyses). French makers ground in more poppy oil with wax for the same
/// body, so their paint takes the same effective share.
const TUBE_OIL_19: &[(&str, f32)] = &[
    ("lead white", 0.134),
    ("zinc white", 0.173),
    ("lemon chrome", 0.309),
    ("chrome yellow", 0.309),
    ("orange chrome", 0.175),
    ("barium yellow", 0.28),
    ("strontium yellow", 0.28),
    ("zinc yellow", 0.28),
    ("pale cadmium", 0.336),
    ("cadmium yellow", 0.336),
    ("deep cadmium", 0.336),
    ("Naples yellow", 0.145),
    ("Indian yellow", 0.44),
    ("yellow lake", 0.42),
    ("yellow ochre", 0.379),
    ("brown ochre", 0.379),
    ("raw sienna", 0.651),
    ("Mars yellow", 0.30),
    ("Mars orange", 0.32),
    ("Mars red", 0.25),
    ("Mars brown", 0.27),
    ("red lead", 0.095),
    ("vermilion", 0.187),
    ("orange vermilion", 0.187),
    ("Chinese vermilion", 0.187),
    ("cadmium red", 0.25),
    ("red earth", 0.41),
    ("Indian red", 0.25),
    ("rose madder", 0.507),
    ("carmine lake", 0.40),
    ("magenta", 0.45),
    ("burnt sienna", 0.59),
    ("raw umber", 0.49),
    ("bone brown", 0.498),
    ("bitumen", 0.559),
    ("bone black", 0.498),
    // (Uebele's own figure: vine black grinds with only 35–40% oil)
    ("vine black", 0.40),
    ("cobalt blue", 0.404),
    ("cerulean blue", 0.45),
    ("ultramarine blue", 0.277),
    ("ultramarine ash", 0.35),
    ("Prussian blue", 0.428),
    ("Antwerp blue", 0.35),
    ("viridian", 0.351),
    ("emerald green", 0.15),
    ("cobalt violet", 0.30),
];

/// About 1920–1965: no maker's tube was analysed for its oil, so these are
/// the period's recipes for artists' tube colours (Uebele 1913) and pastes
/// ground in raw linseed (Ingalls' table in Gardner 1927), with the 1942
/// standard's zinc white (CS98-42) and Mayer's lead white paste (1960).
/// Makers' stearate (from about 1930) and fillers (from about 1940) raise
/// the oil some more; nothing measures how much.
const TUBE_OIL_20: &[(&str, f32)] = &[
    ("lead white", 0.11),
    ("zinc white", 0.18),
    // cadmium-barium (cadmium lithopone), as most of the period's cadmiums
    // were (Mayer 1970; barium sulphate in a 1963 tube, Tate 2016)
    ("pale cadmium", 0.22),
    ("cadmium yellow", 0.22),
    // (an estimate: Ingalls gives no deep cadmium lithopone)
    ("deep cadmium", 0.20),
    ("yellow ochre", 0.30),
    ("red earth", 0.22),
    ("burnt sienna", 0.55),
    ("bone black", 0.47),
    // (an estimate: nothing found for the period)
    ("cerulean blue", 0.40),
    ("cobalt blue", 0.60),
    ("ultramarine blue", 0.30),
    // (Ingalls' 50% pigment; Uebele's 65:35 is stiffer than the pigment's
    // oil absorption allows)
    ("viridian", 0.50),
];

/// The oil absorption of the 20th century's extended cadmiums: about 40%
/// cadmium sulphide to 60% barium sulphate (oil absorption 12, Ingalls'
/// blanc fixe paste), by weight among the solids.
const EXTENDED_20: &[(&str, f32)] = &[("pale cadmium", 16.8), ("cadmium yellow", 16.0), ("deep cadmium", 15.2)];

/// Today's artists' oil colours: Golden's measured oil (pigment volume
/// 46% for ultramarine), Mecklenburg's reference paints, Rublev's lead
/// white, and Williamsburg's 2023 ranking of oil by volume, read as pigment
/// volume 47% (low), 40% (moderate), 31% (medium) and 22% (high), which
/// Golden's ultramarine and cobalt blue set. Lead-tin yellow and cerulean
/// are estimates.
const TUBE_OIL_MODERN: &[(&str, f32)] = &[
    ("lead white", 0.13),
    ("lead-tin yellow", 0.18),
    ("cadmium yellow", 0.20),
    ("yellow ochre", 0.28),
    ("transparent oxide yellow", 0.45),
    ("cadmium red", 0.20),
    ("permanent alizarin", 0.60),
    ("burnt sienna", 0.28),
    ("raw umber", 0.34),
    ("bone black", 0.44),
    ("ultramarine blue", 0.32),
    ("cerulean blue", 0.40),
    ("green earth", 0.38),
    ("cobalt violet", 0.25),
];

impl Tube {
    /// This tube drying at `rate` in engine 3 (`Tube::drying_3`).
    fn engine3(self, rate: f32) -> Tube {
        Tube { drying_3: rate, ..self }
    }
}

/// Every tube the engine knows, each defined once: `tube(name, pigment,
/// masstone, hiding, stiffness, tinting strength, drying)`. Boxes
/// (`Palette::tube_box`, `Palette::named_box`) take their tubes from here by
/// name.
///
/// The first fourteen are the tube box's (unchanged since round 19: old
/// paintings replay with them bit for bit).
///
/// Each tube is in the build only with a box that holds it (the `box-*`
/// features; `tube_box`, from build.rs, for the default box): a painter's
/// build for one box holds that box's tube records and no others
/// (`the_catalog_is_this_builds_boxes`). The rest came with round 20's
/// boxes; notes/r20/TUBES.md gives each one's numbers, the proposal they
/// come from and why. The last six (strontium yellow to vine black) came
/// with the giverny and impressionist boxes; their materials notes
/// (notes/research/{giverny,impressionist}_materials.md) map them to the
/// pigments found, and the comment at each gives its basis. All numbers are
/// estimates from the pigment literature, not measurements (the three
/// chromate yellows' color and strength are from measurements of
/// reconstructed pigments and test paints).
pub fn catalog() -> Vec<Tube> {
    vec![
        // ---- the tube box (round 19)
        tube("lead white", "basic lead carbonate", "#efe9dc", 0.82, 0.8, 1.0, drier::LEAD_WHITE),
        // semi-transparent cobalt glass, weak
        #[cfg(tube_box)]
        tube("smalt", "cobalt potash glass, coarse", "#5a6e9e", 0.3, 0.55, 0.45, drier::SMALT),
        #[cfg(tube_box)]
        tube("pale smalt", "a paler grade of smalt", "#8d9bb8", 0.35, 0.55, 0.35, drier::SMALT),
        tube("yellow ochre", "hydrated iron oxide earth", "#b98a36", 0.8, 0.7, 0.8, drier::OCHRE),
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-inness", feature = "box-alma-tadema", feature = "box-hopper", feature = "box-impressionist"))]
        tube("red earth", "iron oxide earth", "#9c4a30", 0.85, 0.7, 0.9, drier::RED_EARTH),
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-giverny", feature = "box-impressionist"))]
        tube("vermilion", "mercuric sulfide", "#cf3a24", 0.9, 0.75, 1.0, drier::VERMILION),
        #[cfg(any(tube_box, feature = "box-inness", feature = "box-alma-tadema", feature = "box-tonn"))]
        tube("raw umber", "iron and manganese oxide earth", "#5c4c3a", 0.8, 0.65, 0.9, drier::UMBER),
        // (every box but the giverny box, which has no black)
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-inness", feature = "box-alma-tadema", feature = "box-tonn", feature = "box-hopper", feature = "box-impressionist"))]
        tube("bone black", "charred bone (carbon, calcium phosphate)", "#1e1b19", 0.9, 0.7, 1.1, drier::BONE_BLACK).engine3(drier::engine3::BONE_BLACK),
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-inness", feature = "box-alma-tadema", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist"))]
        tube("cobalt blue", "cobalt aluminate", "#2f55a8", 0.55, 0.6, 0.8, drier::COBALT_BLUE).engine3(drier::engine3::COBALT_BLUE),
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-impressionist"))]
        tube("chrome yellow", "lead chromate", "#e8b21c", 0.9, 0.7, 1.0, drier::CHROME_YELLOW),
        // Prussian blue transparent and very strong [AP3 pp.196–197]
        // (tinting strength 3, below the sourced "very high", because
        // Mixbox's latent already carries some of a dark pigment's strength)
        #[cfg(any(tube_box, feature = "box-impressionist"))]
        tube("Prussian blue", "iron ferrocyanide", "#172440", 0.35, 0.45, 3.0, drier::PRUSSIAN_BLUE).engine3(drier::engine3::PRUSSIAN_BLUE),
        // green earth translucent, weak, short of body [AP1 p.146; FIELD
        // p.129], its masstone from Munsell 7.5G/2.9/1.5 [AP1 Table 1]; its
        // drying rate is an estimate (an earth: medium)
        #[cfg(any(tube_box, feature = "box-tonn"))]
        tube("green earth", "celadonite and glauconite clay", "#3a4843", 0.2, 0.35, 0.3, drier::OCHRE),
        // cobalt-zinc oxide: semi-transparent, weak, permanent [WEB-co];
        // drying estimated as cobalt's
        #[cfg(tube_box)]
        tube("Rinmann's green", "cobalt-zinc oxide", "#5f8f76", 0.35, 0.5, 0.4, drier::COBALT_BLUE),
        // verdigris ground in oil: "poor hiding power in oil" [AP2 p.132];
        // copper is a drier (drying rate estimated as smalt's)
        #[cfg(tube_box)]
        tube("copper green", "verdigris ground in oil", "#3f7f6a", 0.25, 0.4, 1.0, drier::SMALT),
        // ---- round 20 (notes/r20/TUBES.md)
        #[cfg(any(feature = "box-sargent", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist"))]
        tube("zinc white", "zinc oxide", "#f3f3ef", 0.6, 0.6, 1.0, drier::ZINC_WHITE),
        #[cfg(feature = "box-tonn")]
        tube("lead-tin yellow", "lead-tin oxide", "#e3cc6a", 0.85, 0.75, 0.6, drier::LEAD_WHITE),
        #[cfg(any(feature = "box-alma-tadema", feature = "box-impressionist"))]
        tube("Naples yellow", "lead antimonate", "#e2b964", 0.85, 0.75, 0.6, drier::NAPLES_YELLOW),
        #[cfg(any(feature = "box-sargent", feature = "box-inness"))]
        tube("lemon chrome", "pale lead chromate with lead sulfate", "#eed83c", 0.8, 0.7, 0.8, drier::CHROME_YELLOW),
        #[cfg(any(feature = "box-alma-tadema", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist"))]
        tube("pale cadmium", "cadmium sulfide, a pale grade", "#f0c63c", 0.85, 0.7, 1.1, drier::CADMIUM).engine3(drier::engine3::CADMIUM),
        #[cfg(any(feature = "box-alma-tadema", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist"))]
        tube("deep cadmium", "cadmium sulfide, a deep grade", "#e8861e", 0.9, 0.6, 1.2, drier::CADMIUM).engine3(drier::engine3::CADMIUM),
        #[cfg(any(feature = "box-sargent", feature = "box-inness", feature = "box-tonn", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist"))]
        tube("cadmium yellow", "cadmium sulfide", "#e8a51f", 0.85, 0.7, 1.1, drier::CADMIUM).engine3(drier::engine3::CADMIUM),
        #[cfg(any(feature = "box-sargent", feature = "box-impressionist"))]
        tube("Indian yellow", "magnesium and calcium euxanthate", "#e1a11e", 0.15, 0.4, 0.8, drier::INDIAN_YELLOW),
        #[cfg(feature = "box-sargent")]
        tube("Mars yellow", "synthetic iron oxide hydroxide", "#c4872b", 0.85, 0.7, 1.1, drier::MARS),
        #[cfg(feature = "box-tonn")]
        tube("transparent oxide yellow", "transparent synthetic iron oxide", "#7a4a14", 0.2, 0.5, 0.9, drier::RED_EARTH),
        #[cfg(feature = "box-alma-tadema")]
        tube("brown ochre", "iron oxide earth, a darker grade", "#86592e", 0.8, 0.7, 0.8, drier::OCHRE),
        #[cfg(any(feature = "box-sargent", feature = "box-inness", feature = "box-impressionist"))]
        tube("raw sienna", "sienna earth, unroasted", "#9a6a2b", 0.4, 0.5, 0.7, drier::SIENNA),
        #[cfg(any(feature = "box-inness", feature = "box-impressionist"))]
        tube("orange chrome", "basic lead chromate", "#e0712a", 0.88, 0.75, 0.9, drier::CHROME_YELLOW),
        // Mars orange: "much transparency" in the period account (Salter's
        // Field, 1869), so less hiding than the other Mars tubes
        #[cfg(feature = "box-sargent")]
        tube("Mars orange", "synthetic iron oxide, an orange grade", "#b8602a", 0.5, 0.7, 1.1, drier::MARS),
        #[cfg(any(feature = "box-sargent", feature = "box-impressionist"))]
        tube("red lead", "lead tetroxide", "#e0542b", 0.85, 0.8, 0.8, drier::RED_LEAD),
        #[cfg(feature = "box-alma-tadema")]
        tube("orange vermilion", "mercuric sulfide, a yellower grade", "#dd4a22", 0.9, 0.75, 1.0, drier::VERMILION),
        #[cfg(feature = "box-alma-tadema")]
        tube("Chinese vermilion", "mercuric sulfide, a deeper grade", "#b8282e", 0.9, 0.75, 1.0, drier::VERMILION),
        #[cfg(any(feature = "box-sargent", feature = "box-tonn"))]
        tube("cadmium red", "cadmium sulfoselenide", "#c3321f", 0.9, 0.7, 1.1, drier::CADMIUM).engine3(drier::engine3::CADMIUM),
        #[cfg(feature = "box-sargent")]
        tube("Mars red", "synthetic iron oxide", "#a33f2a", 0.9, 0.7, 1.2, drier::MARS),
        #[cfg(feature = "box-inness")]
        tube("Indian red", "nearly pure ferric oxide", "#7a3a33", 0.92, 0.7, 1.2, drier::RED_EARTH),
        #[cfg(any(feature = "box-sargent", feature = "box-alma-tadema", feature = "box-giverny", feature = "box-impressionist"))]
        tube("rose madder", "madder lake on alumina", "#8e2238", 0.1, 0.35, 0.9, drier::MADDER_LAKE).engine3(drier::engine3::ALIZARIN),
        #[cfg(feature = "box-tonn")]
        tube("permanent alizarin", "a quinacridone", "#5e1624", 0.15, 0.45, 1.3, drier::MADDER_LAKE).engine3(drier::engine3::ALIZARIN),
        // an aniline dye laked on alumina: transparent, strong, at madder
        // lake's rate; it fades in light, which the engine doesn't model
        #[cfg(feature = "box-sargent")]
        tube("magenta", "fuchsine (aniline) lake on alumina", "#8f1650", 0.1, 0.35, 1.5, drier::MADDER_LAKE),
        #[cfg(any(feature = "box-sargent", feature = "box-alma-tadema", feature = "box-tonn", feature = "box-hopper", feature = "box-impressionist"))]
        tube("burnt sienna", "roasted sienna earth", "#7c3f24", 0.45, 0.55, 0.9, drier::SIENNA).engine3(drier::engine3::BURNT_SIENNA),
        // Mars brown at sienna's rate: iron oxides dry well but lack umber's
        // manganese (notes/r20/TUBES.md)
        #[cfg(feature = "box-sargent")]
        tube("Mars brown", "synthetic iron oxide, roasted", "#5a3a28", 0.85, 0.65, 1.0, drier::SIENNA),
        #[cfg(feature = "box-sargent")]
        tube("bone brown", "bone roasted until brown", "#4b3527", 0.6, 0.6, 0.9, drier::BONE_BROWN),
        #[cfg(feature = "box-inness")]
        tube("bitumen", "asphaltum", "#2e2017", 0.12, 0.3, 0.7, drier::BITUMEN),
        #[cfg(any(feature = "box-sargent", feature = "box-tonn", feature = "box-hopper", feature = "box-impressionist"))]
        tube("cerulean blue", "cobalt stannate", "#3f82b3", 0.8, 0.7, 0.6, drier::COBALT_BLUE),
        #[cfg(any(feature = "box-sargent", feature = "box-tonn", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist"))]
        tube("ultramarine blue", "synthetic ultramarine", "#232a8c", 0.3, 0.5, 1.1, drier::ULTRAMARINE).engine3(drier::engine3::ULTRAMARINE),
        // the last, palest extraction of natural ultramarine: mostly
        // colorless matter, so weak and transparent
        #[cfg(feature = "box-sargent")]
        tube("ultramarine ash", "natural ultramarine, a pale last extraction", "#7d8aa8", 0.15, 0.5, 0.3, drier::ULTRAMARINE).engine3(drier::engine3::ULTRAMARINE),
        #[cfg(feature = "box-inness")]
        tube("Antwerp blue", "Prussian blue on an alumina base", "#26406c", 0.4, 0.45, 1.6, drier::ANTWERP_BLUE),
        #[cfg(any(feature = "box-sargent", feature = "box-alma-tadema", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist"))]
        tube("viridian", "hydrated chromium oxide", "#1c4a40", 0.3, 0.5, 0.9, drier::VIRIDIAN),
        #[cfg(any(feature = "box-sargent", feature = "box-impressionist"))]
        tube("emerald green", "copper aceto-arsenite", "#23a57a", 0.6, 0.6, 0.6, drier::COPPER),
        // cobalt pigments are siccative in oil; set at cobalt blue's rate
        #[cfg(any(feature = "box-sargent", feature = "box-tonn", feature = "box-giverny", feature = "box-impressionist"))]
        tube("cobalt violet", "cobalt phosphate or arsenate", "#7e4c8e", 0.35, 0.55, 0.35, drier::COBALT_BLUE),
        // ---- the giverny and impressionist boxes (notes/research/{giverny,impressionist}_materials.md)
        // lead-free chromate yellows of the 1850s-80s, as reconstructed from
        // historical recipes and measured (Otero et al. 2017, Heritage
        // Science 5:46): the pigments' color L*a*b* 94/-11/55, 90/-8/52,
        // 87/5/89, and tinting strength in test paints (PVA, with barium
        // sulfate) 78%, 92%, 65% of lead chromate's; masstones darkened for
        // oil (this easel's adjustment); hiding from their refractive indices
        #[cfg(feature = "box-impressionist")]
        tube("strontium yellow", "strontium chromate", "#f2df53", 0.55, 0.65, 0.8, drier::CHROMATE),
        #[cfg(any(feature = "box-giverny", feature = "box-impressionist"))]
        tube("barium yellow", "barium chromate (lemon yellow)", "#efd75c", 0.45, 0.7, 0.9, drier::CHROMATE),
        // zinc yellow darkens with time (to dichromate brown or Cr2O3 green)
        #[cfg(any(feature = "box-giverny", feature = "box-impressionist"))]
        tube("zinc yellow", "potassium zinc chromate", "#fcc400", 0.4, 0.6, 0.65, drier::ZINC_YELLOW),
        // cochineal lake: found with madder in late-19th-c. French paint
        // (Pozzi et al. 2014); fugitive. Estimates
        #[cfg(any(feature = "box-giverny", feature = "box-impressionist"))]
        tube("carmine lake", "carminic acid (cochineal) on alumina", "#861c3c", 0.1, 0.3, 1.2, drier::MADDER_LAKE).engine3(drier::engine3::ALIZARIN),
        // flavonoid yellow lake (Butler 1984; NG TB 24); fugitive. Estimates
        #[cfg(feature = "box-impressionist")]
        tube("yellow lake", "flavonoid dye (weld, quercitron) on alumina and chalk", "#9e7525", 0.08, 0.35, 0.5, drier::MADDER_LAKE).engine3(drier::engine3::ALIZARIN),
        // Engine-3 drying follows the medium carbon-black class; estimate.
        // (A stroke touch-dry in about 4.9 days, by the drying test's film;
        // engine 2's rate alone would take about 12.6 at engine 3's pace,
        // 5.0 in engine 2 itself.)
        // charcoal black: bluish, weak, without bone's phosphate (Butler
        // 1984 found it in 9 of 10 paintings examined). Estimates
        #[cfg(feature = "box-impressionist")]
        tube("vine black", "charcoal of vine twigs", "#323538", 0.75, 0.45, 0.6, drier::LAMP_BLACK).engine3(drier::engine3::BONE_BLACK),
    ]
}

/// The named tubes from the catalog, in the order given. Unknown names panic
/// (boxes are fixed lists, checked by the tests).
fn pick(names: &[&str]) -> Vec<Tube> {
    let all = catalog();
    names.iter().map(|n| all.iter().find(|t| t.name == *n).unwrap_or_else(|| panic!("no tube {n:?} in the catalog")).clone()).collect()
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
    /// The share of turpentine (volatile solvent) the pile is thinned with
    /// (engine 4, see `Paint::solvent`).
    pub solvent: f32,
    /// The drying of the oil the paint is ground in, relative to linseed
    /// (1): walnut about 0.8, poppy about 0.6 (it also yellows least). From
    /// engine 6, relative to its tubes' own oil (`tube_oil_rate`): paint
    /// reground in poppy is `POPPY_RATE / tube_oil_rate`.
    pub oil_rate: f32,
    /// Engine 6: the pile's oil when its pigment packs and when an absorbent
    /// ground has drained it, relative to its own (`Paint::packed`,
    /// `Paint::floor`): its tubes', weighted by the oil each brings.
    pub packed: f32,
    pub floor: f32,
    /// Engine 6: the oil's share of the pile's volume as ground (its tubes',
    /// by volume: `Paint::oil_volume`).
    pub oil_volume: f32,
    /// The engine its palette paints with: from engine 6, blotting draws no
    /// more oil than an absorbent ground can (`Mixture::paint`).
    pub engine: u32,
    /// Engine 6: how fast its tubes' oils dry against linseed's, by volume
    /// (`Oil::rate`; 1 before), which `drying` already counts. `oil_rate` is
    /// relative to it from engine 6: 1 is the paint as its tubes come.
    pub tube_oil_rate: f32,
    /// Engine 6: the share of its oil from waxed tubes (`Paint::wax`).
    pub wax: f32,
}

/// The box a painting is painted from when nothing names another.
#[cfg(tube_box)]
pub const DEFAULT_BOX: &str = "tube box";

/// `DEFAULT_BOX`, if this build has it: a painter's build for one box has no
/// default box (a log naming no box is not its studio's) and no name for it.
pub fn default_box() -> Option<&'static str> {
    #[cfg(tube_box)]
    return Some(DEFAULT_BOX);
    #[cfg(not(tube_box))]
    None
}

/// The default box's tubes, in its order.
#[cfg(tube_box)]
const TUBE_BOX: &[&str] = &[
    "lead white", "smalt", "pale smalt", "yellow ochre", "red earth", "vermilion", "raw umber", "bone black", "cobalt blue", "chrome yellow", "Prussian blue", "green earth", "Rinmann's green", "copper green",
];

/// The other boxes: (name, tubes), each exactly the tubes its research note
/// documents (notes/r20/TUBES.md). Each is in the build only with its
/// `box-*` feature, so a painter's build names no other studio's box.
const BOXES: &[(&str, &[&str])] = &[
    #[cfg(feature = "box-sargent")]
    (
        "sargent",
        &[
            "lead white", "zinc white", "lemon chrome", "chrome yellow", "cadmium yellow", "Indian yellow", "yellow ochre", "Mars yellow", "raw sienna", "Mars orange", "red lead", "vermilion", "cadmium red", "Mars red",
            "red earth", "rose madder", "magenta", "burnt sienna", "Mars brown", "bone brown", "bone black", "cerulean blue", "cobalt blue", "ultramarine blue", "ultramarine ash", "viridian", "emerald green",
            "cobalt violet",
        ],
    ),
    #[cfg(feature = "box-inness")]
    (
        "inness",
        &[
            "lead white", "lemon chrome", "cadmium yellow", "yellow ochre", "raw sienna", "orange chrome", "red earth", "Indian red", "raw umber", "bitumen", "bone black", "cobalt blue", "Antwerp blue",
        ],
    ),
    #[cfg(feature = "box-alma-tadema")]
    (
        "alma-tadema",
        &[
            "lead white", "Naples yellow", "pale cadmium", "deep cadmium", "yellow ochre", "brown ochre", "orange vermilion", "Chinese vermilion", "red earth", "rose madder", "burnt sienna", "raw umber",
            "bone black", "cobalt blue", "viridian",
        ],
    ),
    #[cfg(feature = "box-tonn")]
    (
        "tonn",
        &[
            "lead white", "lead-tin yellow", "cadmium yellow", "yellow ochre", "transparent oxide yellow", "cadmium red", "permanent alizarin", "burnt sienna", "raw umber", "bone black", "ultramarine blue", "cerulean blue",
            "green earth", "cobalt violet",
        ],
    ),
    #[cfg(feature = "box-hopper")]
    (
        "hopper",
        &[
            "lead white", "zinc white", "pale cadmium", "cadmium yellow", "deep cadmium", "yellow ochre", "red earth", "burnt sienna", "bone black", "cerulean blue", "cobalt blue", "ultramarine blue", "viridian",
        ],
    ),
    // late Monet, as analyses of his paintings from about 1897 to 1926 found it
    // (notes/research/giverny_materials.md)
    #[cfg(feature = "box-giverny")]
    (
        "giverny",
        &[
            "lead white", "zinc white", "cobalt blue", "ultramarine blue", "cobalt violet", "viridian", "pale cadmium", "cadmium yellow", "deep cadmium", "barium yellow", "zinc yellow", "vermilion", "rose madder", "carmine lake", "yellow ochre",
        ],
    ),
    // what analyses found across the Impressionists, 1869 to the 1890s
    // (notes/research/impressionist_materials.md)
    #[cfg(feature = "box-impressionist")]
    (
        "impressionist",
        &[
            "lead white", "zinc white", "cobalt blue", "ultramarine blue", "cerulean blue", "Prussian blue", "emerald green", "viridian", "chrome yellow", "orange chrome", "barium yellow", "strontium yellow", "zinc yellow", "pale cadmium", "cadmium yellow", "deep cadmium", "Naples yellow", "Indian yellow", "yellow lake", "yellow ochre", "red earth", "raw sienna", "burnt sienna", "vermilion", "red lead", "rose madder", "carmine lake", "cobalt violet", "bone black", "vine black",
        ],
    ),
];

pub struct Palette {
    pub name: &'static str,
    pub tubes: Vec<Tube>,
    /// The engine version paint from this box is painted with (`ENGINE`;
    /// a replay sets its log's). A canvas prepared by a `Style` takes it.
    pub engine: u32,
    lat: Vec<[f32; mixbox::LATENT_SIZE]>,
    /// Scattering per coat of each tube paint.
    scat: Vec<f32>,
    /// Each tube's packed oil, drained floor and oil share by volume, in
    /// its box's period (`tube_packing`).
    pack: Vec<(f32, f32, f32)>,
    /// Each tube's oil and whether it holds wax, in its box (`grind_of`).
    grind: Vec<(Oil, bool)>,
}

impl Clone for Palette {
    fn clone(&self) -> Self {
        Palette { engine: self.engine, ..Palette::new(self.name, self.tubes.clone()) }
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
        let pack = tubes.iter().map(|t| tube_packing(t, name)).collect();
        let grind = tubes.iter().map(|t| grind_of(name, t.name)).collect();
        Palette { name, tubes, engine: crate::ENGINE, lat, scat, pack, grind }
    }

    /// Return a palette restricted to the named tubes. Unknown names panic.
    pub fn only(&self, names: &[&str]) -> Palette {
        let tubes = names
            .iter()
            .map(|n| self.tubes.iter().find(|t| t.name == *n).unwrap_or_else(|| panic!("no tube {n:?} in palette {}", self.name)).clone())
            .collect();
        Palette { engine: self.engine, ..Palette::new(self.name, tubes) }
    }

    /// Lead white, smalt (semi-transparent cobalt glass, weak) and pale
    /// smalt, yellow ochre, red earth, vermilion, raw umber, bone black.
    /// Naples yellow is not included. With green tubes added:
    /// `smalt_box_greens`.
    #[cfg(tube_box)]
    pub fn smalt_box() -> Self {
        Palette::new("smalt box", pick(&["lead white", "smalt", "pale smalt", "yellow ochre", "red earth", "vermilion", "raw umber", "bone black"]))
    }

    /// `smalt_box` without smalt (pale smalt stays), plus cobalt blue
    /// and chrome yellow. With green tubes added: `cobalt_box_greens`.
    #[cfg(tube_box)]
    pub fn cobalt_box() -> Self {
        let mut t = Palette::smalt_box().tubes;
        t.retain(|t| t.name != "smalt");
        t.extend(pick(&["cobalt blue", "chrome yellow"]));
        Palette::new("cobalt box", t)
    }

    /// Two tubes: Prussian blue and green earth (see `catalog` for their
    /// sources).
    #[cfg(tube_box)]
    pub fn green_tubes() -> Vec<Tube> {
        pick(&["Prussian blue", "green earth"])
    }

    /// `smalt_box` plus `green_tubes`: Prussian blue (hiding 0.35,
    /// stiffness 0.45, tinting strength 3) and green earth (hiding 0.2,
    /// stiffness 0.35, tinting strength 0.3).
    #[cfg(tube_box)]
    pub fn smalt_box_greens() -> Self {
        Palette::smalt_box().with(Palette::green_tubes()).named("smalt box, greens")
    }

    /// `cobalt_box` plus `green_tubes` and Rinmann's green (cobalt-zinc
    /// oxide: semi-transparent, weak, permanent [WEB-co]; hiding 0.35,
    /// stiffness 0.5, tinting strength 0.4; drying estimated as cobalt's).
    #[cfg(tube_box)]
    pub fn cobalt_box_greens() -> Self {
        let mut t = Palette::green_tubes();
        t.extend(pick(&["Rinmann's green"]));
        Palette::cobalt_box().with(t).named("cobalt box, greens")
    }

    /// The default box, `DEFAULT_BOX`: lead white, smalt, pale smalt, yellow
    /// ochre, red earth, vermilion, raw umber, bone black, cobalt blue,
    /// chrome yellow, Prussian blue, green earth, Rinmann's green and copper
    /// green. All were made by the 1820s (cobalt blue from 1802, chrome
    /// yellow sold in Germany from about 1820, Rinmann's green rare and
    /// costly [AP3; WEB-co]). A painting whose log names no box is painted
    /// from it.
    #[cfg(tube_box)]
    pub fn tube_box() -> Self {
        Palette::new(DEFAULT_BOX, pick(TUBE_BOX))
    }

    /// The same tubes under another name.
    pub fn named(self, name: &'static str) -> Palette {
        Palette { name, ..self }
    }

    /// A copper green (verdigris ground in oil); not in the standard
    /// palettes (add it with `with`). Masstone and numbers are assumptions;
    /// "poor hiding power in oil" [AP2 p.132]; copper is a drier (drying
    /// rate estimated as smalt's).
    #[cfg(tube_box)]
    pub fn copper_green() -> Tube {
        pick(&["copper green"]).remove(0)
    }

    /// The names of the boxes this build knows: the default first, then
    /// those its features include (the `box-*` features; the replay build
    /// has them all). A painter's build for one box knows only that box,
    /// not the default.
    pub fn box_names() -> Vec<&'static str> {
        let mut v = Vec::new();
        #[cfg(tube_box)]
        v.push(DEFAULT_BOX);
        v.extend(BOXES.iter().map(|b| b.0));
        v
    }

    /// The box a new painting takes when nothing names one: the default
    /// box, or in a painter's build for one box (no default box), that box.
    pub fn fallback_box() -> &'static str {
        Palette::box_names()[0]
    }

    /// The box called `name`, if this build knows it.
    pub fn named_box(name: &str) -> Option<Palette> {
        #[cfg(tube_box)]
        if name == DEFAULT_BOX {
            return Some(Palette::tube_box());
        }
        BOXES.iter().find(|b| b.0 == name).map(|b| Palette::new(b.0, pick(b.1)))
    }

    /// The tubes as a markdown table: name, pigment, hiding, stiffness,
    /// tinting strength and drying (the guide's "The tube box:" table).
    pub fn table(&self) -> String {
        let mut s = String::from("| tube | pigment | hiding | stiffness | tinting strength | drying |\n|---|---|---|---|---|---|\n");
        for t in &self.tubes {
            s.push_str(&format!("| {} | {} | {:?} | {:?} | {:?} | {:?} |\n", t.name, t.pigment, t.hiding, t.stiff, t.strength, self.drying_of(t)));
        }
        s
    }

    /// This palette with extra tubes appended.
    pub fn with(&self, extra: Vec<Tube>) -> Palette {
        let mut t = self.tubes.clone();
        t.extend(extra);
        Palette { engine: self.engine, ..Palette::new(self.name, t) }
    }

    /// How fast the tube `t` dries in this box's engine version.
    pub fn drying_of(&self, t: &Tube) -> f32 {
        if self.engine >= 3 { t.drying_3 } else { t.drying }
    }

    /// How fast this box's tube `i` dries: its pigment's rate (`drying_of`),
    /// and from engine 6 the oil it is ground in and its wax (`grind_of`).
    fn rate_of(&self, i: usize) -> f32 {
        let d = self.drying_of(&self.tubes[i]);
        if self.engine < 6 {
            return d;
        }
        let (oil, wax) = self.grind[i];
        d * oil.rate() * if wax { WAX_RATE } else { 1.0 }
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
        let drying = parts.iter().map(|&(i, f)| self.rate_of(i) * f).sum::<f32>() / parts.iter().map(|p| p.1).sum::<f32>().max(1e-9);
        // (engine 6: its tubes' oils, by volume, for `oil_rate`; the share
        // of its oil that is waxed)
        let six = self.engine >= 6;
        let tube_oil_rate = if six { parts.iter().map(|&(i, f)| self.grind[i].0.rate() * f).sum::<f32>() / parts.iter().map(|p| p.1).sum::<f32>().max(1e-9) } else { 1.0 };
        let waxed: f32 = parts.iter().filter(|&&(i, _)| self.grind[i].1).map(|&(i, f)| f * self.pack[i].2).sum();
        // each tube's share of the pile's oil: its volume times its oil share
        let (mut oil, mut packed, mut floor) = (0.0, 0.0, 0.0);
        for &(i, f) in &parts {
            let (p, fl, o) = self.pack[i];
            oil += f * o;
            packed += f * o * p;
            floor += f * o * fl;
        }
        let (packed, floor) = if oil > 1e-9 { (packed / oil, floor / oil) } else { (PACKED_OIL, DRAINED_FLOOR) };
        let total: f32 = parts.iter().map(|p| p.1).sum();
        let oil_volume = if total > 1e-9 && oil > 1e-9 { oil / total } else { OIL_VOLUME };
        let wax = if six && oil > 1e-9 { waxed / oil } else { 0.0 };
        Mixture { hiding: hiding_of(luminance(color), scatter), parts, color, scatter, stiff, drying, solvent: 0.0, oil_rate: 1.0, packed, floor, oil_volume, engine: self.engine, tube_oil_rate, wax }
    }

    /// How the parts (tube index, fraction by volume) look dry, in a pastel
    /// stick (engine 6): `pastel::dry_color` from each tube's masstone,
    /// scattering in oil and refractive index.
    pub fn dry_color(&self, parts: &[(usize, f32)]) -> crate::Rgb {
        let white = self.tubes.iter().position(|t| t.name == "lead white").map_or_else(|| self.scat.iter().cloned().fold(0.0, f32::max), |i| self.scat[i]);
        let list: Vec<(crate::Rgb, f32, f32, f32)> = parts
            .iter()
            .map(|&(i, f)| {
                let name = self.tubes[i].name;
                // engine 7: a black's colour comes from its carbon, which
                // stays on the particles it coats: its scattering rises in air
                // by the measured air/oil ratio (`pastel::carbon_ratio`), not
                // to a white's
                let n = match crate::pastel::carbon_ratio(name) {
                    Some(r) if self.engine >= 7 => -r,
                    _ => crate::pastel::refractive_index(name),
                };
                (self.tubes[i].color, self.scat[i], n, f)
            })
            .collect();
        crate::pastel::dry_color(&list, white)
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
        Mixture { solvent: m.solvent, oil_rate: m.oil_rate, ..self.mixture(parts) }
    }

    /// Human-readable recipe, e.g. "lead white 0.72 + yellow ochre 0.20 + raw umber 0.08".
    pub fn recipe(&self, m: &Mixture) -> String {
        m.parts.iter().map(|&(i, f)| format!("{} {:.2}", self.tubes[i].name, f)).collect::<Vec<_>>().join(" + ")
    }
}

impl Mixture {
    /// This mixture as paint on the brush, thinned with `medium` (0..1).
    pub fn paint(&self, medium: f32) -> Paint {
        // (a negative medium is oil drawn out of the paint, blotted: more
        // pigment to the volume, stiffer; at most half its oil. From engine
        // 6 no further than its drained floor: blotting paper draws the oil
        // by capillarity as an absorbent ground does, `bristle::ground_drain`)
        let medium = if self.engine >= 6 { medium.max(self.floor.min(1.0) - 1.0) } else { medium };
        let k = (1.0 - medium).clamp(0.0, 1.5);
        // medium dilutes the pigment: K and S per coat fall with the pigment
        // concentration, the masstone stays; the paint flows (stiffness
        // falls faster than hiding). The paint carries S itself: hiding
        // rounds to 1 for strong scatterers and would lose it.
        let p = Paint::km(self.color, self.scatter * k.max(1e-3), (self.stiff * k * k).min(1.0));
        // its oil relative to tube paint: medium adds oil, blotting draws
        // that share of it out (blot 0.5 leaves half)
        // (packed and drained oil stay the pigment's: medium and blotting
        // change only how much more or less the paint holds)
        Paint { solvent: self.solvent, oil: if medium < 0.0 { (1.0 + medium).max(0.1) } else { 1.0 + 1.5 * medium }, packed: self.packed, floor: self.floor, oil_volume: self.oil_volume, wax: self.wax, ..p }
    }

    /// This pile as paint on the brush, thinned with `medium` (0..1), drying
    /// at its tubes' rate (`drying`). Medium adds oil, which the drying
    /// model already counts (a fat film stays open longer).
    pub fn laid(&self, medium: f32) -> Paint {
        self.paint(medium).with_drying(self.drying * self.oil_rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A pile laid as knifed dries at its tubes' rate in its box's engine,
    /// mixed by volume: lead white fast, bone black slower (engine 2: its
    /// `Tube::drying`; engine 3: `Tube::drying_3`), half and half in
    /// between.
    #[test]
    #[cfg(tube_box)]
    fn a_pile_dries_at_its_tubes_rate() {
        for (engine, bone_black) in [(2, drier::BONE_BLACK), (3, 0.9)] {
            let mut pal = Palette::tube_box();
            pal.engine = engine;
            let at = |n: &str| pal.tubes.iter().position(|t| t.name == n).unwrap();
            let (w, k) = (at("lead white"), at("bone black"));
            let white = pal.pile(vec![(w, 1.0)]).laid(0.2);
            let black = pal.pile(vec![(k, 1.0)]).laid(0.2);
            let half = pal.pile(vec![(w, 0.5), (k, 0.5)]).laid(0.2);
            assert_eq!((white.drying, black.drying), (drier::LEAD_WHITE, bone_black), "engine {engine}");
            assert!((half.drying - 0.5 * (drier::LEAD_WHITE + bone_black)).abs() < 1e-6, "engine {engine}: {}", half.drying);
            assert_eq!(pal.with(vec![]).engine, engine, "a box with more tubes keeps its engine");
        }
        let pal = Palette::tube_box();
        let at = |n: &str| pal.tubes.iter().position(|t| t.name == n).unwrap();
        let (w, k) = (at("lead white"), at("bone black"));
        let half = pal.pile(vec![(w, 0.5), (k, 0.5)]).laid(0.2);
        // the paint is otherwise the pile's own
        let p = pal.pile(vec![(w, 0.5), (k, 0.5)]);
        assert_eq!((half.color, half.scatter, half.stiff), (p.paint(0.2).color, p.paint(0.2).scatter, p.paint(0.2).stiff));
    }

    /// The tube box is round 19's, tube for tube and number for number:
    /// paintings made from it replay bit for bit.
    #[test]
    #[cfg(tube_box)]
    fn the_tube_box_is_unchanged() {
        let r19: [(&str, &str, &str, f32, f32, f32, f32); 14] = [
            ("lead white", "basic lead carbonate", "#efe9dc", 0.82, 0.8, 1.0, 2.0),
            ("smalt", "cobalt potash glass, coarse", "#5a6e9e", 0.3, 0.55, 0.45, 1.6),
            ("pale smalt", "a paler grade of smalt", "#8d9bb8", 0.35, 0.55, 0.35, 1.6),
            ("yellow ochre", "hydrated iron oxide earth", "#b98a36", 0.8, 0.7, 0.8, 0.8),
            ("red earth", "iron oxide earth", "#9c4a30", 0.85, 0.7, 0.9, 1.0),
            ("vermilion", "mercuric sulfide", "#cf3a24", 0.9, 0.75, 1.0, 0.4),
            ("raw umber", "iron and manganese oxide earth", "#5c4c3a", 0.8, 0.65, 0.9, 2.4),
            ("bone black", "charred bone (carbon, calcium phosphate)", "#1e1b19", 0.9, 0.7, 1.1, 0.4),
            ("cobalt blue", "cobalt aluminate", "#2f55a8", 0.55, 0.6, 0.8, 1.4),
            ("chrome yellow", "lead chromate", "#e8b21c", 0.9, 0.7, 1.0, 1.8),
            ("Prussian blue", "iron ferrocyanide", "#172440", 0.35, 0.45, 3.0, 1.8),
            ("green earth", "celadonite and glauconite clay", "#3a4843", 0.2, 0.35, 0.3, 0.8),
            ("Rinmann's green", "cobalt-zinc oxide", "#5f8f76", 0.35, 0.5, 0.4, 1.4),
            ("copper green", "verdigris ground in oil", "#3f7f6a", 0.25, 0.4, 1.0, 1.6),
        ];
        let b = Palette::tube_box();
        assert_eq!(b.name, DEFAULT_BOX);
        assert_eq!(b.tubes.len(), r19.len());
        for (t, (name, pigment, color, hiding, stiff, strength, drying)) in b.tubes.iter().zip(r19) {
            assert_eq!((t.name, t.pigment), (name, pigment));
            assert_eq!(t.color.map(f32::to_bits), hex(color).map(f32::to_bits), "{name}");
            assert_eq!([t.hiding, t.stiff, t.strength, t.drying].map(f32::to_bits), [hiding, stiff, strength, drying].map(f32::to_bits), "{name}");
        }
        // and the older boxes built from the same definitions
        assert_eq!(Palette::named_box(DEFAULT_BOX).unwrap().tubes.len(), 14);
        let names = |p: Palette| p.tubes.iter().map(|t| t.name).collect::<Vec<_>>();
        assert_eq!(names(Palette::cobalt_box_greens()), ["lead white", "pale smalt", "yellow ochre", "red earth", "vermilion", "raw umber", "bone black", "cobalt blue", "chrome yellow", "Prussian blue", "green earth", "Rinmann's green"]);
        assert_eq!(names(Palette::smalt_box_greens()).len(), 10);
    }

    /// One definition per tube: no name twice in the catalog, and every
    /// tube's numbers in range.
    #[test]
    fn the_catalog_defines_each_tube_once() {
        let cat = catalog();
        let mut names: Vec<&str> = cat.iter().map(|t| t.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), cat.len(), "a tube is defined twice");
        for t in &cat {
            assert!(t.hiding > 0.0 && t.hiding <= 1.0 && t.stiff > 0.0 && t.stiff <= 1.0 && t.strength > 0.0 && t.drying > 0.0 && t.drying_3 > 0.0, "{t:?}");
            assert!(!t.pigment.is_empty(), "{}", t.name);
        }
    }

    /// The catalog holds exactly the tubes of this build's boxes: a
    /// painter's build for one box (`--no-default-features --features
    /// box-<name>`, see crates/easel/tests/painter.rs) carries no other tube.
    #[test]
    fn the_catalog_is_this_builds_boxes() {
        let mut want: Vec<&str> = Palette::box_names().into_iter().flat_map(|b| Palette::named_box(b).unwrap().tubes.into_iter().map(|t| t.name)).collect();
        want.sort();
        want.dedup();
        let mut have: Vec<&str> = catalog().iter().map(|t| t.name).collect();
        have.sort();
        assert_eq!(have, want);
    }

    /// Every box's tubes come from the catalog, each once, and a box is
    /// found by its name; an unknown name finds none.
    #[test]
    fn every_box_resolves_in_the_catalog() {
        let cat = catalog();
        for name in Palette::box_names() {
            let b = Palette::named_box(name).unwrap_or_else(|| panic!("box {name:?}"));
            assert_eq!(b.name, name);
            let mut seen: Vec<&str> = b.tubes.iter().map(|t| t.name).collect();
            seen.sort();
            seen.dedup();
            assert_eq!(seen.len(), b.tubes.len(), "{name}: a tube twice");
            for t in &b.tubes {
                let c = cat.iter().find(|c| c.name == t.name).unwrap();
                assert_eq!(format!("{c:?}"), format!("{t:?}"));
            }
        }
        assert!(Palette::named_box("no such box").is_none());
        #[cfg(feature = "all-boxes")]
        assert_eq!(Palette::box_names(), [DEFAULT_BOX, "sargent", "inness", "alma-tadema", "tonn", "hopper", "giverny", "impressionist"]);
    }

    /// Engine 6: each box's tubes hold their own oil: the default box's
    /// lead white is bladder paint (0.14 oil: packed at 10·0.86/14 of it,
    /// drained to 0.6 of that), Hopper's cobalt blue is Winsor & Newton's
    /// (22·0.53/47), Alma-Tadema's Naples yellow is held to `MAX_PACKED`,
    /// Tonn's lead white is Williamsburg's flake white (0.15), and a box
    /// without its own figure takes its period's (Hopper's cadmiums).
    #[test]
    #[cfg(any(tube_box, feature = "box-hopper", feature = "box-alma-tadema", feature = "box-tonn"))]
    fn tubes_hold_their_periods_oil() {
        let packing = |b: &str, name: &str| {
            let pal = Palette::named_box(b).unwrap();
            let i = pal.tubes.iter().position(|t| t.name == name).unwrap();
            (pal.pack[i].0, pal.pack[i].1)
        };
        let near = |a: (f32, f32), b: (f32, f32)| (a.0 - b.0).abs() < 1e-4 && (a.1 - b.1).abs() < 1e-4;
        #[cfg(tube_box)]
        {
            assert_eq!(period_of(DEFAULT_BOX), Period::Bladder);
            let lw = packing(DEFAULT_BOX, "lead white");
            assert!(near(lw, (8.6 / 14.0, 0.6 * 8.6 / 14.0)), "{lw:?}");
        }
        #[cfg(feature = "box-hopper")]
        {
            assert_eq!(period_of("hopper"), Period::Tube20);
            let cb = packing("hopper", "cobalt blue");
            assert!(near(cb, (22.0 * 0.53 / 47.0, 0.6 * 22.0 * 0.53 / 47.0)), "{cb:?}");
            // its cadmiums are extended with barium sulphate
            let cy = packing("hopper", "cadmium yellow");
            assert!(near(cy, (16.0 * 0.78 / 22.0, 0.75 * 16.0 * 0.78 / 22.0)), "{cy:?}");
        }
        #[cfg(feature = "box-alma-tadema")]
        {
            assert_eq!(period_of("alma-tadema"), Period::Tube19);
            assert!(near(packing("alma-tadema", "Naples yellow"), (MAX_PACKED, 0.5 * MAX_PACKED)));
        }
        #[cfg(feature = "box-tonn")]
        {
            assert_eq!(period_of("tonn"), Period::Modern);
            let lw = packing("tonn", "lead white");
            assert!(near(lw, (10.0 * 0.85 / 15.0, 0.6 * 10.0 * 0.85 / 15.0)), "{lw:?}");
        }
    }

    /// A pile's packed and drained oil are its tubes', weighted by the oil
    /// each brings (lead white is a stiff paste, burnt sienna an oily one,
    /// so half and half packs nearer the sienna's); medium and blotting
    /// leave them as they are (they are the pigment's). Its oil by volume
    /// is its tubes', by volume.
    #[test]
    #[cfg(feature = "box-sargent")]
    fn a_piles_packing_is_its_tubes_by_their_oil() {
        let pal = Palette::named_box("sargent").unwrap();
        let at = |n: &str| pal.tubes.iter().position(|t| t.name == n).unwrap();
        let (w, s) = (at("lead white"), at("burnt sienna"));
        let m = pal.pile(vec![(w, 0.5), (s, 0.5)]);
        let ((pw, fw, ow), (ps, fs, os)) = (pal.pack[w], pal.pack[s]);
        assert!(os > ow, "burnt sienna is the oilier paint: {os} {ow}");
        let want = ((pw * ow + ps * os) / (ow + os), (fw * ow + fs * os) / (ow + os));
        assert!((m.packed - want.0).abs() < 1e-5 && (m.floor - want.1).abs() < 1e-5, "{} {} vs {want:?}", m.packed, m.floor);
        assert!(m.packed < 0.5 * (pw + ps), "{} {pw} {ps}", m.packed);
        assert!((m.oil_volume - 0.5 * (ow + os)).abs() < 1e-6 && m.paint(0.3).oil_volume == m.oil_volume, "{}", m.oil_volume);
        for medium in [-0.3, 0.0, 0.5] {
            let p = m.paint(medium);
            assert_eq!((p.packed, p.floor), (m.packed, m.floor));
        }
    }

    /// Engine 6: each box's tubes come in their colourmen's oils, with their
    /// wax: Friedrich's in walnut, the Impressionists' cobalt in poppy and
    /// waxed, late Monet's lead white in linseed and unwaxed, Sargent's zinc
    /// white in poppy; a pile dries at its tubes' oils and wax (engine 5 at
    /// its pigments' rates alone) and carries the share of its oil that is
    /// waxed.
    #[test]
    #[cfg(feature = "all-boxes")]
    fn tubes_come_in_their_colourmen_s_oils() {
        assert_eq!(grind_of(DEFAULT_BOX, "lead white"), (Oil::Walnut, false));
        assert_eq!(grind_of("impressionist", "cobalt blue"), (Oil::Poppy, true));
        assert_eq!(grind_of("impressionist", "vermilion"), (Oil::Linseed, true));
        assert_eq!(grind_of("giverny", "lead white"), (Oil::Linseed, false));
        assert_eq!(grind_of("giverny", "cobalt blue"), (Oil::Poppy, false));
        assert_eq!(grind_of("sargent", "zinc white"), (Oil::Poppy, false));
        assert_eq!(grind_of("inness", "cobalt blue"), (Oil::Linseed, false));
        let mut pal = Palette::named_box("impressionist").unwrap();
        let i = pal.tubes.iter().position(|t| t.name == "cobalt blue").unwrap();
        let own = pal.drying_of(&pal.tubes[i]);
        let m = pal.pile(vec![(i, 1.0)]);
        assert!((m.drying - own * POPPY_RATE * WAX_RATE).abs() < 1e-6 && (m.tube_oil_rate - POPPY_RATE).abs() < 1e-6 && (m.wax - 1.0).abs() < 1e-6, "{} {} {}", m.drying, m.tube_oil_rate, m.wax);
        assert_eq!(m.laid(0.0).wax, 1.0);
        pal.engine = 5;
        let m = pal.pile(vec![(i, 1.0)]);
        assert!((m.drying - own).abs() < 1e-6 && m.tube_oil_rate == 1.0 && m.wax == 0.0);
    }

    /// From engine 6, blotting paper draws a paint's oil no lower than its
    /// drained floor, as an absorbent ground does: zinc white (fine, its
    /// floor 0.73 of its oil) blotted by half keeps 0.73; lead white (floor
    /// 0.39) is blotted the full half. Engine 5 blots both by half.
    #[test]
    #[cfg(feature = "box-sargent")]
    fn blotting_stops_at_the_drained_floor_from_engine_6() {
        let mut pal = Palette::named_box("sargent").unwrap();
        let at = |pal: &Palette, n: &str| pal.tubes.iter().position(|t| t.name == n).unwrap();
        let (zinc, lead) = (at(&pal, "zinc white"), at(&pal, "lead white"));
        let blotted = |pal: &Palette, i: usize| pal.pile(vec![(i, 1.0)]).paint(-0.5).oil;
        assert!(pal.pack[zinc].1 > 0.5 && pal.pack[lead].1 < 0.5);
        assert!((blotted(&pal, zinc) - pal.pack[zinc].1).abs() < 1e-6 && (blotted(&pal, lead) - 0.5).abs() < 1e-6);
        pal.engine = 5;
        assert!((blotted(&pal, zinc) - 0.5).abs() < 1e-6 && (blotted(&pal, lead) - 0.5).abs() < 1e-6);
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::*;
    use crate::pigment::Pigment;

    /// A prescribed dark pile deepens a light ground more than a dark one,
    /// with more darkening as the film thickens.
    #[test]
    #[cfg(tube_box)]
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
            Tube { name: "white", pigment: "", color: [0.99; 3], hiding: 0.99, stiff: 0.5, strength: 1.0, drying: 1.0, drying_3: 1.0, oa: 25.0, density: 3.0, drain: 0.9 },
            Tube { name: "black", pigment: "", color: [0.01; 3], hiding: 0.99, stiff: 0.5, strength: 1.0, drying: 1.0, drying_3: 1.0, oa: 25.0, density: 3.0, drain: 0.9 },
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
