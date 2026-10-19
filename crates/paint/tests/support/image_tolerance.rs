#![allow(dead_code)]

pub fn assert_close(actual: &image::RgbaImage, expected: &image::RgbaImage) {
    assert_eq!(actual.dimensions(), expected.dimensions(), "image dimensions changed");
    let maximum = actual.as_raw().iter().zip(expected.as_raw()).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
    assert!(maximum <= 2, "maximum RGBA channel difference {maximum}/255 exceeds 2/255");
}

pub fn assert_fields_close(actual: impl IntoIterator<Item = f32>, expected: impl IntoIterator<Item = f32>) {
    let actual: Vec<_> = actual.into_iter().collect();
    let expected: Vec<_> = expected.into_iter().collect();
    assert_eq!(actual.len(), expected.len(), "field dimensions changed");
    for (i, (a, b)) in actual.into_iter().zip(expected).enumerate() {
        assert!(a.is_finite() && b.is_finite() && (a - b).abs() <= 1e-6 + 1e-5 * b.abs(), "field {i}: {a} != {b} within 1e-6 absolute + 1e-5 relative");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_levels_pass_in_every_channel_and_three_fail() {
        let expected = image::RgbaImage::from_pixel(1, 1, image::Rgba([100, 100, 100, 100]));
        assert_close(&image::RgbaImage::from_pixel(1, 1, image::Rgba([102, 98, 102, 98])), &expected);
        for channel in 0..4 {
            let mut actual = expected.clone();
            actual.get_pixel_mut(0, 0)[channel] += 3;
            assert!(std::panic::catch_unwind(|| assert_close(&actual, &expected)).is_err());
        }
        assert!(std::panic::catch_unwind(|| assert_close(&image::RgbaImage::new(2, 1), &expected)).is_err());
    }

    #[test]
    fn numeric_fields_accept_rounding_and_reject_regressions_and_nonfinite_values() {
        assert_fields_close([0.0, 1.00001], [1e-6, 1.0]);
        for value in [1.01, f32::NAN, f32::INFINITY] {
            assert!(std::panic::catch_unwind(|| assert_fields_close([value], [1.0])).is_err());
        }
        assert!(std::panic::catch_unwind(|| assert_fields_close([1.0], [1.0, 1.0])).is_err());
    }
}
