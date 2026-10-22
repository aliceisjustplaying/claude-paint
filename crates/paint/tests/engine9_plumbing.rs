use paint::{Canvas, Knife, Paint, Palette, PIGMENT_MIXER, mixer};
use std::io::Cursor;

fn save(c: &Canvas) -> Vec<u8> {
    let mut bytes = Vec::new();
    c.write_state(&mut bytes, "engine9 plumbing").unwrap();
    bytes
}

fn painted(engine: u32) -> Canvas {
    let mut c = Canvas::new_window(16, 1.0, [0.3; 3], None).with_engine(engine).with_drain(true);
    let mut k = Knife::new(200.0);
    let mut p = Paint::body([0.2, 0.4, 0.7]);
    p.parts = vec![(3, 0.6), (8, 0.4)];
    k.load(p, 0.5);
    c.knife(&mut k, &[(300.0, 500.0), (700.0, 500.0)], (0.5, 0.5), Some(0.0), true, 0.0);
    assert!(c.wet_total() > 0.0);
    c
}

fn floats(c: &Canvas, name: &str) -> Vec<f32> {
    match c.state_fields().into_iter().find(|f| f.name == name).unwrap().values {
        paint::state_dump::Values::F32(values) => values,
        _ => panic!("not a float field"),
    }
}

#[test]
fn engine9_keeps_engine8_switches_and_engine5_stays_the_low_level_default() {
    assert_eq!(paint::ENGINE, 5);
    assert!(paint::supported_engine(PIGMENT_MIXER));
    assert!(paint::engine7::active(PIGMENT_MIXER));
    let c = Canvas::new_window(2, 1.0, [0.3; 3], None)
        .with_engine(PIGMENT_MIXER)
        .with_knife_skim(true)
        .with_blend_feather(true)
        .with_brush_skim(true)
        .with_quarter_tiles(true)
        .with_adaptive_sweep(true)
        .with_engine(PIGMENT_MIXER);
    assert!(c.knife_skim() && c.blend_feather() && c.brush_skim());
    assert!(!c.quarter_tiles() && !c.adaptive_sweep());
    let (restored, _) = Canvas::read_state(&mut Cursor::new(save(&c))).unwrap();
    assert!(restored.knife_skim() && restored.blend_feather() && restored.brush_skim());
}

#[test]
fn public_palette_engine_changes_and_clones_keep_the_universal_cache() {
    let mut p = Palette::tube_box();
    let parts = vec![(0, 0.4), (1, 0.6)];
    for engine in [5, PIGMENT_MIXER, 5, PIGMENT_MIXER] {
        p.engine = engine;
        let mut expected = [0.0; mixer::LATENT_SIZE];
        let mut total = 0.0;
        for &(i, fraction) in &parts {
            let weight = fraction * p.tubes[i].strength;
            let latent = mixer::encode(&p.tubes[i].color);
            for k in 0..mixer::LATENT_SIZE { expected[k] += latent[k] * weight; }
            total += weight;
        }
        for value in &mut expected { *value /= total; }
        let expected = mixer::decode(&expected);
        assert_eq!(p.pile(parts.clone()).color, expected);
        assert_eq!(p.clone().pile(parts.clone()).color, expected);
    }
}

#[test]
fn own_mixer_checkpoint_roundtrips_occupied_latents_and_pigment_parts() {
    let c = painted(PIGMENT_MIXER);
    let bytes = save(&c);
    assert_eq!(&bytes[..8], b"PAINTC19");
    let (restored, header) = Canvas::read_state(&mut Cursor::new(&bytes)).unwrap();
    assert_eq!(header, "engine9 plumbing");
    assert_eq!(save(&restored), bytes);
    assert_eq!(restored.seen(), c.seen());
}

fn untagged(bytes: Vec<u8>) -> Vec<u8> {
    let header_len = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
    let envelope = 16 + header_len;
    let base = u64::from_le_bytes(bytes[envelope + 8..envelope + 16].try_into().unwrap());
    let pigments = u64::from_le_bytes(bytes[envelope + 16..envelope + 24].try_into().unwrap());
    let mut old = bytes[..envelope].to_vec();
    if pigments == 1 {
        old[..8].copy_from_slice(b"PAINTC18");
        old.extend_from_slice(&base.to_le_bytes());
    } else {
        old[..8].copy_from_slice(format!("PAINTC{base:02}").as_bytes());
    }
    old.extend_from_slice(&bytes[envelope + 24..]);
    old
}

#[test]
fn obsolete_checkpoints_are_refused_only_when_occupied() {
    let old = untagged(save(&painted(5)));
    let error = Canvas::read_state(&mut Cursor::new(old)).err().unwrap();
    assert!(error.to_string().contains("obsolete pigment representation"));
    let empty = Canvas::new_window(2, 1.0, [0.3; 3], None).with_engine(5);
    let (restored, _) = Canvas::read_state(&mut Cursor::new(untagged(save(&empty)))).unwrap();
    assert_eq!(&save(&restored)[..8], b"PAINTC19");
}

#[test]
fn engine_switches_keep_the_universal_pigment_representation() {
    let c = painted(5);
    let latent = floats(&c, "wet.lat");
    let colors = c.seen();
    let switched = c.with_engine(PIGMENT_MIXER);
    assert_eq!(floats(&switched, "wet.lat"), latent);
    assert_eq!(switched.seen(), colors);
    assert_eq!(&save(&switched)[..8], b"PAINTC19");
}

#[test]
fn unknown_checkpoint_mixer_identity_is_refused() {
    let mut bytes = save(&Canvas::new_window(2, 1.0, [0.3; 3], None));
    let header_len = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
    let identity = 16 + header_len;
    bytes[identity..identity + 8].copy_from_slice(&2u64.to_le_bytes());
    assert!(Canvas::read_state(&mut Cursor::new(bytes)).err().unwrap().to_string().contains("obsolete pigment representation"));
}
