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
    let (restored, _) = Canvas::read_state(&mut Cursor::new(save(&c).as_slice())).unwrap();
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
            let strength = if engine >= PIGMENT_MIXER { mixer::tube_strength9(p.tubes[i].name).unwrap_or(p.tubes[i].strength) } else { p.tubes[i].strength };
            let weight = fraction * strength;
            let latent = mixer::tube(p.tubes[i].name).unwrap_or_else(|| if engine >= PIGMENT_MIXER { mixer::encode9(&p.tubes[i].color) } else { mixer::encode(&p.tubes[i].color) });
            for k in 0..mixer::LATENT_SIZE { expected[k] += latent[k] * weight; }
            total += weight;
        }
        if engine < PIGMENT_MIXER {
            for value in &mut expected { *value /= total; }
        }
        let expected = if engine >= PIGMENT_MIXER { mixer::decode9(&expected) } else { mixer::decode(&expected) };
        assert_eq!(p.pile(parts.clone()).color, expected);
        assert_eq!(p.clone().pile(parts.clone()).color, expected);
    }
}

#[test]
fn own_mixer_checkpoint_roundtrips_occupied_latents_and_pigment_parts() {
    let c = painted(PIGMENT_MIXER);
    let bytes = save(&c);
    assert_eq!(&bytes[..8], b"PAINTC19");
    let (restored, header) = Canvas::read_state(&mut Cursor::new(bytes.as_slice())).unwrap();
    assert_eq!(header, "engine9 plumbing");
    assert_eq!(save(&restored), bytes);
    assert_eq!(restored.seen(), c.seen());
}

#[test]
fn obsolete_checkpoints_are_refused_only_when_occupied() {
    for version in 7..=18 {
        let old = super::historical_checkpoint::fixture(version, true);
        let error = Canvas::read_state(&mut Cursor::new(old.as_slice())).err().unwrap();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData, "version {version}: {error}");
        assert!(error.to_string().contains("wet paint in an obsolete pigment representation"), "version {version}: {error}");
        let old = super::historical_checkpoint::fixture(version, false);
        let (restored, _) = Canvas::read_state(&mut Cursor::new(old.as_slice())).unwrap_or_else(|e| panic!("version {version}: {e}"));
        assert_eq!(restored.pixels(), &[[0.2, 0.4, 0.6], [0.8, 0.6, 0.4]]);
        assert_eq!(floats(&restored, "height"), [1.0, 2.0]);
        assert_eq!(floats(&restored, "film"), [3.0, 4.0]);
        assert_eq!(floats(&restored, "wet.vol"), [0.0, 0.0]);
        assert_eq!(floats(&restored, "wet.cover"), [0.25, 0.75]);
        assert_eq!(floats(&restored, "clock.cure"), [0.1, 0.1]);
        assert!(floats(&restored, "wet.lat").iter().all(|&v| v == 0.0));
        assert!(!restored.state_fields().iter().any(|f| f.name.starts_with("wet.parts.")));
        let current = save(&restored);
        assert_eq!(&current[..8], b"PAINTC19");
        let (again, _) = Canvas::read_state(&mut Cursor::new(current.as_slice())).unwrap();
        assert_eq!(save(&again), current);
    }
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
    assert!(Canvas::read_state(&mut Cursor::new(bytes.as_slice())).err().unwrap().to_string().contains("obsolete pigment representation"));
}
