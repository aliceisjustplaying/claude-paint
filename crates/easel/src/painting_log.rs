//! The painting log's text, head and successful chunks. Configuration is supplied by
//! the caller; reading a log never consults the executable or environment.

use paint::Palette;
use std::fmt::Write as _;

pub const MARK: &str = "--@ chunk";
pub const BOX_MARK: &str = "--@ box";
pub const ENGINE_MARK: &str = "--@ engine";
pub const SKETCH_MARK: &str = "--@ sketch";

/// Head errors are retained separately: box lookup checks the engine first, while
/// chunk counting and sketch width have always accepted even an invalid head.
pub struct Head {
    box_name: Result<Option<String>, String>,
    engine: Result<u32, String>,
    pub sketch: bool,
}

impl Head {
    pub fn box_name(&self) -> Result<Option<&str>, String> {
        self.box_name
            .as_ref()
            .map(|n| n.as_deref())
            .map_err(Clone::clone)
    }

    pub fn engine(&self) -> Result<u32, String> {
        self.engine.clone()
    }

    /// A save names its box and engine directly, with the log's validation rules.
    #[cfg(feature = "replay")]
    pub fn saved(box_name: &str, engine: u32) -> Self {
        let box_name = if paint::palette::default_box() == Some(box_name) {
            Ok(None)
        } else {
            box_line(format!("{BOX_MARK} {box_name}").trim()).map(Some)
        };
        let engine = engine_line(&format!("{ENGINE_MARK} {engine}"));
        Self {
            box_name,
            engine,
            sketch: false,
        }
    }

    pub fn tubes(&self, configured: Option<&(String, String)>) -> Result<Palette, String> {
        let engine = self.engine()?;
        let logged = self.box_name()?;
        let Some(name) = logged.or(paint::palette::default_box()) else {
            return Err(format!(
                "the log names no box, so it was painted from the default box, which this easel doesn't have (its boxes: {}); nothing ran",
                box_list()
            ));
        };
        if let Some((c, from)) = configured
            && c != name
        {
            let log_says = match logged {
                Some(n) => format!("the log was painted from the box {n:?} (its {BOX_MARK} line)"),
                None => {
                    format!("the log names no box, so it was painted from the default box {name:?}")
                }
            };
            return Err(format!(
                "{log_says}, but {from} says {c:?}: a painting is replayed with the box it was painted from; nothing ran"
            ));
        }
        let mut b = find_box(name)?;
        b.engine = engine;
        Ok(b)
    }
}

pub struct PaintingLog {
    pub head: Head,
    pub chunks: Vec<String>,
    // Keep the source spelling too: old logs need not have canonical comments,
    // numbering, whitespace or even nonempty chunks. Rendering must not rewrite them.
    text: String,
}

impl PaintingLog {
    /// Scan the head and chunks once. Validation is demanded by the head's reader,
    /// not by parsing: `run` checks for no chunks before checking its box, for example.
    pub fn parse(text: &str) -> Self {
        let mut box_name = Ok(None);
        let mut engine = Ok(None);
        let mut sketch = false;
        let mut chunks = Vec::new();
        let mut cur: Option<String> = None;
        for line in text.lines() {
            if line.trim_start().starts_with(MARK) {
                if let Some(c) = cur.take() {
                    push_chunk(&mut chunks, c);
                }
                cur = Some(String::new());
            } else if let Some(c) = cur.as_mut() {
                c.push_str(line);
                c.push('\n');
            } else {
                let l = line.trim();
                // Once a scanner failed it used to return immediately. Preserve its
                // first error, independently of the other head fields' errors.
                if l.starts_with(BOX_MARK) && box_name.is_ok() {
                    box_name = box_line(l).and_then(|n| {
                        if box_name.as_ref().unwrap().is_some() {
                            Err(format!("the log names its box twice ({BOX_MARK} lines)"))
                        } else {
                            Ok(Some(n))
                        }
                    });
                }
                if l.starts_with(ENGINE_MARK) && engine.is_ok() {
                    engine = engine_line(l).and_then(|v| {
                        if engine.as_ref().unwrap().is_some() {
                            Err(format!(
                                "the log names its engine twice ({ENGINE_MARK} lines)"
                            ))
                        } else {
                            Ok(Some(v))
                        }
                    });
                }
                sketch |= l == SKETCH_MARK;
            }
        }
        if let Some(c) = cur {
            push_chunk(&mut chunks, c);
        }
        Self {
            head: Head {
                box_name,
                engine: engine.map(|v| v.unwrap_or(1)),
                sketch,
            },
            chunks,
            text: text.to_string(),
        }
    }

