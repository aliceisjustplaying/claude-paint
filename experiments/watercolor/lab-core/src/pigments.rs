// Copyright (c) 2026 Ryan McCarty. MIT License.
// Mechanical port of uxpreview/watercolor-lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
// See experiments/watercolor/THIRD_PARTY_NOTICES.md for the full license.

use crate::km::{hex_to_linear, km_from_masstone};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pigment {
    pub id: String,
    pub name: String,
    pub code: String,
    pub hex: String,
    pub scattering: f32,
    pub strength: f32,
    pub granulation: f32,
    pub staining: f32,
    pub density: f32,
    pub k: [f32; 3],
    pub s: [f32; 3],
}

pub const DEFAULT_PIGMENT_ID: &str = "french-ultramarine";

/// Source: src/paint/pigments.ts:71–121 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn pigments() -> Vec<Pigment> {
    let specs = [
        (
            "hansa-yellow",
            "Hansa Yellow Light",
            "PY3",
            "#f0dd18",
            0.28,
            1.2,
            0.0,
            0.55,
            0.15,
        ),
        (
            "cadmium-yellow",
            "Cadmium Yellow",
            "PY35",
            "#f6b90a",
            1.9,
            1.3,
            0.1,
            0.25,
            0.65,
        ),
        (
            "indian-yellow",
            "Indian Yellow",
            "PY110",
            "#d98a10",
            0.12,
            1.6,
            0.0,
            0.7,
            0.1,
        ),
        (
            "naples-yellow",
            "Naples Yellow",
            "PBr24",
            "#eec87e",
            2.3,
            1.2,
            0.15,
            0.1,
            0.7,
        ),
        (
            "yellow-ochre",
            "Yellow Ochre",
            "PY43",
            "#b8823f",
            1.1,
            1.2,
            0.55,
            0.2,
            0.75,
        ),
        (
            "green-gold",
            "Green Gold",
            "PY129",
            "#7f7a24",
            0.15,
            1.4,
            0.0,
            0.6,
            0.15,
        ),
        (
            "cadmium-orange",
            "Cadmium Orange",
            "PO20",
            "#e0731f",
            1.9,
            1.3,
            0.1,
            0.3,
            0.65,
        ),
        (
            "pyrrol-scarlet",
            "Pyrrol Scarlet",
            "PR255",
            "#c72f1a",
            0.35,
            1.6,
            0.0,
            0.75,
            0.2,
        ),
        (
            "cadmium-red",
            "Cadmium Red",
            "PR108",
            "#b52a2c",
            1.8,
            1.3,
            0.15,
            0.3,
            0.7,
        ),
        (
            "quinacridone-rose",
            "Quinacridone Rose",
            "PV19",
            "#97203f",
            0.08,
            1.8,
            0.0,
            0.8,
            0.1,
        ),
        (
            "alizarin-crimson",
            "Alizarin Crimson",
            "PR83",
            "#6e1423",
            0.1,
            2.2,
            0.0,
            0.75,
            0.12,
        ),
        (
            "quinacridone-burnt-orange",
            "Quin. Burnt Orange",
            "PO48",
            "#8f4522",
            0.15,
            1.5,
            0.15,
            0.7,
            0.2,
        ),
        (
            "cobalt-violet",
            "Cobalt Violet",
            "PV14",
            "#8d4f85",
            0.7,
            1.1,
            0.75,
            0.05,
            0.8,
        ),
        (
            "dioxazine-violet",
            "Dioxazine Violet",
            "PV23",
            "#2a163f",
            0.1,
            2.5,
            0.0,
            0.85,
            0.12,
        ),
        (
            "ultramarine-violet",
            "Ultramarine Violet",
            "PV15",
            "#4f4386",
            0.4,
            1.3,
            0.65,
            0.1,
            0.6,
        ),
        (
            "french-ultramarine",
            "French Ultramarine",
            "PB29",
            "#363a92",
            0.45,
            1.6,
            0.85,
            0.15,
            0.6,
        ),
        (
            "cobalt-blue",
            "Cobalt Blue",
            "PB28",
            "#2f4da8",
            0.7,
            1.4,
            0.6,
            0.15,
            0.7,
        ),
        (
            "cerulean-blue",
            "Cerulean Blue",
            "PB35",
            "#34718f",
            2.3,
            1.3,
            0.85,
            0.1,
            0.85,
        ),
        (
            "phthalo-blue",
            "Phthalo Blue (GS)",
            "PB15:3",
            "#032f56",
            0.07,
            3.0,
            0.0,
            0.9,
            0.08,
        ),
        (
            "prussian-blue",
            "Prussian Blue",
            "PB27",
            "#122b3b",
            0.12,
            2.2,
            0.05,
            0.8,
            0.2,
        ),
        (
            "indanthrone-blue",
            "Indanthrone Blue",
            "PB60",
            "#26315e",
            0.15,
            1.8,
            0.1,
            0.7,
            0.25,
        ),
        (
            "phthalo-green",
            "Phthalo Green (BS)",
            "PG7",
            "#063f33",
            0.07,
            3.0,
            0.0,
            0.9,
            0.08,
        ),
        (
            "viridian", "Viridian", "PG18", "#2a6b52", 0.35, 1.2, 0.6, 0.15, 0.55,
        ),
        (
            "sap-green",
            "Sap Green",
            "mix",
            "#47571f",
            0.18,
            1.6,
            0.05,
            0.5,
            0.2,
        ),
        (
            "hookers-green",
            "Hooker's Green",
            "mix",
            "#2f4f30",
            0.2,
            1.6,
            0.1,
            0.5,
            0.25,
        ),
        (
            "terre-verte",
            "Terre Verte",
            "PG23",
            "#71805e",
            0.5,
            1.0,
            0.6,
            0.05,
            0.7,
        ),
        (
            "raw-sienna",
            "Raw Sienna",
            "PBr7",
            "#9c6428",
            0.55,
            1.3,
            0.6,
            0.2,
            0.7,
        ),
        (
            "burnt-sienna",
            "Burnt Sienna",
            "PBr7",
            "#7a3b22",
            0.5,
            1.4,
            0.6,
            0.25,
            0.7,
        ),
        (
            "raw-umber",
            "Raw Umber",
            "PBr7",
            "#655136",
            0.55,
            1.3,
            0.65,
            0.2,
            0.75,
        ),
        (
            "burnt-umber",
            "Burnt Umber",
            "PBr7",
            "#5b3a25",
            0.55,
            1.4,
            0.6,
            0.25,
            0.75,
        ),
        (
            "indian-red",
            "Indian Red",
            "PR101",
            "#7f3a33",
            1.7,
            1.3,
            0.7,
            0.3,
            0.9,
        ),
        (
            "buff-titanium",
            "Buff Titanium",
            "PW6:1",
            "#e6d5b2",
            2.4,
            1.1,
            0.55,
            0.05,
            0.8,
        ),
        (
            "sepia", "Sepia", "mix", "#4a3423", 0.35, 1.5, 0.35, 0.4, 0.5,
        ),
        (
            "paynes-gray",
            "Payne's Gray",
            "mix",
            "#31404c",
            0.25,
            1.5,
            0.35,
            0.5,
            0.4,
        ),
        (
            "neutral-tint",
            "Neutral Tint",
            "mix",
            "#37323e",
            0.2,
            1.8,
            0.15,
            0.6,
            0.3,
        ),
        (
            "lamp-black",
            "Lamp Black",
            "PBk6",
            "#22211f",
            0.5,
            1.6,
            0.3,
            0.5,
            0.45,
        ),
    ];
    specs
        .into_iter()
        .map(
            |(id, name, code, hex, scattering, strength, granulation, staining, density)| {
                let (k, s) = km_from_masstone(hex_to_linear(hex), scattering, strength);
                Pigment {
                    id: id.into(),
                    name: name.into(),
                    code: code.into(),
                    hex: hex.into(),
                    scattering: scattering as f32,
                    strength: strength as f32,
                    granulation: granulation as f32,
                    staining: staining as f32,
                    density: density as f32,
                    k,
                    s,
                }
            },
        )
        .collect()
}

