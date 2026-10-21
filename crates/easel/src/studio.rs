use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

enum Profile {
    Developer,
    Painter { root: PathBuf, box_name: String },
}

static PROFILE: OnceLock<Profile> = OnceLock::new();

pub fn load() -> Result<(), String> {
    let exe = std::env::current_exe().and_then(|p| p.canonicalize()).map_err(|e| format!("easel: cannot locate its own executable: {e}"))?;
    let bin = exe.parent().ok_or("easel: executable has no directory")?;
    let file = bin.join("studio");
    let profile = match std::fs::read_to_string(&file) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Profile::Developer,
        Err(e) => return Err(format!("{}: {e}", file.display())),
        Ok(text) => match text.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["developer"] => Profile::Developer,
            ["painter", name] => {
                let name = if *name == "none" { paint::palette::DEFAULT_BOX } else { name };
                if paint::Palette::named_box(name).is_none() {
                    return Err("easel: invalid studio box".into());
                }
                Profile::Painter {
                    root: bin.parent().ok_or("easel: studio executable must be inside <studio>/bin")?.to_path_buf(),
                    box_name: name.to_string(),
                }
            }
            _ => return Err("easel: invalid studio setting".into()),
        },
    };
    PROFILE.set(profile).map_err(|_| "easel: studio setting already loaded".to_string())
}

pub fn painter() -> bool {
    matches!(PROFILE.get(), Some(Profile::Painter { .. }))
}

static SERVING: AtomicBool = AtomicBool::new(false);

/// This process is a session's server (`easel serve`), where a painter's Lua runs.
pub fn serving() {
    SERVING.store(true, Ordering::Relaxed);
}

/// A painter's live session: its Lua gets no finishing verbs and none of old logs' forms. A painter studio's other
/// commands (run, finish, render-checkpoint, ...) are the scripts' and replay with everything.
pub fn painter_lua() -> bool {
    painter() && SERVING.load(Ordering::Relaxed)
}

pub fn root() -> Option<&'static Path> {
    match PROFILE.get()? {
        Profile::Painter { root, .. } => Some(root),
        Profile::Developer => None,
    }
}

pub fn box_name() -> Option<&'static str> {
    match PROFILE.get()? {
        Profile::Painter { box_name, .. } => Some(box_name),
        Profile::Developer => None,
    }
}
