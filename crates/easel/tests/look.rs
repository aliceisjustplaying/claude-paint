#![cfg(feature = "replay")]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Easel(PathBuf);

impl Easel {
    fn call(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_easel"))
            .args(["-s", "view-test"])
            .args(args)
            .env("EASEL_ROOT", &self.0)
            .env("EASEL_WIDTH", "2400")
            .env("RAYON_NUM_THREADS", "2")
            .env_remove("EASEL_SESSION")
            .output().unwrap()
    }

    fn ok(&self, args: &[&str]) -> String {
        let result = self.call(args);
        assert!(result.status.success(), "{args:?}: {}{}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
        String::from_utf8(result.stdout).unwrap()
    }

    fn image(&self, args: &[&str]) -> (String, image::RgbImage) {
        let reply = self.ok(args);
        let image = image::open(reply.split_whitespace().next().unwrap()).unwrap().to_rgb8();
        (reply, image)
    }
}

impl Drop for Easel {
    fn drop(&mut self) {
        let _ = self.call(&["close"]);
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn wide_crops_and_named_mask_overlays_are_read_only_in_every_canvas_view() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("look-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let easel = Easel(root);
    easel.ok(&["open", "view-test"]);
    easel.ok(&["do", "canvas{size=300,aspect=5,linen=15,ground={{pile={{'lead white',1}},um=50,apply='knife'}}}; shape=rect(200,40,400,160); soft=rect(600,40,800,160):soften(10); p=pile{{'lead white',1}}"]);
    let log = easel.ok(&["log"]);
    let status = easel.ok(&["status"]);
    let (plain_reply, plain) = easel.image(&["look"]);
    assert_eq!(plain.dimensions(), (1000, 200));
    assert!(!plain_reply.contains("scale"));
    let (reply, wide) = easel.image(&["look", "--crop", "0,40,1000,140"]);
    assert_eq!(wide.dimensions(), (1200, 120));
    assert!(reply.contains("scale 1.2000 x 1.2000 px/unit"), "{reply}");
    let (reply, small) = easel.image(&["look", "--crop", "0,40,1000,140", "--size", "300"]);
    assert_eq!(small.dimensions(), (300, 30));
    assert!(reply.contains("scale 0.3000 x 0.3000 px/unit"), "{reply}");
    let (_, detail) = easel.image(&["look", "--crop", "100,0,500,200"]);
    assert_eq!(detail.dimensions(), (960, 480));
    let (_, overlaid) = easel.image(&["look", "--mask", "shape,soft"]);
    assert_eq!(overlaid.get_pixel(50, 100), plain.get_pixel(50, 100));
    assert_ne!(overlaid.get_pixel(300, 100), plain.get_pixel(300, 100));
    assert_ne!(overlaid.get_pixel(700, 100), plain.get_pixel(700, 100));
    let (_, after) = easel.image(&["look"]);
    assert_eq!(after, plain);
    for mode in ["value", "squint", "mirror", "gallery", "relief"] {
        let (_, image) = easel.image(&["look", "--crop", "100,0,500,200", "--mode", mode, "--mask", "shape"]);
        assert_eq!(image.dimensions(), detail.dimensions());
    }
    let survey = easel.ok(&["look", "--survey"]);
    assert!(survey.contains("survey: 1 rows x 2 columns of 500 x 200 units, at full detail"), "{survey}");
    assert_eq!(survey.matches("(1200x480)").count(), 2);
    let survey_mask = easel.ok(&["look", "--survey", "--mask", "shape"]);
    assert_eq!(survey_mask.matches("(1200x480)").count(), 2);
    let previous = plain_reply.split_whitespace().next().unwrap();
    let (_, compared) = easel.image(&["look", "--compare", previous, "--mask", "shape"]);
    assert_eq!(compared.dimensions(), (1612, 160));
    easel.image(&["look", "--hold", "p", "--at", "300,100", "--crop", "0,0,1000,200", "--mask", "shape"]);
    plain.save(easel.0.join("motif.png")).unwrap();
    let (_, reference) = easel.image(&["look", "--ref", "motif.png", "--crop", "0,40,1000,140"]);
    let (reply, reference_mask) = easel.image(&["look", "--ref", "motif.png", "--crop", "0,40,1000,140", "--mask", "shape"]);
    assert_eq!(reference.dimensions(), reference_mask.dimensions());
    assert_ne!(reference, reference_mask);
    assert!(reply.contains("scale 1.2000 x 1.2000 px/unit"), "{reply}");
    assert!(!easel.call(&["look", "--ref", "motif.png", "--mask", "missing"]).status.success());
    for args in [vec!["look", "--mask", "missing"], vec!["look", "--mask", "p"], vec!["look", "--palette", "--mask", "shape"]] {
        let result = easel.call(&args);
        assert!(!result.status.success(), "{args:?}");
    }
    assert_eq!(easel.ok(&["log"]), log);
    assert_eq!(easel.ok(&["status"]), status);
}
