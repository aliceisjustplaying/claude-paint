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

fn tube(name: &'static str, pigment: &'static str, color: &str, hiding: f32, stiff: f32, strength: f32, drying: f32) -> Tube {
    Tube { name, pigment, color: hex(color), hiding, stiff, strength, drying }
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
/// come from and why. All numbers are estimates from the pigment literature,
/// not measurements.
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
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-inness", feature = "box-alma-tadema", feature = "box-hopper"))]
        tube("red earth", "iron oxide earth", "#9c4a30", 0.85, 0.7, 0.9, drier::RED_EARTH),
        #[cfg(any(tube_box, feature = "box-sargent"))]
        tube("vermilion", "mercuric sulfide", "#cf3a24", 0.9, 0.75, 1.0, drier::VERMILION),
        #[cfg(any(tube_box, feature = "box-inness", feature = "box-alma-tadema", feature = "box-tonn"))]
        tube("raw umber", "iron and manganese oxide earth", "#5c4c3a", 0.8, 0.65, 0.9, drier::UMBER),
        tube("bone black", "charred bone (carbon, calcium phosphate)", "#1e1b19", 0.9, 0.7, 1.1, drier::BONE_BLACK),
        #[cfg(any(tube_box, feature = "box-sargent", feature = "box-inness", feature = "box-alma-tadema", feature = "box-hopper"))]
        tube("cobalt blue", "cobalt aluminate", "#2f55a8", 0.55, 0.6, 0.8, drier::COBALT_BLUE),
        #[cfg(any(tube_box, feature = "box-sargent"))]
        tube("chrome yellow", "lead chromate", "#e8b21c", 0.9, 0.7, 1.0, drier::CHROME_YELLOW),
        // Prussian blue transparent and very strong [AP3 pp.196–197]
        // (tinting strength 3, below the sourced "very high", because
        // Mixbox's latent already carries some of a dark pigment's strength)
        #[cfg(tube_box)]
        tube("Prussian blue", "iron ferrocyanide", "#172440", 0.35, 0.45, 3.0, drier::PRUSSIAN_BLUE),
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
        #[cfg(any(feature = "box-sargent", feature = "box-hopper"))]
        tube("zinc white", "zinc oxide", "#f3f3ef", 0.6, 0.6, 1.0, drier::ZINC_WHITE),
        #[cfg(feature = "box-tonn")]
        tube("lead-tin yellow", "lead-tin oxide", "#e3cc6a", 0.85, 0.75, 0.6, drier::LEAD_WHITE),
        #[cfg(feature = "box-alma-tadema")]
        tube("Naples yellow", "lead antimonate", "#e2b964", 0.85, 0.75, 0.6, drier::NAPLES_YELLOW),
        #[cfg(any(feature = "box-sargent", feature = "box-inness"))]
        tube("lemon chrome", "pale lead chromate with lead sulfate", "#eed83c", 0.8, 0.7, 0.8, drier::CHROME_YELLOW),
        #[cfg(any(feature = "box-alma-tadema", feature = "box-hopper"))]
        tube("pale cadmium", "cadmium sulfide, a pale grade", "#f0c63c", 0.85, 0.7, 1.1, drier::CADMIUM),
        #[cfg(any(feature = "box-alma-tadema", feature = "box-hopper"))]
        tube("deep cadmium", "cadmium sulfide, a deep grade", "#e8861e", 0.9, 0.6, 1.2, drier::CADMIUM),
        #[cfg(any(feature = "box-sargent", feature = "box-inness", feature = "box-tonn", feature = "box-hopper"))]
        tube("cadmium yellow", "cadmium sulfide", "#e8a51f", 0.85, 0.7, 1.1, drier::CADMIUM),
        #[cfg(feature = "box-sargent")]
        tube("Indian yellow", "magnesium and calcium euxanthate", "#e1a11e", 0.15, 0.4, 0.8, drier::INDIAN_YELLOW),
        #[cfg(feature = "box-sargent")]
        tube("Mars yellow", "synthetic iron oxide hydroxide", "#c4872b", 0.85, 0.7, 1.1, drier::MARS),
        #[cfg(feature = "box-tonn")]
        tube("transparent oxide yellow", "transparent synthetic iron oxide", "#7a4a14", 0.2, 0.5, 0.9, drier::RED_EARTH),
        #[cfg(feature = "box-alma-tadema")]
        tube("brown ochre", "iron oxide earth, a darker grade", "#86592e", 0.8, 0.7, 0.8, drier::OCHRE),
        #[cfg(any(feature = "box-sargent", feature = "box-inness"))]
        tube("raw sienna", "sienna earth, unroasted", "#9a6a2b", 0.4, 0.5, 0.7, drier::SIENNA),
        #[cfg(feature = "box-inness")]
        tube("orange chrome", "basic lead chromate", "#e0712a", 0.88, 0.75, 0.9, drier::CHROME_YELLOW),
        // Mars orange: "much transparency" in the period account (Salter's
        // Field, 1869), so less hiding than the other Mars tubes
        #[cfg(feature = "box-sargent")]
        tube("Mars orange", "synthetic iron oxide, an orange grade", "#b8602a", 0.5, 0.7, 1.1, drier::MARS),
        #[cfg(feature = "box-sargent")]
        tube("red lead", "lead tetroxide", "#e0542b", 0.85, 0.8, 0.8, drier::RED_LEAD),
        #[cfg(feature = "box-alma-tadema")]
        tube("orange vermilion", "mercuric sulfide, a yellower grade", "#dd4a22", 0.9, 0.75, 1.0, drier::VERMILION),
        #[cfg(feature = "box-alma-tadema")]
        tube("Chinese vermilion", "mercuric sulfide, a deeper grade", "#b8282e", 0.9, 0.75, 1.0, drier::VERMILION),
        #[cfg(any(feature = "box-sargent", feature = "box-tonn"))]
        tube("cadmium red", "cadmium sulfoselenide", "#c3321f", 0.9, 0.7, 1.1, drier::CADMIUM),
        #[cfg(feature = "box-sargent")]
        tube("Mars red", "synthetic iron oxide", "#a33f2a", 0.9, 0.7, 1.2, drier::MARS),
        #[cfg(feature = "box-inness")]
        tube("Indian red", "nearly pure ferric oxide", "#7a3a33", 0.92, 0.7, 1.2, drier::RED_EARTH),
        #[cfg(any(feature = "box-sargent", feature = "box-alma-tadema"))]
        tube("rose madder", "madder lake on alumina", "#8e2238", 0.1, 0.35, 0.9, drier::MADDER_LAKE),
        #[cfg(feature = "box-tonn")]
        tube("permanent alizarin", "a quinacridone", "#5e1624", 0.15, 0.45, 1.3, drier::MADDER_LAKE),
        // an aniline dye laked on alumina: transparent, strong, at madder
        // lake's rate; it fades in light, which the engine doesn't model
        #[cfg(feature = "box-sargent")]
        tube("magenta", "fuchsine (aniline) lake on alumina", "#8f1650", 0.1, 0.35, 1.5, drier::MADDER_LAKE),
        #[cfg(any(feature = "box-sargent", feature = "box-alma-tadema", feature = "box-tonn", feature = "box-hopper"))]
        tube("burnt sienna", "roasted sienna earth", "#7c3f24", 0.45, 0.55, 0.9, drier::SIENNA),
        // Mars brown at sienna's rate: iron oxides dry well but lack umber's
        // manganese (notes/r20/TUBES.md)
        #[cfg(feature = "box-sargent")]
        tube("Mars brown", "synthetic iron oxide, roasted", "#5a3a28", 0.85, 0.65, 1.0, drier::SIENNA),
        #[cfg(feature = "box-sargent")]
        tube("bone brown", "bone roasted until brown", "#4b3527", 0.6, 0.6, 0.9, drier::BONE_BROWN),
        #[cfg(feature = "box-inness")]
        tube("bitumen", "asphaltum", "#2e2017", 0.12, 0.3, 0.7, drier::BITUMEN),
        #[cfg(any(feature = "box-sargent", feature = "box-tonn", feature = "box-hopper"))]
        tube("cerulean blue", "cobalt stannate", "#3f82b3", 0.8, 0.7, 0.6, drier::COBALT_BLUE),
        #[cfg(any(feature = "box-sargent", feature = "box-tonn", feature = "box-hopper"))]
        tube("ultramarine blue", "synthetic ultramarine", "#232a8c", 0.3, 0.5, 1.1, drier::ULTRAMARINE),
        // the last, palest extraction of natural ultramarine: mostly
        // colorless matter, so weak and transparent
        #[cfg(feature = "box-sargent")]
        tube("ultramarine ash", "natural ultramarine, a pale last extraction", "#7d8aa8", 0.15, 0.5, 0.3, drier::ULTRAMARINE),
        #[cfg(feature = "box-inness")]
        tube("Antwerp blue", "Prussian blue on an alumina base", "#26406c", 0.4, 0.45, 1.6, drier::ANTWERP_BLUE),
        #[cfg(any(feature = "box-sargent", feature = "box-alma-tadema", feature = "box-hopper"))]
        tube("viridian", "hydrated chromium oxide", "#1c4a40", 0.3, 0.5, 0.9, drier::VIRIDIAN),
        #[cfg(feature = "box-sargent")]
        tube("emerald green", "copper aceto-arsenite", "#23a57a", 0.6, 0.6, 0.6, drier::COPPER),
        // cobalt pigments are siccative in oil; set at cobalt blue's rate
        #[cfg(any(feature = "box-sargent", feature = "box-tonn"))]
        tube("cobalt violet", "cobalt phosphate or arsenate", "#7e4c8e", 0.35, 0.55, 0.35, drier::COBALT_BLUE),
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
        Palette { name, tubes, engine: crate::ENGINE, lat, scat }
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
            s.push_str(&format!("| {} | {} | {:?} | {:?} | {:?} | {:?} |\n", t.name, t.pigment, t.hiding, t.stiff, t.strength, t.drying));
        }
        s
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
    #[cfg(tube_box)]
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
            assert!(t.hiding > 0.0 && t.hiding <= 1.0 && t.stiff > 0.0 && t.stiff <= 1.0 && t.strength > 0.0 && t.drying > 0.0, "{t:?}");
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
        assert_eq!(Palette::box_names(), [DEFAULT_BOX, "sargent", "inness", "alma-tadema", "tonn", "hopper"]);
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
            Tube { name: "white", pigment: "", color: [0.99; 3], hiding: 0.99, stiff: 0.5, strength: 1.0, drying: 1.0 },
            Tube { name: "black", pigment: "", color: [0.01; 3], hiding: 0.99, stiff: 0.5, strength: 1.0, drying: 1.0 },
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
