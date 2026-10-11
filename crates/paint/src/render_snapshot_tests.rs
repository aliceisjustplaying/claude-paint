use super::*;
use crate::Crop;

const REVISION: [u8; 32] = [0x35; 32];
const RUNTIME: [u8; 32] = [0xa7; 32];

fn canvas(engine: u32, crop: Option<Crop>) -> Canvas {
    let mut canvas = Canvas::new_window(48, 1.25, [0.3, 0.5, 0.2], crop).with_engine(engine);
    canvas.wet.solv.resize(canvas.px.len(), 0.0);
    for i in 0..canvas.px.len() {
        let g = canvas.f.whole_index(i);
        canvas.px[i] = [0.1 + (g % 17) as f32 * 0.025, 0.2, 0.6];
        canvas.height[i] = (g % 13) as f32 * 43.0;
        canvas.wet.vol[i] = (g % 7) as f32 * 0.07;
        canvas.wet.cover[i] = (g % 4) as f32 * 0.25;
        canvas.wet.lat[i] = mixbox::linear_float_rgb_to_latent(&[0.6, 0.1, 0.3]);
        canvas.wet.hide[i][0] = 0.8;
        canvas.gloss[i] = (g % 5) as f32 * 0.2;
        if engine >= 3 {
            canvas.wet.solv[i] = (g % 11) as f32 * 3.0;
        }
    }
    canvas
}

fn encode(snapshot: &RenderSnapshot) -> Vec<u8> {
    let mut bytes = Vec::new();
    snapshot.write(&mut bytes).unwrap();
    bytes
}

fn fixture() -> Vec<u8> {
    let canvas = Canvas::new_window(4, 1.25, [0.3, 0.5, 0.2], None);
    encode(&RenderSnapshot::capture(&canvas, REVISION, RUNTIME).unwrap())
}

fn set_word(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn resign(bytes: &mut [u8]) {
    let mut digest = Sha256::new();
    digest.update(&bytes[..METADATA_LEN]);
    digest.update(&bytes[HEADER_LEN..]);
    bytes[METADATA_LEN..HEADER_LEN].copy_from_slice(&digest.finalize());
}

fn invalid_with_digest(mut bytes: Vec<u8>, expected: &str) {
    resign(&mut bytes);
    let error = RenderSnapshot::read(&mut bytes.as_slice()).err().unwrap();
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    assert!(error.to_string().contains(expected), "{error}");
}

fn rgb_bits(pixels: &[Rgb]) -> Vec<u32> {
    pixels.iter().flatten().map(|v| v.to_bits()).collect()
}

fn float_bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|v| v.to_bits()).collect()
}

#[test]
fn native_roundtrip_is_bit_exact_for_every_engine_and_crop() {
    for engine in 1..=5 {
        for crop in [
            None,
            Some(Crop {
                units: [250.0, 180.0, 700.0, 600.0],
                margin: 120.0,
            }),
        ] {
            let canvas = canvas(engine, crop);
            let snapshot = RenderSnapshot::capture(&canvas, REVISION, RUNTIME).unwrap();
            let bytes = encode(&snapshot);
            assert_eq!(bytes.len(), HEADER_LEN + canvas.px.len() * 20);
            assert_eq!(
                snapshot.content_id(),
                <[u8; 32]>::from(Sha256::digest(&bytes))
            );
            let restored = RenderSnapshot::read(&mut bytes.as_slice()).unwrap();
            assert_eq!(restored.frame(), canvas.window());
            assert_eq!(restored.keep(), canvas.keep);
            assert_eq!(restored.dimensions(), (canvas.f.w, canvas.f.h));
            assert_eq!(restored.revision(), REVISION);
            assert_eq!(restored.runtime(), RUNTIME);
            assert_eq!(restored.engine, engine);
            assert_eq!(
                restored.um_px.to_bits(),
                (canvas.px_mm() * 1000.0).to_bits()
            );
            assert_eq!(restored.content_id(), snapshot.content_id());
            assert_eq!(encode(&restored), bytes);
            assert_eq!(rgb_bits(restored.color()), rgb_bits(&canvas.seen()));
            assert_eq!(
                float_bits(&restored.surface),
                float_bits(&canvas.wet_surface())
            );
            assert_eq!(float_bits(&restored.wet), float_bits(&canvas.wet.vol));
            for (az, el, strength) in [
                (115.0, 55.0, 1.0),
                (135.0, 25.0, 1.0),
                (15.0, 8.0, 1.0),
                (270.0, 89.0, 0.0),
                (45.0, 3.0, 0.4),
            ] {
                assert_eq!(
                    rgb_bits(&restored.lit(az, el, strength)),
                    rgb_bits(&canvas.seen_lit(az, el, strength)),
                    "engine {engine}, crop {crop:?}, light {az}/{el}/{strength}"
                );
            }
            if crop.is_some() {
                assert!(restored.frame().x0 > 0 && restored.frame().y0 > 0);
                assert!(restored.keep().0 > 0 && restored.keep().1 > 0);
                assert!(restored.keep().2 < restored.frame().w);
                assert!(restored.keep().3 < restored.frame().h);
            }
        }
    }
}

