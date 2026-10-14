use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};
use watercolor_pilot::{studio, Watercolor, TICK_NS};

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let result = (|| -> mlua::Result<()> {
        if args.len() < 4 || args[0] != "--box" || args[1] != "watercolor" {
            return Err(mlua::Error::external("explicit --box watercolor required; commands: init DIR, run DIR SCRIPT, render DIR OUTPUT.png, restore DIR NAME.json"));
        }
        let root = Path::new(&args[3]);
        let mut work = root.to_path_buf();
        let mut writer = None;
        if args.iter().any(|s| s == "--scratch") {
            let main = studio::load(root)?;
            let paper = main.canvas.as_ref().ok_or_else(|| {
                mlua::Error::external("scratch paper takes its size, kind and seed from the painting's paper, which doesn't exist yet; make it first with paper{} in a paint chunk without scratch: true")
            })?;
            let lab = paper
                .lab
                .as_ref()
                .ok_or_else(|| mlua::Error::external("scratch paper needs the Lab engine"))?;
            work = root.join("out/watercolor/scratch");
            if args[2] == "run"
                || args.iter().any(|s| s == "--new")
                || !work.join("checkpoint.json").exists()
            {
                fs::create_dir_all(&work).map_err(mlua::Error::external)?;
                writer = Some(studio::Writer::open(&work)?);
            }
            if !work.join("box").exists() {
                studio::init(&work)?;
            }
            if args.iter().any(|s| s == "--new") && work.join("checkpoint.json").exists() {
                let logs = root.join("paintings/lua");
                fs::create_dir_all(&logs).map_err(mlua::Error::external)?;
                let n = (1..)
                    .find(|n| !logs.join(format!("scratch-{n}.json")).exists())
                    .unwrap();
                let old = studio::load(&work)?;
                fs::write(
                    logs.join(format!("scratch-{n}.lua")),
                    old.applied_sources.join("\n\n"),
                )
                .map_err(mlua::Error::external)?;
                fs::rename(
                    work.join("checkpoint.json"),
                    logs.join(format!("scratch-{n}.json")),
                )
                .map_err(mlua::Error::external)?;
            }
            if !work.join("checkpoint.json").exists() {
                let mut fresh = studio::Session::default();
                fresh.engine_version = main.engine_version;
                if fresh.engine_version == 1 {
                    fresh.applied_sources.clear();
                }
                fresh.canvas = Some(
                    Watercolor::new_lab(
                        paper.width,
                        paper.height,
                        lab.simulation.paper_seed,
                        lab.simulation.paper_kind,
                    )
                    .map_err(mlua::Error::external)?,
                );
                fresh.applied_sources.push(format!(
                    "paper{{width={},height={},kind={},seed={}}}",
                    paper.width,
                    paper.height,
                    serde_json::to_string(&lab.simulation.paper_kind)
                        .map_err(mlua::Error::external)?,
                    lab.simulation.paper_seed
                ));
                fs::write(
                    work.join("checkpoint.json"),
                    serde_json::to_vec(&fresh).map_err(mlua::Error::external)?,
                )
                .map_err(mlua::Error::external)?;
            }
        } else if args.iter().any(|s| s == "--new") {
            return Err(mlua::Error::external("new_scratch needs scratch: true"));
        } else if args[2] == "run" {
            writer = Some(studio::Writer::open(&work)?);
        }
        let clock = |state: &studio::Session| {
            state.canvas.as_ref().map_or(0.0, |c| {
                (c.tick as f64 * TICK_NS as f64 + c.residual_ns as f64) / 1e9
            })
        };
        let stamp = |state: &studio::Session| {
            let seconds = clock(state) as u64 + 9 * 3600;
            format!(
                "day {}, {:02}:{:02}",
                seconds / 86400 + 1,
                seconds / 3600 % 24,
                seconds / 60 % 60
            )
        };
        match args[2].as_str() {
            "restore" if args.len() == 5 => studio::restore(root, &args[4])?,
            "init" if args.len() == 4 || (args.len() == 5 && args[4] == "--replay") => {
                if !root.join("box").exists() {
                    studio::init(root)?;
                    if args.len() == 5 {
                        let mut state = studio::Session::default();
                        state.engine_version = 1;
                        state.applied_sources.clear();
                        fs::write(
                            root.join("checkpoint.json"),
                            serde_json::to_vec(&state).map_err(mlua::Error::external)?,
                        )
                        .map_err(mlua::Error::external)?;
                    }
                }
            }
            "run" if args.len() >= 5 => {
                let source = if args[4] == "-" {
                    let mut text = String::new();
                    io::stdin()
                        .read_to_string(&mut text)
                        .map_err(mlua::Error::external)?;
                    text
                } else {
                    fs::read_to_string(&args[4]).map_err(mlua::Error::external)?
                };
                let state = writer.as_ref().unwrap().run(&source)?;
                let canvas = state
                    .canvas
                    .as_ref()
                    .ok_or_else(|| mlua::Error::external("the painting has no paper, so the chunk can't complete; add paper{} to this chunk and run it again"))?;
                let logs = root.join("paintings/lua");
                fs::create_dir_all(&logs).map_err(mlua::Error::external)?;
                fs::write(
                    logs.join(if work == root {
                        "painting.lua"
                    } else {
                        "scratch.lua"
                    }),
                    state.applied_sources.join("\n\n"),
                )
                .map_err(mlua::Error::external)?;
                if let Some(lab) = &canvas.lab {
                    println!(
                        "Lab GPU: {}x{} cells",
                        lab.simulation.width, lab.simulation.height
                    );
                }
                println!(
                    "{}\npainting time: {:.4}s\nok",
                    stamp(&state),
                    clock(&state)
                );
            }
            "render" if args.len() >= 5 => {
                let state = studio::load(&work)?;
                let canvas = state
                    .canvas
                    .as_ref()
                    .ok_or_else(|| mlua::Error::external("no paper state"))?;
                let mut mode = "gallery";
                let mut size = if canvas.schema == 5 {
                    [canvas.width as u32, canvas.height as u32]
                } else {
                    [
                        1200,
                        (1200.0 * canvas.height as f64 / canvas.width as f64)
                            .round()
                            .max(1.0) as u32,
                    ]
                };
                let mut crop = None;
                let options: Vec<_> = args[5..]
                    .iter()
                    .filter(|v| v.as_str() != "--scratch")
                    .collect();
                for option in options.chunks(2) {
                    if option.len() != 2 {
                        return Err(mlua::Error::external("look option needs a value"));
                    }
                    match option[0].as_str() {
                        "--mode" => mode = option[1],
                        "--size" => {
                            let long_side: u32 =
                                option[1].parse().map_err(mlua::Error::external)?;
                            if !(16..=2400).contains(&long_side) {
                                return Err(mlua::Error::external(
                                    "look size must be 16..2400 pixels",
                                ));
                            }
                            let scale = long_side as f64 / canvas.width.max(canvas.height) as f64;
                            size = [
                                (canvas.width as f64 * scale).round().max(1.0) as u32,
                                (canvas.height as f64 * scale).round().max(1.0) as u32,
                            ];
                        }
                        "--crop" => {
                            let values: Vec<u32> = option[1]
                                .split(',')
                                .map(str::parse)
                                .collect::<std::result::Result<_, _>>()
                                .map_err(mlua::Error::external)?;
                            if values.len() != 4 || values[2] <= values[0] || values[3] <= values[1]
                            {
                                return Err(mlua::Error::external("crop is x0,y0,x1,y1 in cells"));
                            }
                            crop = Some([
                                values[0],
                                values[1],
                                values[2] - values[0],
                                values[3] - values[1],
                            ]);
                        }
                        _ => {
                            return Err(mlua::Error::external(
                                "watercolor look supports mode, size and crop",
                            ))
                        }
                    }
                }
                if crop.is_some() {
                    size = [canvas.width as u32, canvas.height as u32];
                }
                let path = PathBuf::from(&args[4]);
                let path = if path.is_absolute() {
                    path
                } else {
                    root.join(path)
                };
                studio::render_size(canvas, &path, mode, size, crop)?;
                if mode == "color" {
                    for light in ["gallery", "relief"] {
                        let directory = path.parent().unwrap().join(format!(".{light}"));
                        fs::create_dir_all(&directory).map_err(mlua::Error::external)?;
                        studio::render_size(
                            canvas,
                            &directory.join(path.file_name().unwrap()),
                            light,
                            size,
                            crop,
                        )?;
                    }
                }
                println!("{}", path.strip_prefix(root).unwrap_or(&path).display());
            }
            "status" => {
                let mut state = studio::load(&work)?;
                if let Some(canvas) = &mut state.canvas {
                    let adapter = if let Some(lab) = &mut canvas.lab {
                        if canvas.schema == 5 {
                            lab.require_gpu().map_err(mlua::Error::external)?
                        } else {
                            lab.require_gpu()
                                .unwrap_or_else(|_| "Lab CPU fallback".into())
                        }
                    } else {
                        "legacy CPU".into()
                    };
                    println!(
                        "watercolor: {}x{} cells; {} named brushes; {adapter}",
                        canvas.width,
                        canvas.height,
                        state.brushes.len()
                    );
                } else {
                    println!("watercolor: no paper yet");
                }
                println!("{}\npainting time: {:.4}s", stamp(&state), clock(&state));
            }
            "log" => println!("{}", studio::load(&work)?.applied_sources.join("\n\n")),
            "note" if args.len() == 5 && args[4] == "-" => {
                let state = studio::load(root)?;
                let mut text = String::new();
                io::stdin()
                    .read_to_string(&mut text)
                    .map_err(mlua::Error::external)?;
                let path = root.join("notes/journal.md");
                let mut journal = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                    .map_err(mlua::Error::external)?;
                writeln!(
                    journal,
                    "- {}: {}",
                    stamp(&state),
                    text.trim().replace('\n', "\n  ")
                )
                .map_err(mlua::Error::external)?;
                println!("noted in notes/journal.md");
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
