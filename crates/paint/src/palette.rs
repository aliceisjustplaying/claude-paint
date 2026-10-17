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

use crate::color::{Rgb, hex, luminance};
use crate::drying::drier;
use crate::pigment::{hiding_of, scatter_for};
use crate::rng::Rng;
use crate::wet::Paint;

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
}

fn tube(name: &'static str, pigment: &'static str, color: &str, hiding: f32, stiff: f32, strength: f32, drying: f32) -> Tube {
    Tube { name, pigment, color: hex(color), hiding, stiff, strength, drying, drying_3: drying }
}

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
        #[cfg(any(tube_box, feature = "box-friedrich", feature = "box-every"))]
        tube("smalt", "cobalt potash glass, coarse", "#5a6e9e", 0.3, 0.55, 0.45, drier::SMALT),
        #[cfg(any(tube_box, feature = "box-friedrich", feature = "box-every"))]
        tube("pale smalt", "a paler grade of smalt", "#8d9bb8", 0.35, 0.55, 0.35, drier::SMALT),
        tube("yellow ochre", "hydrated iron oxide earth", "#b98a36", 0.8, 0.7, 0.8, drier::OCHRE),
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-inness", feature = "box-alma-tadema", feature = "box-hopper", feature = "box-impressionist", feature = "box-friedrich", feature = "box-seurat", feature = "box-cezanne", feature = "box-every"))]
        tube("red earth", "iron oxide earth", "#9c4a30", 0.85, 0.7, 0.9, drier::RED_EARTH),
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-giverny", feature = "box-impressionist", feature = "box-friedrich", feature = "box-seurat", feature = "box-cezanne", feature = "box-every"))]
        tube("vermilion", "mercuric sulfide", "#cf3a24", 0.9, 0.75, 1.0, drier::VERMILION),
        #[cfg(any(tube_box, feature = "box-inness", feature = "box-alma-tadema", feature = "box-tonn", feature = "box-friedrich", feature = "box-every"))]
        tube("raw umber", "iron and manganese oxide earth", "#5c4c3a", 0.8, 0.65, 0.9, drier::UMBER),
        // (every box but the giverny box, which has no black)
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-inness", feature = "box-alma-tadema", feature = "box-tonn", feature = "box-hopper", feature = "box-impressionist", feature = "box-friedrich", feature = "box-seurat", feature = "box-cezanne", feature = "box-every"))]
        tube("bone black", "charred bone (carbon, calcium phosphate)", "#1e1b19", 0.9, 0.7, 1.1, drier::BONE_BLACK).engine3(drier::engine3::BONE_BLACK),
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-inness", feature = "box-alma-tadema", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist", feature = "box-friedrich", feature = "box-seurat", feature = "box-cezanne", feature = "box-every"))]
        tube("cobalt blue", "cobalt aluminate", "#2f55a8", 0.55, 0.6, 0.8, drier::COBALT_BLUE).engine3(drier::engine3::COBALT_BLUE),
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-impressionist", feature = "box-friedrich", feature = "box-seurat", feature = "box-cezanne", feature = "box-every"))]
        tube("chrome yellow", "lead chromate", "#e8b21c", 0.9, 0.7, 1.0, drier::CHROME_YELLOW),
        // Prussian blue transparent and very strong [AP3 pp.196–197]
        // (tinting strength 3, below the sourced "very high", because
        // Mixbox's latent already carries some of a dark pigment's strength)
        #[cfg(any(tube_box, feature = "box-impressionist", feature = "box-friedrich", feature = "box-every"))]
        tube("Prussian blue", "iron ferrocyanide", "#172440", 0.35, 0.45, 3.0, drier::PRUSSIAN_BLUE).engine3(drier::engine3::PRUSSIAN_BLUE),
        // green earth translucent, weak, short of body [AP1 p.146; FIELD
        // p.129], its masstone from Munsell 7.5G/2.9/1.5 [AP1 Table 1]; its
        // drying rate is an estimate (an earth: medium)
        #[cfg(any(tube_box, feature = "box-tonn", feature = "box-friedrich", feature = "box-every"))]
        tube("green earth", "celadonite and glauconite clay", "#3a4843", 0.2, 0.35, 0.3, drier::OCHRE),
        // cobalt-zinc oxide: semi-transparent, weak, permanent [WEB-co];
        // drying estimated as cobalt's
        #[cfg(any(tube_box, feature = "box-friedrich", feature = "box-every"))]
        tube("Rinmann's green", "cobalt-zinc oxide", "#5f8f76", 0.35, 0.5, 0.4, drier::COBALT_BLUE),
        // verdigris ground in oil: "poor hiding power in oil" [AP2 p.132];
        // copper is a drier (drying rate estimated as smalt's)
        #[cfg(any(tube_box, feature = "box-friedrich", feature = "box-every"))]
        tube("copper green", "verdigris ground in oil", "#3f7f6a", 0.25, 0.4, 1.0, drier::SMALT),
        // ---- round 20 (notes/r20/TUBES.md)
        #[cfg(any(feature = "box-sargent", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist", feature = "box-every"))]
        tube("zinc white", "zinc oxide", "#f3f3ef", 0.6, 0.6, 1.0, drier::ZINC_WHITE),
        #[cfg(any(feature = "box-tonn", feature = "box-every"))]
        tube("lead-tin yellow", "lead-tin oxide", "#e3cc6a", 0.85, 0.75, 0.6, drier::LEAD_WHITE),
        #[cfg(any(feature = "box-alma-tadema", feature = "box-impressionist", feature = "box-friedrich", feature = "box-every"))]
        tube("Naples yellow", "lead antimonate", "#e2b964", 0.85, 0.75, 0.6, drier::NAPLES_YELLOW),
        #[cfg(any(feature = "box-sargent", feature = "box-inness", feature = "box-every"))]
        tube("lemon chrome", "pale lead chromate with lead sulfate", "#eed83c", 0.8, 0.7, 0.8, drier::CHROME_YELLOW),
        #[cfg(any(feature = "box-alma-tadema", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist", feature = "box-every"))]
        tube("pale cadmium", "cadmium sulfide, a pale grade", "#f0c63c", 0.85, 0.7, 1.1, drier::CADMIUM).engine3(drier::engine3::CADMIUM),
        #[cfg(any(feature = "box-alma-tadema", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist", feature = "box-every"))]
        tube("deep cadmium", "cadmium sulfide, a deep grade", "#e8861e", 0.9, 0.6, 1.2, drier::CADMIUM).engine3(drier::engine3::CADMIUM),
        #[cfg(any(feature = "box-sargent", feature = "box-inness", feature = "box-tonn", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist", feature = "box-seurat", feature = "box-every"))]
        tube("cadmium yellow", "cadmium sulfide", "#e8a51f", 0.85, 0.7, 1.1, drier::CADMIUM).engine3(drier::engine3::CADMIUM),
        #[cfg(any(feature = "box-sargent", feature = "box-impressionist", feature = "box-every"))]
        tube("Indian yellow", "magnesium and calcium euxanthate", "#e1a11e", 0.15, 0.4, 0.8, drier::INDIAN_YELLOW),
        #[cfg(any(feature = "box-sargent", feature = "box-every"))]
        tube("Mars yellow", "synthetic iron oxide hydroxide", "#c4872b", 0.85, 0.7, 1.1, drier::MARS),
        #[cfg(any(feature = "box-tonn", feature = "box-every"))]
        tube("transparent oxide yellow", "transparent synthetic iron oxide", "#7a4a14", 0.2, 0.5, 0.9, drier::RED_EARTH),
        #[cfg(any(feature = "box-alma-tadema", feature = "box-every"))]
        tube("brown ochre", "iron oxide earth, a darker grade", "#86592e", 0.8, 0.7, 0.8, drier::OCHRE),
        #[cfg(any(feature = "box-sargent", feature = "box-inness", feature = "box-impressionist", feature = "box-every"))]
        tube("raw sienna", "sienna earth, unroasted", "#9a6a2b", 0.4, 0.5, 0.7, drier::SIENNA),
        #[cfg(any(feature = "box-inness", feature = "box-impressionist", feature = "box-every"))]
        tube("orange chrome", "basic lead chromate", "#e0712a", 0.88, 0.75, 0.9, drier::CHROME_YELLOW),
        // Mars orange: "much transparency" in the period account (Salter's
        // Field, 1869), so less hiding than the other Mars tubes
        #[cfg(any(feature = "box-sargent", feature = "box-every"))]
        tube("Mars orange", "synthetic iron oxide, an orange grade", "#b8602a", 0.5, 0.7, 1.1, drier::MARS),
        #[cfg(any(feature = "box-sargent", feature = "box-impressionist", feature = "box-every"))]
        tube("red lead", "lead tetroxide", "#e0542b", 0.85, 0.8, 0.8, drier::RED_LEAD),
        #[cfg(any(feature = "box-alma-tadema", feature = "box-every"))]
        tube("orange vermilion", "mercuric sulfide, a yellower grade", "#dd4a22", 0.9, 0.75, 1.0, drier::VERMILION),
        #[cfg(any(feature = "box-alma-tadema", feature = "box-every"))]
        tube("Chinese vermilion", "mercuric sulfide, a deeper grade", "#b8282e", 0.9, 0.75, 1.0, drier::VERMILION),
        #[cfg(any(feature = "box-sargent", feature = "box-tonn", feature = "box-every"))]
        tube("cadmium red", "cadmium sulfoselenide", "#c3321f", 0.9, 0.7, 1.1, drier::CADMIUM).engine3(drier::engine3::CADMIUM),
        #[cfg(any(feature = "box-sargent", feature = "box-every"))]
        tube("Mars red", "synthetic iron oxide", "#a33f2a", 0.9, 0.7, 1.2, drier::MARS),
        #[cfg(any(feature = "box-inness", feature = "box-every"))]
        tube("Indian red", "nearly pure ferric oxide", "#7a3a33", 0.92, 0.7, 1.2, drier::RED_EARTH),
        #[cfg(any(feature = "box-sargent", feature = "box-alma-tadema", feature = "box-giverny", feature = "box-impressionist", feature = "box-friedrich", feature = "box-seurat", feature = "box-cezanne", feature = "box-every"))]
        tube("rose madder", "madder lake on alumina", "#8e2238", 0.1, 0.35, 0.9, drier::MADDER_LAKE).engine3(drier::engine3::ALIZARIN),
        #[cfg(any(feature = "box-tonn", feature = "box-every"))]
        tube("permanent alizarin", "a quinacridone", "#5e1624", 0.15, 0.45, 1.3, drier::MADDER_LAKE).engine3(drier::engine3::ALIZARIN),
        // an aniline dye laked on alumina: transparent, strong, at madder
        // lake's rate; it fades in light, which the engine doesn't model
        #[cfg(any(feature = "box-sargent", feature = "box-every"))]
        tube("magenta", "fuchsine (aniline) lake on alumina", "#8f1650", 0.1, 0.35, 1.5, drier::MADDER_LAKE),
        #[cfg(any(feature = "box-sargent", feature = "box-alma-tadema", feature = "box-tonn", feature = "box-hopper", feature = "box-impressionist", feature = "box-every"))]
        tube("burnt sienna", "roasted sienna earth", "#7c3f24", 0.45, 0.55, 0.9, drier::SIENNA).engine3(drier::engine3::BURNT_SIENNA),
        // Mars brown at sienna's rate: iron oxides dry well but lack umber's
        // manganese (notes/r20/TUBES.md)
        #[cfg(any(feature = "box-sargent", feature = "box-every"))]
        tube("Mars brown", "synthetic iron oxide, roasted", "#5a3a28", 0.85, 0.65, 1.0, drier::SIENNA),
        #[cfg(any(feature = "box-sargent", feature = "box-every"))]
        tube("bone brown", "bone roasted until brown", "#4b3527", 0.6, 0.6, 0.9, drier::BONE_BROWN),
        #[cfg(any(feature = "box-inness", feature = "box-every"))]
        tube("bitumen", "asphaltum", "#2e2017", 0.12, 0.3, 0.7, drier::BITUMEN),
        #[cfg(any(feature = "box-sargent", feature = "box-tonn", feature = "box-hopper", feature = "box-impressionist", feature = "box-every"))]
        tube("cerulean blue", "cobalt stannate", "#3f82b3", 0.8, 0.7, 0.6, drier::COBALT_BLUE),
        #[cfg(any(feature = "box-sargent", feature = "box-tonn", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist", feature = "box-seurat", feature = "box-cezanne", feature = "box-every"))]
        tube("ultramarine blue", "synthetic ultramarine", "#232a8c", 0.3, 0.5, 1.1, drier::ULTRAMARINE).engine3(drier::engine3::ULTRAMARINE),
        // the last, palest extraction of natural ultramarine: mostly
        // colorless matter, so weak and transparent
        #[cfg(any(feature = "box-sargent", feature = "box-every"))]
        tube("ultramarine ash", "natural ultramarine, a pale last extraction", "#7d8aa8", 0.15, 0.5, 0.3, drier::ULTRAMARINE).engine3(drier::engine3::ULTRAMARINE),
        #[cfg(any(feature = "box-inness", feature = "box-every"))]
        tube("Antwerp blue", "Prussian blue on an alumina base", "#26406c", 0.4, 0.45, 1.6, drier::ANTWERP_BLUE),
        #[cfg(any(feature = "box-sargent", feature = "box-alma-tadema", feature = "box-hopper", feature = "box-giverny", feature = "box-impressionist", feature = "box-seurat", feature = "box-cezanne", feature = "box-every"))]
        tube("viridian", "hydrated chromium oxide", "#1c4a40", 0.3, 0.5, 0.9, drier::VIRIDIAN),
        #[cfg(any(feature = "box-sargent", feature = "box-impressionist", feature = "box-seurat", feature = "box-cezanne", feature = "box-every"))]
        tube("emerald green", "copper aceto-arsenite", "#23a57a", 0.6, 0.6, 0.6, drier::COPPER),
        // cobalt pigments are siccative in oil; set at cobalt blue's rate
        #[cfg(any(feature = "box-sargent", feature = "box-tonn", feature = "box-giverny", feature = "box-impressionist", feature = "box-every"))]
        tube("cobalt violet", "cobalt phosphate or arsenate", "#7e4c8e", 0.35, 0.55, 0.35, drier::COBALT_BLUE),
        // lead-free chromate yellows of the 1850s-80s, as reconstructed from
        // historical recipes and measured (Otero et al. 2017, Heritage
        // Science 5:46): the pigments' color L*a*b* 94/-11/55, 90/-8/52,
        // 87/5/89, and tinting strength in test paints (PVA, with barium
        // sulfate) 78%, 92%, 65% of lead chromate's; masstones darkened for
        // oil (this easel's adjustment); hiding from their refractive indices
        #[cfg(any(feature = "box-impressionist", feature = "box-seurat", feature = "box-every"))]
        tube("strontium yellow", "strontium chromate", "#f2df53", 0.55, 0.65, 0.8, drier::CHROMATE),
        #[cfg(any(feature = "box-giverny", feature = "box-impressionist", feature = "box-every"))]
        tube("barium yellow", "barium chromate (lemon yellow)", "#efd75c", 0.45, 0.7, 0.9, drier::CHROMATE),
        // zinc yellow darkens with time (to dichromate brown or Cr2O3 green)
        #[cfg(any(feature = "box-giverny", feature = "box-impressionist", feature = "box-seurat", feature = "box-every"))]
        tube("zinc yellow", "potassium zinc chromate", "#fcc400", 0.4, 0.6, 0.65, drier::ZINC_YELLOW),
        // cochineal lake: found with madder in late-19th-c. French paint
        // (Pozzi et al. 2014); fugitive. Estimates
        #[cfg(any(feature = "box-giverny", feature = "box-impressionist", feature = "box-every"))]
        tube("carmine lake", "carminic acid (cochineal) on alumina", "#861c3c", 0.1, 0.3, 1.2, drier::MADDER_LAKE).engine3(drier::engine3::ALIZARIN),
        // flavonoid yellow lake (Butler 1984; NG TB 24); fugitive. Estimates
        #[cfg(any(feature = "box-impressionist", feature = "box-every"))]
        tube("yellow lake", "flavonoid dye (weld, quercitron) on alumina and chalk", "#9e7525", 0.08, 0.35, 0.5, drier::MADDER_LAKE).engine3(drier::engine3::ALIZARIN),
        // Engine-3 drying follows the medium carbon-black class; estimate.
        // (A stroke touch-dry in about 4.9 days, by the drying test's film;
        // engine 2's rate alone would take about 12.6 at engine 3's pace,
        // 5.0 in engine 2 itself.)
        // charcoal black: bluish, weak, without bone's phosphate (Butler
        // 1984 found it in 9 of 10 paintings examined). Estimates
        #[cfg(any(feature = "box-impressionist", feature = "box-every"))]
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
    /// (1): walnut about 0.8, poppy about 0.6 (it also yellows least).
    pub oil_rate: f32,
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
    #[cfg(feature = "box-friedrich")]
    (
        "friedrich",
        &[
            "lead white", "smalt", "pale smalt", "yellow ochre", "red earth", "vermilion", "raw umber", "bone black", "cobalt blue", "chrome yellow", "Prussian blue", "green earth", "Rinmann's green", "copper green", "Naples yellow", "rose madder",
        ],
    ),
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
    // late Monet, as analyses of his paintings from 1887 to 1926 found it
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
    #[cfg(feature = "box-seurat")]
    (
        "seurat",
        &[
            "lead white", "chrome yellow", "cadmium yellow", "strontium yellow", "zinc yellow", "vermilion", "rose madder", "yellow ochre", "red earth", "bone black", "cobalt blue", "ultramarine blue", "viridian", "emerald green",
        ],
    ),
    #[cfg(feature = "box-cezanne")]
    (
        "cezanne",
        &[
            "lead white", "chrome yellow", "vermilion", "rose madder", "yellow ochre", "red earth", "bone black", "cobalt blue", "ultramarine blue", "viridian", "emerald green",
        ],
    ),
    // every tube in the catalog (round 31: "all the colors")
    #[cfg(feature = "box-every")]
    (
        "every",
        &[
            "lead white", "smalt", "pale smalt", "yellow ochre", "red earth", "vermilion", "raw umber", "bone black", "cobalt blue", "chrome yellow", "Prussian blue", "green earth", "Rinmann's green", "copper green", "zinc white", "lead-tin yellow", "Naples yellow", "lemon chrome", "pale cadmium", "deep cadmium", "cadmium yellow", "Indian yellow", "Mars yellow", "transparent oxide yellow", "brown ochre", "raw sienna", "orange chrome", "Mars orange", "red lead", "orange vermilion", "Chinese vermilion", "cadmium red", "Mars red", "Indian red", "rose madder", "permanent alizarin", "magenta", "burnt sienna", "Mars brown", "bone brown", "bitumen", "cerulean blue", "ultramarine blue", "ultramarine ash", "Antwerp blue", "viridian", "emerald green", "cobalt violet", "strontium yellow", "barium yellow", "zinc yellow", "carmine lake", "yellow lake", "vine black",
        ],
    ),
];