#[test]
fn native_float_bits_include_signed_zero_subnormals_and_unclamped_values() {
    let mut canvas = Canvas::new_window(2, 1.0, [0.0; 3], None).with_engine(1);
    canvas.px[0] = [-0.0, f32::from_bits(1), -0.25];
    canvas.px[1] = [1.25, f32::MIN_POSITIVE, f32::from_bits(0x3eaaaaab)];
    canvas.height[0] = -17.5;
    canvas.wet.vol[0] = -0.0;
    canvas.wet.vol[1] = f32::from_bits(1);
    let snapshot = RenderSnapshot::capture(&canvas, REVISION, RUNTIME).unwrap();
    let bytes = encode(&snapshot);
    let restored = RenderSnapshot::read(&mut bytes.as_slice()).unwrap();
    assert_eq!(rgb_bits(restored.color()), rgb_bits(&canvas.seen()));
    assert_eq!(float_bits(&restored.surface), float_bits(&snapshot.surface));
    assert_eq!(float_bits(&restored.wet), float_bits(&canvas.wet.vol));
    assert_eq!(encode(&restored), bytes);
}

fn visible(pixels: &[Rgb], frame: Frame, keep: (usize, usize, usize, usize)) -> Vec<Rgb> {
    (keep.1..keep.3)
        .flat_map(|y| {
            pixels[y * frame.w + keep.0..y * frame.w + keep.2]
                .iter()
                .copied()
        })
        .collect()
}

#[test]
fn crop_lights_the_entire_halo_before_selecting_keep() {
    for engine in 1..=5 {
        let mut whole = Canvas::new_window(96, 1.0, [0.3; 3], None).with_engine(engine);
        let mut crop = Canvas::new_window(
            96,
            1.0,
            [0.3; 3],
            Some(Crop {
                units: [250.0, 250.0, 700.0, 700.0],
                margin: 150.0,
            }),
        )
        .with_engine(engine);
        for canvas in [&mut whole, &mut crop] {
            canvas.mm_per_unit = 0.01;
            for i in 0..canvas.px.len() {
                let gx = i % canvas.f.w + canvas.f.x0;
                canvas.px[i] = [0.25 + (gx % 4) as f32 / 16.0, 0.25, 0.5];
                canvas.height[i] = if (67..71).contains(&gx) { 240.0 } else { 0.0 };
            }
        }
        let snapshot = RenderSnapshot::capture(&crop, REVISION, RUNTIME).unwrap();
        let bytes = encode(&snapshot);
        let restored = RenderSnapshot::read(&mut bytes.as_slice()).unwrap();
        let (kx0, ky0, kx1, ky1) = crop.keep;
        assert!(kx0 > 0 && ky0 > 0 && kx1 < crop.f.w && ky1 < crop.f.h);
        let world_keep = (
            kx0 + crop.f.x0,
            ky0 + crop.f.y0,
            kx1 + crop.f.x0,
            ky1 + crop.f.y0,
        );
        assert_eq!(
            rgb_bits(&visible(restored.color(), crop.f, crop.keep)),
            rgb_bits(&visible(&whole.seen(), whole.f, world_keep))
        );
        for (az, el) in [(0.0, 25.0), (180.0, 55.0)] {
            let pixels = restored.lit(az, el, 1.0);
            assert_eq!(pixels.len(), crop.f.w * crop.f.h);
            assert_eq!(rgb_bits(&pixels), rgb_bits(&crop.seen_lit(az, el, 1.0)));
            assert_eq!(
                rgb_bits(&visible(&pixels, crop.f, crop.keep)),
                rgb_bits(&visible(&whole.seen_lit(az, el, 1.0), whole.f, world_keep))
            );
        }
        let mut keep_only = Canvas::new_window(
            kx1 - kx0,
            (kx1 - kx0) as f32 / (ky1 - ky0) as f32,
            [0.3; 3],
            None,
        )
        .with_engine(engine);
        keep_only.mm_per_unit = crop.mm_per_unit * keep_only.f.scale / crop.f.scale;
        keep_only.px = visible(&crop.px, crop.f, crop.keep);
        keep_only.height = (ky0..ky1)
            .flat_map(|y| {
                crop.height[y * crop.f.w + kx0..y * crop.f.w + kx1]
                    .iter()
                    .copied()
            })
            .collect();
        assert_ne!(
            rgb_bits(&visible(&restored.lit(0.0, 25.0, 1.0), crop.f, crop.keep)),
            rgb_bits(&keep_only.seen_lit(0.0, 25.0, 1.0))
        );
    }
}