/// Source: src/paint/pigments.ts:123 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn pigment_by_id(id: &str) -> Option<Pigment> {
    pigments().into_iter().find(|p| p.id == id)
}

/// Source: src/paint/pigments.ts:129–135 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn handling_note(p: &Pigment) -> String {
    let mut notes = vec![if p.scattering >= 1.2 {
        "opaque"
    } else if p.scattering >= 0.4 {
        "semi-transparent"
    } else {
        "transparent"
    }];
    if p.granulation >= 0.5 {
        notes.push("granulating");
    }
    if p.staining >= 0.65 {
        notes.push("staining");
    }
    notes.join(" · ")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PigmentFact {
    pub key: String,
    pub label: String,
    pub value: f32,
    pub level: f32,
    pub word: String,
}

/// Source: src/paint/pigments.ts:149–188 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn pigment_facts(p: &Pigment) -> Vec<PigmentFact> {
    [
        (
            "transparency",
            "Opacity",
            p.scattering,
            (p.scattering / 2.5).clamp(0.0, 1.0),
            if p.scattering >= 1.2 {
                "opaque"
            } else if p.scattering >= 0.4 {
                "semi-transparent"
            } else {
                "transparent"
            },
        ),
        (
            "strength",
            "Tinting strength",
            p.strength,
            ((p.strength - 0.8) / 1.2).clamp(0.0, 1.0),
            if p.strength >= 1.7 {
                "very strong"
            } else if p.strength >= 1.3 {
                "strong"
            } else if p.strength >= 1.0 {
                "moderate"
            } else {
                "weak"
            },
        ),
        (
            "granulation",
            "Granulation",
            p.granulation,
            p.granulation.clamp(0.0, 1.0),
            if p.granulation >= 0.7 {
                "heavy"
            } else if p.granulation >= 0.5 {
                "granulating"
            } else if p.granulation >= 0.2 {
                "slight"
            } else {
                "smooth"
            },
        ),
        (
            "staining",
            "Staining",
            p.staining,
            p.staining.clamp(0.0, 1.0),
            if p.staining >= 0.65 {
                "staining"
            } else if p.staining >= 0.35 {
                "moderate"
            } else {
                "lifts clean"
            },
        ),
        (
            "density",
            "Settling",
            p.density,
            p.density.clamp(0.0, 1.0),
            if p.density >= 0.7 {
                "sinks fast"
            } else if p.density >= 0.4 {
                "settles"
            } else {
                "stays afloat"
            },
        ),
    ]
    .into_iter()
    .map(|(key, label, value, level, word)| PigmentFact {
        key: key.into(),
        label: label.into(),
        value,
        level,
        word: word.into(),
    })
    .collect()
}

