//! Reading a painting's own source for its stages: the stage names it
//! declares, where each stage's block ends, and the fingerprint of the code
//! a stage's checkpoint depends on (see `Run` in run.rs for the model).
//!
//! This is a small lexer, not a parser: it knows comments, string, raw
//! string and char literals (so braces in them don't count) and matches
//! parentheses and braces.

/// Which bytes of `src` are code (not in a comment or a literal).
pub(crate) fn code_mask(src: &str) -> Vec<bool> {
    let b = src.as_bytes();
    let n = b.len();
    let mut code = vec![true; n];
    let mut i = 0;
    let mark = |from: usize, to: usize, code: &mut Vec<bool>| code[from..to.min(n)].iter_mut().for_each(|c| *c = false);
    while i < n {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'/') => {
                let e = b[i..].iter().position(|&c| c == b'\n').map_or(n, |p| i + p);
                mark(i, e, &mut code);
                i = e;
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let (mut j, mut depth) = (i + 2, 1);
                while j < n && depth > 0 {
                    if b[j] == b'/' && b.get(j + 1) == Some(&b'*') {
                        depth += 1;
                        j += 2;
                    } else if b[j] == b'*' && b.get(j + 1) == Some(&b'/') {
                        depth -= 1;
                        j += 2;
                    } else {
                        j += 1;
                    }
                }
                mark(i, j, &mut code);
                i = j;
            }
            b'r' if (b.get(i + 1) == Some(&b'"') || b.get(i + 1) == Some(&b'#')) && (i == 0 || !is_ident(b[i - 1])) => {
                let hashes = b[i + 1..].iter().take_while(|&&c| c == b'#').count();
                if b.get(i + 1 + hashes) != Some(&b'"') {
                    i += 1;
                    continue;
                }
                let close: Vec<u8> = std::iter::once(b'"').chain(std::iter::repeat_n(b'#', hashes)).collect();
                let start = i + 2 + hashes;
                let e = b[start..].windows(close.len()).position(|w| w == close.as_slice()).map_or(n, |p| start + p + close.len());
                mark(i, e, &mut code);
                i = e;
            }
            b'"' => {
                let mut j = i + 1;
                while j < n && b[j] != b'"' {
                    j += if b[j] == b'\\' { 2 } else { 1 };
                }
                mark(i, j + 1, &mut code);
                i = j + 1;
            }
            b'\'' => {
                // a char literal ('x', '\n', '\u{..}', 'é') or a lifetime
                let j = if b.get(i + 1) == Some(&b'\\') {
                    b[i + 2..].iter().position(|&c| c == b'\'').map(|p| i + 2 + p)
                } else {
                    let len = src[i + 1..].chars().next().map_or(1, char::len_utf8);
                    (b.get(i + 1 + len) == Some(&b'\'')).then_some(i + 1 + len)
                };
                match j {
                    Some(j) => {
                        mark(i, j + 1, &mut code);
                        i = j + 1;
                    }
                    None => i += 1,
                }
            }
            _ => i += 1,
        }
    }
    code
}