#[test]
fn malformed_metadata_with_valid_digest_is_rejected() {
    let original = fixture();
    for (offset, values, expected) in [
        (8, vec![0, 1, 3, u32::MAX], "version"),
        (12, vec![0, 2, u32::MAX], "renderer"),
        (32, vec![0, 9601, u32::MAX], "frame bounds"),
        (36, vec![0, 9601, u32::MAX], "frame bounds"),
        (40, vec![1, u32::MAX], "frame bounds"),
        (44, vec![1, u32::MAX], "frame bounds"),
        (48, vec![0, u32::MAX], "pixel budget"),
        (52, vec![0, u32::MAX], "pixel budget"),
        (48, vec![5], "frame bounds"),
        (52, vec![4], "frame bounds"),
        (60, vec![4, u32::MAX], "keep bounds"),
        (64, vec![3, u32::MAX], "keep bounds"),
        (68, vec![0, 5, u32::MAX], "keep bounds"),
        (72, vec![0, 4, u32::MAX], "keep bounds"),
        (76, vec![0, 6, u32::MAX], "engine"),
    ] {
        for value in values {
            let mut bytes = original.clone();
            set_word(&mut bytes, offset, value);
            invalid_with_digest(bytes, expected);
        }
    }
    for offset in [56, 80] {
        for value in [
            0.0f32,
            -0.0,
            -1.0,
            f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ] {
            let mut bytes = original.clone();
            set_word(&mut bytes, offset, value.to_bits());
            invalid_with_digest(bytes, "scale");
        }
    }
    for offset in [0, 16, 28, 31] {
        let mut bytes = original.clone();
        bytes[offset] ^= 0x80;
        invalid_with_digest(bytes, if offset == 0 { "version" } else { "renderer" });
    }
    let mut bytes = original;
    for offset in [32, 36, 48, 52, 68, 72] {
        set_word(&mut bytes, offset, 4001);
    }
    invalid_with_digest(bytes, "pixel budget");
}

#[test]
fn malformed_optical_payload_with_valid_digest_is_rejected() {
    let original = fixture();
    let n = (original.len() - HEADER_LEN) / 20;
    for offset in [
        HEADER_LEN,
        HEADER_LEN + 4,
        HEADER_LEN + 8,
        HEADER_LEN + n * 12,
        HEADER_LEN + n * 16,
    ] {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut bytes = original.clone();
            set_word(&mut bytes, offset, value.to_bits());
            invalid_with_digest(bytes, "non-finite");
        }
    }
    let mut bytes = original;
    set_word(&mut bytes, HEADER_LEN + n * 16, (-0.1f32).to_bits());
    invalid_with_digest(bytes, "negative");
}

