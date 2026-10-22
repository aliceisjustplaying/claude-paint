//! Thinner check 13 (a): frozen non-mixer pigment fields and box ordering,
//! with explicitly authorized accepted-mixer color and strength captures.
//!
//!   cargo test -p paint --release --test thinner_pigments
//!
//! `tests/thinner/tubes_af49348.txt` retains the historical catalog layout and
//! non-mixer fields; its color/strength values were refreshed after round-5
//! acceptance. `tubes_current.txt` freezes the expanded current table.

use paint::Palette;

/// Every tube the engine knows (`Debug`: name, pigment, masstone, hiding,
/// stiffness, strength, drying rates, floats in their shortest exact
/// form), then each box this build has, its tubes in order and its engine.
fn tube_table() -> String {
    let mut s = String::from("catalog\n");
    for t in paint::palette::catalog() {
        s += &format!("{t:?}\n");
    }
    for name in Palette::box_names() {
        let b = Palette::named_box(name).unwrap_or_else(|| panic!("box {name}"));
        s += &format!("box {name} (engine {})\n", b.engine);
        for t in &b.tubes {
            s += &format!("  {t:?}\n");
        }
    }
    s
}

/// Prints the current tube table for an explicitly authorized capture.
#[test]
#[ignore = "prints the current tube table for authorized capture updates"]
fn print_tube_table() {
    let out = std::env::var("TUBE_TABLE_OUT").expect("TUBE_TABLE_OUT: the file to write");
    std::fs::write(&out, tube_table()).unwrap_or_else(|e| panic!("{out}: {e}"));
}

/// Check 13 (a): frozen non-mixer fields and legacy box tube ordering remain
/// exact, alongside the accepted color/strength capture. New tubes, boxes, engine labels and the
/// pigments' packing fields (oil absorption, density, drain) are allowed, but
/// the expanded current table has its own exact reviewed snapshot.
#[test]
fn c13_every_pigment_value_is_af49348s() {
    fn sections(table: &str) -> std::collections::BTreeMap<&str, Vec<&str>> {
        let mut result = std::collections::BTreeMap::new();
        let mut section = "catalog";
        for line in table.lines() {
            if line == "catalog" {
                result.insert(section, Vec::new());
            } else if let Some(header) = line.strip_prefix("box ") {
                section = header.split_once(" (engine ").expect("box engine label").0;
                assert!(result.insert(section, Vec::new()).is_none(), "duplicate box {section}");
            } else {
                result.get_mut(section).expect("table section").push(line.trim_start());
            }
        }
        result
    }
    // a tube's line as af49348 prints it: without the packing fields
    fn historical(line: &str) -> String {
        line.split_once(", oa: ").map_or_else(|| line.to_string(), |(a, _)| format!("{a} }}"))
    }
    let got = tube_table();
    let old = sections(include_str!("thinner/tubes_af49348.txt"));
    let now = sections(&got);
    for (section, tubes) in old {
        let current = now.get(section).unwrap_or_else(|| panic!("missing historical section {section}"));
        if section == "catalog" {
            for tube in tubes {
                let name = tube.split_once(", pigment:").expect("tube name").0;
                let actual = current.iter().find(|line| line.split_once(", pigment:").expect("tube name").0 == name);
                assert_eq!(actual.map(|l| historical(l)).as_deref(), Some(tube), "historical catalog fields changed: {name}");
            }
        } else {
            let current: Vec<String> = current.iter().map(|l| historical(l)).collect();
            assert_eq!(current, tubes, "historical box tube values/order changed: {section}");
        }
    }
    assert_eq!(got, include_str!("thinner/tubes_current.txt"), "current expanded tube table changed");
}