pub struct Palette {
    pub name: &'static str,
    pub tubes: Vec<Tube>,
    /// The engine version paint from this box is painted with (`ENGINE`;
    /// a replay sets its log's). A canvas prepared by a `Style` takes it.
    pub engine: u32,
    pub quarter_tiles: bool,
    pub adaptive_sweep: bool,
    /// `body_of` joins a limb to the nearest other limb, and only when it
    /// starts within its own width of it (`Outline::body`): new paintings.
    pub limbs_join_nearest: bool,
    pub knife_skim: bool,
    /// Engine 7: a blender lays wet paint only where wet paint was, and a
    /// little way past it (`Canvas::with_blend_feather`): new paintings.
    pub blend_feather: bool,
    /// Engine 7: a brush's paint laid on wet paint stays on top of it
    /// (`Canvas::with_brush_skim`): new paintings.
    pub brush_skim: bool,
    lat: Vec<[f32; mixbox::LATENT_SIZE]>,
    /// Scattering per coat of each tube paint.
    scat: Vec<f32>,
}

impl Clone for Palette {
    fn clone(&self) -> Self {
        Palette { engine: self.engine, quarter_tiles: self.quarter_tiles, adaptive_sweep: self.adaptive_sweep, limbs_join_nearest: self.limbs_join_nearest, knife_skim: self.knife_skim, blend_feather: self.blend_feather, brush_skim: self.brush_skim, ..Palette::new(self.name, self.tubes.clone()) }
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
        Palette { name, tubes, engine: crate::ENGINE, quarter_tiles: false, adaptive_sweep: false, limbs_join_nearest: false, knife_skim: false, blend_feather: false, brush_skim: false, lat, scat }
    }