#[test]
fn integrity_authenticates_metadata_and_every_payload_plane() {
    let original = fixture();
    let n = (original.len() - HEADER_LEN) / 20;
    for offset in [
        56,
        60,
        76,
        80,
        84,
        115,
        116,
        147,
        METADATA_LEN,
        HEADER_LEN,
        HEADER_LEN + 4,
        HEADER_LEN + 8,
        HEADER_LEN + n * 12,
        HEADER_LEN + n * 16,
        original.len() - 4,
    ] {
        let mut bytes = original.clone();
        bytes[offset] ^= 1;
        let error = RenderSnapshot::read(&mut bytes.as_slice()).err().unwrap();
        assert!(
            error.to_string().contains("integrity"),
            "offset {offset}: {error}"
        );
    }
    let mut trailing = original;
    trailing.push(0);
    let error = RenderSnapshot::read(&mut trailing.as_slice())
        .err()
        .unwrap();
    assert!(error.to_string().contains("trailing"));
}

#[test]
fn rejects_every_header_payload_and_tail_truncation() {
    let bytes = fixture();
    for end in 0..bytes.len() {
        let error = RenderSnapshot::read(&mut &bytes[..end]).err().unwrap();
        assert_eq!(
            error.kind(),
            io::ErrorKind::UnexpectedEof,
            "truncation at {end}"
        );
    }
}

#[test]
fn content_id_covers_canonical_header_integrity_and_payload() {
    let original = fixture();
    let snapshot = RenderSnapshot::read(&mut original.as_slice()).unwrap();
    assert_eq!(RENDERER, "legacy-look-1");
    assert_eq!(
        snapshot.content_id(),
        <[u8; 32]>::from(Sha256::digest(&original))
    );
    for offset in [84, 116, HEADER_LEN] {
        let mut bytes = original.clone();
        bytes[offset] ^= 1;
        resign(&mut bytes);
        let restored = RenderSnapshot::read(&mut bytes.as_slice()).unwrap();
        assert_ne!(restored.content_id(), snapshot.content_id());
        assert_eq!(
            restored.content_id(),
            <[u8; 32]>::from(Sha256::digest(&bytes))
        );
        assert_eq!(encode(&restored), bytes);
    }
}

#[test]
fn rejects_invalid_capture_before_rendering_or_allocating() {
    let original = Canvas::new_window(4, 1.0, [0.3; 3], None);
    for frame in [
        Frame {
            w: usize::MAX,
            h: 2,
            ..original.f
        },
        Frame {
            full_w: 9601,
            ..original.f
        },
        Frame {
            full_h: 0,
            ..original.f
        },
        Frame {
            x0: usize::MAX,
            ..original.f
        },
        Frame {
            y0: 1,
            ..original.f
        },
        Frame {
            scale: f32::NAN,
            ..original.f
        },
        Frame {
            scale: 0.0,
            ..original.f
        },
    ] {
        let mut canvas = original.clone();
        canvas.f = frame;
        assert!(RenderSnapshot::capture(&canvas, REVISION, RUNTIME).is_err());
    }
    for keep in [
        (0, 0, 5, 4),
        (0, 0, 4, 5),
        (3, 0, 2, 4),
        (0, 3, 4, 2),
        (0, 0, 0, 4),
    ] {
        let mut canvas = original.clone();
        canvas.keep = keep;
        assert!(RenderSnapshot::capture(&canvas, REVISION, RUNTIME).is_err());
    }
    for engine in [0, 6, u32::MAX] {
        assert!(
            RenderSnapshot::capture(&original.clone().with_engine(engine), REVISION, RUNTIME)
                .is_err()
        );
    }
    for size in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        let mut canvas = original.clone();
        canvas.mm_per_unit = size;
        assert!(RenderSnapshot::capture(&canvas, REVISION, RUNTIME).is_err());
    }
    for value in [-0.1, f32::NAN, f32::INFINITY] {
        let mut canvas = original.clone();
        canvas.wet.vol[0] = value;
        assert!(RenderSnapshot::capture(&canvas, REVISION, RUNTIME).is_err());
    }
    let mut canvas = original.clone();
    canvas.px[0][0] = f32::NAN;
    assert!(RenderSnapshot::capture(&canvas, REVISION, RUNTIME).is_err());
    let mut canvas = original.clone();
    canvas.height[0] = f32::INFINITY;
    assert!(RenderSnapshot::capture(&canvas, REVISION, RUNTIME).is_err());
    let mut canvas = original;
    canvas.wet.vol.pop();
    assert!(RenderSnapshot::capture(&canvas, REVISION, RUNTIME).is_err());
    assert!(validate_metadata(Frame::new(4000, 4000, 4.0), (0, 0, 4000, 4000), 5, 1.0).is_ok());
    assert!(
        validate_metadata(
            Frame {
                full_w: 9600,
                full_h: 9600,
                x0: 9596,
                y0: 9596,
                ..Frame::new(4, 4, 9.6)
            },
            (1, 1, 3, 3),
            1,
            0.1
        )
        .is_ok()
    );
}

