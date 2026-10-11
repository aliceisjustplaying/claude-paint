use std::{fs, path::Path};
use watercolor_pilot::studio;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let result = (|| -> mlua::Result<()> {
        if args.len() < 4 || args[0] != "--box" || args[1] != "watercolor" {
            return Err(mlua::Error::external("explicit --box watercolor required; commands: init DIR, run DIR SCRIPT, render DIR OUTPUT.png, restore DIR NAME.json"));
        }
        let root = Path::new(&args[3]);
        match args[2].as_str() {
            "restore" if args.len() == 5 => studio::restore(root, &args[4])?,
            "init" if args.len() == 4 => studio::init(root)?,
            "run" if args.len() == 5 => {
                let source = fs::read_to_string(&args[4]).map_err(mlua::Error::external)?;
                let state = studio::run(root, &source)?;
                let canvas = state
                    .canvas
                    .as_ref()
                    .ok_or_else(|| mlua::Error::external("chunk made no paper"))?;
                println!(
                    "watercolor: tick {}, {} named brushes; {}",
                    canvas.tick,
                    state.brushes.len(),
                    if canvas.lab.is_some() { "reference inventory diagnostics" } else { "conserved" }
                );
            }
            "render" if args.len() == 5 => {
                let state = studio::load(root)?;
                let canvas = state
                    .canvas
                    .as_ref()
                    .ok_or_else(|| mlua::Error::external("no paper state"))?;
                studio::render(canvas, Path::new(&args[4]), "gallery", 1200, None)?;
            }
            _ => return Err(mlua::Error::external("invalid watercolor command")),
        }
        Ok(())
    })();
    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