    /// Return a palette restricted to the named tubes. Unknown names panic.
    pub fn only(&self, names: &[&str]) -> Palette {
        let tubes = names
            .iter()
            .map(|n| self.tubes.iter().find(|t| t.name == *n).unwrap_or_else(|| panic!("no tube {n:?} in palette {}", self.name)).clone())
            .collect();
        Palette { engine: self.engine, quarter_tiles: self.quarter_tiles, adaptive_sweep: self.adaptive_sweep, limbs_join_nearest: self.limbs_join_nearest, knife_skim: self.knife_skim, blend_feather: self.blend_feather, brush_skim: self.brush_skim, ..Palette::new(self.name, tubes) }
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
        Palette { engine: self.engine, quarter_tiles: self.quarter_tiles, adaptive_sweep: self.adaptive_sweep, limbs_join_nearest: self.limbs_join_nearest, knife_skim: self.knife_skim, blend_feather: self.blend_feather, brush_skim: self.brush_skim, ..Palette::new(self.name, t) }
    }

    /// How fast the tube `t` dries in this box's engine version.
    pub fn drying_of(&self, t: &Tube) -> f32 {
        if self.engine >= 3 { t.drying_3 } else { t.drying }
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
        let drying = parts.iter().map(|&(i, f)| self.drying_of(&self.tubes[i]) * f).sum::<f32>() / parts.iter().map(|p| p.1).sum::<f32>().max(1e-9);
        Mixture { hiding: hiding_of(luminance(color), scatter), parts, color, scatter, stiff, drying, solvent: 0.0, oil_rate: 1.0 }
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
        // pigment to the volume, stiffer; at most half its oil)
        let k = (1.0 - medium).clamp(0.0, 1.5);
        // medium dilutes the pigment: K and S per coat fall with the pigment
        // concentration, the masstone stays; the paint flows (stiffness
        // falls faster than hiding). The paint carries S itself: hiding
        // rounds to 1 for strong scatterers and would lose it.
        let p = Paint::km(self.color, self.scatter * k.max(1e-3), (self.stiff * k * k).min(1.0));
        // its oil relative to tube paint: medium adds oil, blotting draws
        // that share of it out (blot 0.5 leaves half)
        Paint { solvent: self.solvent, oil: if medium < 0.0 { (1.0 + medium).max(0.1) } else { 1.0 + 1.5 * medium }, ..p }
    }

    /// This pile as paint on the brush, thinned with `medium` (0..1), drying
    /// at its tubes' rate (`drying`). Medium adds oil, which the drying
    /// model already counts (a fat film stays open longer).
    pub fn laid(&self, medium: f32) -> Paint {
        self.paint(medium).with_drying(self.drying * self.oil_rate)
    }
}