struct BlockReader<'a> {
    bytes: &'a [u8],
    requests: Vec<usize>,
    limit: usize,
}

impl Read for BlockReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.requests.push(buf.len());
        let n = buf.len().min(self.bytes.len()).min(self.limit);
        buf[..n].copy_from_slice(&self.bytes[..n]);
        self.bytes = &self.bytes[n..];
        Ok(n)
    }
}

struct BlockWriter {
    bytes: Vec<u8>,
    requests: Vec<usize>,
    limit: usize,
}

impl Write for BlockWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.requests.push(buf.len());
        let n = buf.len().min(self.limit);
        self.bytes.extend_from_slice(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn streams_bounded_blocks_and_supports_short_reads_and_writes() {
    let canvas = Canvas::new_window(192, 1.5, [0.3; 3], None);
    let snapshot = RenderSnapshot::capture(&canvas, REVISION, RUNTIME).unwrap();
    let mut writer = BlockWriter {
        bytes: Vec::new(),
        requests: Vec::new(),
        limit: usize::MAX,
    };
    snapshot.write(&mut writer).unwrap();
    assert!(writer.requests.iter().all(|&n| n <= BLOCK_SIZE));
    assert!(writer.requests.iter().filter(|&&n| n == BLOCK_SIZE).count() > 1);
    let mut reader = BlockReader {
        bytes: &writer.bytes,
        requests: Vec::new(),
        limit: usize::MAX,
    };
    let restored = RenderSnapshot::read(&mut reader).unwrap();
    assert_eq!(restored.content_id(), snapshot.content_id());
    assert!(reader.requests.len() < 20);
    assert!(reader.requests.iter().filter(|&&n| n == BLOCK_SIZE).count() > 1);
    let mut reader = BlockReader {
        bytes: &writer.bytes,
        requests: Vec::new(),
        limit: 17,
    };
    let restored = RenderSnapshot::read(&mut reader).unwrap();
    assert_eq!(restored.content_id(), snapshot.content_id());
    let mut short_writer = BlockWriter {
        bytes: Vec::new(),
        requests: Vec::new(),
        limit: 19,
    };
    restored.write(&mut short_writer).unwrap();
    assert_eq!(short_writer.bytes, writer.bytes);
}

#[test]
fn invalid_dimensions_stop_before_any_payload_read() {
    let mut bytes = fixture();
    set_word(&mut bytes, 48, u32::MAX);
    resign(&mut bytes);
    let mut reader = BlockReader {
        bytes: &bytes,
        requests: Vec::new(),
        limit: usize::MAX,
    };
    assert!(RenderSnapshot::read(&mut reader).is_err());
    assert_eq!(reader.requests, [HEADER_LEN]);
    assert_eq!(reader.bytes.len(), bytes.len() - HEADER_LEN);
}

struct FailingReader<'a> {
    bytes: &'a [u8],
}

impl Read for FailingReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.bytes.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "read failed",
            ));
        }
        self.bytes.read(buf)
    }
}

struct FailingWriter {
    remaining: usize,
}

impl Write for FailingWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            return Err(io::Error::new(io::ErrorKind::StorageFull, "write failed"));
        }
        let n = self.remaining.min(buf.len());
        self.remaining -= n;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn propagates_header_payload_and_tail_io_failures() {
    let bytes = fixture();
    let snapshot = RenderSnapshot::read(&mut bytes.as_slice()).unwrap();
    for end in [0, 8, METADATA_LEN, HEADER_LEN, bytes.len() - 1, bytes.len()] {
        let error = RenderSnapshot::read(&mut FailingReader {
            bytes: &bytes[..end],
        })
        .err()
        .unwrap();
        assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    }
    for end in [0, 8, METADATA_LEN, HEADER_LEN, bytes.len() - 1] {
        let error = snapshot
            .write(&mut FailingWriter { remaining: end })
            .err()
            .unwrap();
        assert_eq!(error.kind(), io::ErrorKind::StorageFull);
    }
}