/// Source: src/paint/pigments.ts:194–217 at a19bfa2a026856a98ceb400a8eec41f091f04a11.
pub fn handling_sentence(p: &Pigment) -> String {
    let cover = if p.scattering >= 1.2 {
        "Opaque: it hides the paper and lightens what it glazes over"
    } else if p.scattering >= 0.4 {
        "Semi-transparent: a wash lets the sheet through, a heavy load does not"
    } else {
        "Transparent: every glaze shows what is under it"
    };
    let movement = if p.granulation >= 0.5 && p.density >= 0.7 {
        "settles fast and hard into the tooth, so a wash calms and granulates"
    } else if p.granulation >= 0.5 {
        "granulates into the tooth as it dries"
    } else if p.density <= 0.3 {
        "stays in the water and keeps travelling, so blooms and backruns run far"
    } else {
        "flows evenly and dries smooth"
    };
    let lift = if p.staining >= 0.65 {
        "and it stains: once settled it will not lift"
    } else if p.staining >= 0.35 {
        "and it lifts partly from damp paper"
    } else {
        "and it lifts clean from damp paper"
    };
    let strength = if p.strength >= 1.7 {
        " A little goes a long way."
    } else if p.strength < 1.0 {
        " It is weak in the well; load the brush."
    } else {
        ""
    };
    format!("{cover}; {movement}, {lift}.{strength}")
}