    /// Write the canonical log of a session, with the same text as the live writer.
    pub fn from_chunks<'a>(
        name: &str,
        tubes: &Palette,
        sketch: bool,
        chunks: impl Iterator<Item = &'a str>,
    ) -> Self {
        let mut text = String::new();
        let _ = writeln!(
            text,
            "-- easel session {name:?}: a painting replayed chunk by chunk."
        );
        let _ = writeln!(
            text,
            "-- Each {MARK:?} line starts one chunk as it was run at the easel."
        );
        let box_name =
            (paint::palette::default_box() != Some(tubes.name)).then(|| tubes.name.to_string());
        if let Some(b) = &box_name {
            let _ = writeln!(text, "{BOX_MARK} {b}");
        }
        if tubes.engine != 1 {
            let _ = writeln!(text, "{ENGINE_MARK} {}", tubes.engine);
        }
        if sketch {
            let _ = writeln!(text, "{SKETCH_MARK}");
        }
        let chunks: Vec<String> = chunks.map(str::to_string).collect();
        for (i, c) in chunks.iter().enumerate() {
            let _ = writeln!(text, "\n{MARK} {}", i + 1);
            text.push_str(c);
            text.push('\n');
        }
        Self {
            head: Head {
                box_name: Ok(box_name),
                engine: Ok(tubes.engine),
                sketch,
            },
            chunks,
            text,
        }
    }

    pub fn render(&self) -> &str {
        &self.text
    }

    pub fn into_text(self) -> String {
        self.text
    }
}

/// A submitted chunk's log spelling, or the same refusal the live session gives.
pub fn chunk_text(src: &str) -> Result<String, String> {
    let src = src.trim_end().to_string();
    if src.lines().any(|l| l.trim_start().starts_with(MARK)) {
        return Err(format!(
            "a chunk can't contain a line starting with {MARK:?} (it separates chunks in the log)"
        ));
    }
    if src.trim().is_empty() {
        return Err("empty chunk".into());
    }
    Ok(src)
}

fn push_chunk(chunks: &mut Vec<String>, c: String) {
    let c = c.trim_end();
    if !c.trim().is_empty() {
        chunks.push(c.to_string());
    }
}

fn box_line(l: &str) -> Result<String, String> {
    let rest = l.strip_prefix(BOX_MARK).unwrap();
    let name = rest.trim();
    if name.is_empty() || !rest.starts_with(' ') {
        return Err(format!(
            "the log's line {l:?} names no box ({BOX_MARK} <name>)"
        ));
    }
    Ok(name.to_string())
}

fn engine_line(l: &str) -> Result<u32, String> {
    let rest = l.strip_prefix(ENGINE_MARK).unwrap();
    rest.strip_prefix(' ')
        .and_then(|v| v.trim().parse::<u32>().ok())
        .filter(|v| (1..=paint::ENGINE).contains(v))
        .ok_or_else(|| {
            format!(
                "the log's line {l:?} names no engine this easel has ({ENGINE_MARK} <1 to {}>)",
                paint::ENGINE
            )
        })
}

pub fn find_box(name: &str) -> Result<Palette, String> {
    Palette::named_box(name)
        .ok_or_else(|| format!("this easel has no box {name:?} (its boxes: {})", box_list()))
}

fn box_list() -> String {
    Palette::box_names()
        .iter()
        .map(|b| format!("{b:?}"))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn box_for(
    log: Option<&PaintingLog>,
    configured: Option<&(String, String)>,
) -> Result<Palette, String> {
    match log {
        Some(log) => log.head.tubes(configured),
        None => find_box(configured.map_or(Palette::fallback_box(), |c| c.0.as_str())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Legacy comments, CRLFs and empty separators survive rendering, but only
    /// nonempty chunks execute. Marks within a chunk are not head declarations.
    #[test]
    fn legacy_text_keeps_its_spelling_and_chunk_boundaries() {
        let text = "-- old painting\r\n--@ sketch\r\n--@ sketch extra\r\n\r\n  --@ chunk 9 · clock 0\r\nx = 1  \r\n--@ box inness\r\n--@ chunk empty\r\n \r\n--@ chunks also separate\r\nprint(x)\r\n\r\n";
        let log = PaintingLog::parse(text);
        assert_eq!(log.render(), text);
        assert_eq!(log.head.box_name(), Ok(None));
        assert_eq!(log.head.engine(), Ok(1));
        assert!(log.head.sketch);
        assert_eq!(log.chunks, ["x = 1  \n--@ box inness", "print(x)"]);
    }

    /// The engine scanner's error wins over even an earlier malformed box;
    /// each scanner keeps its first error, and chunk counting still works.
    #[test]
    fn invalid_heads_keep_validation_order_and_exact_errors() {
        let log = PaintingLog::parse(
            "--@ boxes x\n--@ engine 0\n--@ engine 1\n--@ box inness\n--@ chunk 1\nx=1\n",
        );
        assert_eq!(
            log.head.box_name().unwrap_err(),
            "the log's line \"--@ boxes x\" names no box (--@ box <name>)"
        );
        assert_eq!(
            log.head.tubes(None).unwrap_err(),
            format!(
                "the log's line \"--@ engine 0\" names no engine this easel has (--@ engine <1 to {}>)",
                paint::ENGINE
            )
        );
        assert_eq!(log.chunks, ["x=1"]);
        for (head, error) in [
            (
                "--@ box inness\n--@ box inness",
                "the log names its box twice (--@ box lines)",
            ),
            (
                "--@ box inness\n--@ box",
                "the log's line \"--@ box\" names no box (--@ box <name>)",
            ),
            (
                "--@ engine 1\n--@ engine 1",
                "the log names its engine twice (--@ engine lines)",
            ),
        ] {
            assert_eq!(
                PaintingLog::parse(head).head.tubes(None).unwrap_err(),
                error
            );
        }
    }
}