fn is_ident(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

/// Byte offset of 1-based `line`, `col` (in chars).
fn offset(src: &str, line: u32, col: u32) -> Option<usize> {
    let start = if line <= 1 { 0 } else { src.match_indices('\n').nth(line as usize - 2)?.0 + 1 };
    let rest = &src[start..];
    let c = rest.char_indices().nth(col.saturating_sub(1) as usize).map_or(rest.len(), |(o, _)| o);
    Some(start + c)
}

/// The byte just after the bracket matching the open one at `open`.
fn close_of(b: &[u8], code: &[bool], open: usize) -> Option<usize> {
    let (o, c) = (b[open], if b[open] == b'(' { b')' } else { b'}' });
    let mut depth = 0i32;
    for i in open..b.len() {
        if !code[i] {
            continue;
        }
        if b[i] == o {
            depth += 1;
        } else if b[i] == c {
            depth -= 1;
            if depth == 0 {
                return Some(i + 1);
            }
        }
    }
    None
}

/// Line numbers (0-based) of the end of the block that the `stage(..)` call
/// at `line`, `col` opens (`if o.stage(..) {`), and of the end of the
/// top-level item (usually `fn main`) that holds it. None if the call isn't
/// directly followed by a block.
pub(crate) fn stage_block(src: &str, line: u32, col: u32) -> Option<(usize, usize)> {
    let b = src.as_bytes();
    let code = code_mask(src);
    let at = offset(src, line, col)?;
    let open = (at..b.len()).find(|&i| code[i] && b[i] == b'(')?;
    let after = close_of(b, &code, open)?;
    let brace = (after..b.len()).find(|&i| code[i] && !b[i].is_ascii_whitespace())?;
    if b[brace] != b'{' {
        return None;
    }
    let end = close_of(b, &code, brace)?;
    // depth at the call, then on to where the item closes
    let mut depth = 0i32;
    for i in 0..at {
        if code[i] {
            match b[i] {
                b'{' => depth += 1,
                b'}' => depth -= 1,
                _ => {}
            }
        }
    }
    let mut item = b.len();
    let mut d = depth;
    for i in at..b.len() {
        if code[i] {
            match b[i] {
                b'{' => d += 1,
                b'}' => {
                    d -= 1;
                    if d == 0 {
                        item = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    let line_of = |o: usize| src[..o].matches('\n').count();
    Some((line_of(end - 1), line_of(item.saturating_sub(1))))
}

/// Stage names given as string literals to `.stage(` calls, in order, and
/// whether some call names its stage another way (a variable in a loop).
pub(crate) fn declared_stages(src: &str) -> (Vec<String>, bool) {
    let b = src.as_bytes();
    let code = code_mask(src);
    let (mut names, mut dynamic) = (Vec::new(), false);
    for (i, _) in src.match_indices(".stage(") {
        if !code[i] {
            continue;
        }
        let j = (i + 7..b.len()).find(|&j| !b[j].is_ascii_whitespace());
        match j {
            Some(j) if b[j] == b'"' => {
                let e = (j + 1..b.len()).find(|&k| b[k] == b'"' && b[k - 1] != b'\\').unwrap_or(b.len());
                names.push(src[j + 1..e].to_string());
            }
            _ => dynamic = true,
        }
    }
    (names, dynamic)
}

/// A line's checkpoint tag: `// ckpt: from <stage>` (the stage, and
/// whether the tag stands alone on its line: then it opens a region up to
/// `// ckpt: end`; trailing code, it covers that line only), or the end of
/// a region.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Tag {
    From(String, bool),
    End,
}

/// Every line's tag. Only plain `//` comments count: not doc comments
/// (`///`, `//!`), not text inside another comment or a string.
pub(crate) fn tags(src: &str) -> Vec<Option<Tag>> {
    let code = code_mask(src);
    let mut at = 0;
    src.split('\n')
        .map(|line| {
            let start = at;
            at += line.len() + 1;
            let p = line.find("// ckpt:")?;
            let o = start + p;
            // a comment starts here: the byte before it is code (or the line start)
            let starts = code[o..].first() == Some(&false) && (p == 0 || code[o - 1]) && line.as_bytes().get(p + 2) != Some(&b'/');
            if !starts {
                return None;
            }
            let rest = line[p + "// ckpt:".len()..].trim();
            if rest == "end" {
                return Some(Tag::End);
            }
            let name = rest.strip_prefix("from ")?.trim();
            Some(Tag::From(name.to_string(), line[..p].trim().is_empty()))
        })
        .collect()
}

/// The lines (with their tags, from `tags`) that count for the checkpoint
/// of a stage, given which stage names are known to come no later than it
/// (`before`, as keys) and all stage names the painting declares (`known`,
/// as keys; a tag naming neither is an error): lines tagged for a later
/// stage are left out.
pub(crate) fn tagged_lines<'a>(lines: &[(&'a str, Option<Tag>)], before: &[String], known: &[String], key: impl Fn(&str) -> String) -> Result<Vec<&'a str>, String> {
    let mut out = Vec::new();
    let mut skip_region = false;
    for (l, t) in lines {
        match t {
            Some(Tag::End) => skip_region = false,
            Some(Tag::From(name, alone)) => {
                let k = key(name);
                if !known.contains(&k) && !before.contains(&k) {
                    return Err(format!("`// ckpt: from {name}` names no stage of this painting (in: {})", l.trim()));
                }
                let later = !before.contains(&k);
                if *alone {
                    skip_region = later;
                } else if !later && !skip_region {
                    out.push(*l);
                }
            }
            None if !skip_region => out.push(*l),
            None => {}
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_skips_comments_and_literals() {
        let s = "a { \"}\" '}' '\\'' r#\"}\"# // }\n /* { /* } */ */ 'a: loop {} }";
        let code = code_mask(s);
        let braces: String = s.bytes().zip(&code).filter(|(c, k)| **k && (*c == b'{' || *c == b'}')).map(|(c, _)| c as char).collect();
        assert_eq!(braces, "{{}}");
    }

    #[test]
    fn finds_stage_blocks_and_names() {
        let s = "fn main() {\n    let a = 1;\n    if o.stage(\"sky\", &mut c, &mut r) {\n        paint(\"}\");\n    }\n    let b = 2;\n    if o.stage(\"far range\", &mut c, &mut r) { x(); }\n    o.end(&mut c, &mut r);\n}\nfn helper() {}\n";
        assert_eq!(stage_block(s, 3, 8), Some((4, 8)));
        assert_eq!(stage_block(s, 7, 8), Some((6, 8)));
        assert_eq!(stage_block(s, 8, 5), None);
        assert_eq!(declared_stages(s), (vec!["sky".to_string(), "far range".to_string()], false));
        assert!(declared_stages("for n in x { if o.stage(n, &mut c, &mut ()) {} }").1);
        assert_eq!(declared_stages("// o.stage(\"no\")\n").0, Vec::<String>::new());
    }

    #[test]
    fn tags_only_in_plain_comments() {
        let s = "a(); // ckpt: from oak\n// ckpt: from sky\n/// // ckpt: from x\nlet s = \"// ckpt: from y\";\n// see `// ckpt: from z`\n    // ckpt: end\n";
        let t = tags(s);
        assert_eq!(t[0], Some(Tag::From("oak".into(), false)));
        assert_eq!(t[1], Some(Tag::From("sky".into(), true)));
        assert_eq!(&t[2..5], &[None, None, None]);
        assert_eq!(t[5], Some(Tag::End));
    }
}
