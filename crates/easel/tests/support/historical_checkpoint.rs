fn word(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn floats(bytes: &mut Vec<u8>, values: &[f32]) {
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
}

pub fn fixture(version: u32, occupied: bool) -> Vec<u8> {
    assert!((7..=18).contains(&version));
    let base = if version == 18 { 14 } else { version };
    let engine = match base { 7 | 8 => 1, 9 | 10 => 3, _ => 5 };
    let header = format!("engine={engine}\n");
    let magic = if version < 10 { format!("PAINTCK{version}") } else { format!("PAINTC{version}") };
    let mut bytes = magic.into_bytes();
    word(&mut bytes, header.len() as u64);
    bytes.extend_from_slice(header.as_bytes());
    if version == 18 {
        word(&mut bytes, base as u64);
    }
    for value in [2, 1, 0, 0, 2, 1, 0, 0, 2, 1] {
        word(&mut bytes, value);
    }
    floats(&mut bytes, &[0.002, 0.1]);
    for value in [0, 3, 7, 1, 0, 0, 2, 1] {
        word(&mut bytes, value);
    }
    floats(&mut bytes, &[0.2, 0.4, 0.6, 0.8, 0.6, 0.4]);
    floats(&mut bytes, &[1.0, 2.0]);
    floats(&mut bytes, &[3.0, 4.0]);
    floats(&mut bytes, &[0.0, if occupied { 0.5 } else { 0.0 }]);
    for _ in 0..2 {
        floats(&mut bytes, &[0.1, 0.2, 0.3, 0.4, -0.1, 0.05, 0.15]);
    }
    for _ in 0..2 {
        floats(&mut bytes, &[0.7, 0.6, 1.2]);
        if base >= 11 {
            floats(&mut bytes, &[0.4, 0.5]);
        }
    }
    for value in [2u32, 3, 4, 5] {
        floats(&mut bytes, &[f32::from_bits(value)]);
    }
    word(&mut bytes, 2.5f64.to_bits());
    for value in [6, 1, 0, 0, 2, 1, 1] {
        word(&mut bytes, value);
    }
    for _ in 0..2 {
        floats(&mut bytes, &[0.1, 0.2, 0.3, 0.4, 0.5, 0.6]);
    }
    floats(&mut bytes, &[12.0, 0.25, 0.75]);
    for value in [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] {
        word(&mut bytes, value);
    }
    if base != 7 {
        word(&mut bytes, engine);
    }
    if base >= 11 {
        floats(&mut bytes, &[0.15, 0.25, 0.35, 0.45]);
    }
    if base >= 9 {
        floats(&mut bytes, &[0.0, 0.125]);
    }
    if matches!(base, 14 | 16 | 17) {
        for value in [0x4e415244, 1, 0] {
            word(&mut bytes, value);
        }
        for _ in 0..2 {
            floats(&mut bytes, &[0.45, 0.3, 0.25, 0.6, 0.2]);
        }
    }
    if matches!(base, 10 | 12) {
        for value in [0x4b414f53, 1, 6] {
            word(&mut bytes, value);
        }
        bytes.extend_from_slice(b"cotton");
        floats(&mut bytes, &[0.5, 0.4, 0.3, 20.0, 0.1]);
        for value in [0, 2] {
            word(&mut bytes, value);
        }
        floats(&mut bytes, &[1.0, 1.0]);
    }
    if matches!(base, 13 | 16) {
        for value in [0x52504150, 1] {
            word(&mut bytes, value);
        }
        floats(&mut bytes, &[120.0, 1.0, 20.0, 150.0, 0.5, 0.5, 0.2, 1.0, 0.4, 0.3, 0.8, 10.0]);
        for value in [2, 0, 0] {
            word(&mut bytes, value);
        }
        floats(&mut bytes, &[2.0, 3.0]);
    }
    if matches!(base, 15 | 17) {
        for value in [0x54454853, 1] {
            word(&mut bytes, value);
        }
        floats(&mut bytes, &[100.0, 5.0, 0.8, 0.78, 0.7]);
        floats(&mut bytes, &[1.0, 0.0, 0.2, 0.0, 0.7, 0.2, 0.1, 0.0, 0.0, 0.0]);
    }
    if version == 18 {
        for _ in 0..2 {
            word(&mut bytes, 1);
            word(&mut bytes, 3);
            floats(&mut bytes, &[0.75]);
        }
    }
    bytes
}
