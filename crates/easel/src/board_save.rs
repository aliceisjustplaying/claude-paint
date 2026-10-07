//! Exact, version-2 save representation of the palette board. Hex keeps the
//! key=value header single-line, including arbitrary heap names and float bits.
use crate::board::Board;
#[cfg(feature = "replay")]
use crate::board::Heap;

struct Writer(Vec<u8>);
impl Writer {
    fn u64(&mut self, v: u64) { self.0.extend_from_slice(&v.to_le_bytes()); }
    fn float(&mut self, v: f32) { self.0.extend_from_slice(&v.to_bits().to_le_bytes()); }
    fn flag(&mut self, v: bool) { self.0.push(u8::from(v)); }
    fn parts(&mut self, v: &[(usize, f32)]) {
        self.u64(v.len() as u64);
        for &(i, f) in v { self.u64(i as u64); self.float(f); }
    }
}
#[cfg(feature = "replay")]
struct Reader<'a>(&'a [u8]);
#[cfg(feature = "replay")]
impl Reader<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8], String> {
        if n > self.0.len() { return Err("the save's board is truncated".into()); }
        let (v, rest) = self.0.split_at(n); self.0 = rest; Ok(v)
    }
    fn u64(&mut self) -> Result<u64, String> { Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap())) }
    fn count(&mut self) -> Result<usize, String> {
        let n = usize::try_from(self.u64()?).map_err(|_| "the save's board count overflows")?;
        if n > self.0.len() { return Err("the save's board count exceeds its data".into()); }
        Ok(n)
    }
    fn float(&mut self) -> Result<f32, String> { Ok(f32::from_bits(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))) }
    fn flag(&mut self) -> Result<bool, String> {
        match self.take(1)?[0] { 0 => Ok(false), 1 => Ok(true), _ => Err("the save's board flag isn't 0 or 1".into()) }
    }
    fn parts(&mut self) -> Result<Vec<(usize, f32)>, String> {
        (0..self.count()?).map(|_| Ok((usize::try_from(self.u64()?).map_err(|_| "the save's board tube overflows")?, self.float()?))).collect()
    }
}
impl Board {
    pub(crate) fn is_default(&self) -> bool {
        self.heaps.is_empty() && self.next_id == 0 && self.dirty.to_bits() == 0 && self.set_out.is_none() && self.last.is_none() && self.residue.is_empty() && self.residue_amount.to_bits() == 0
    }
    pub(crate) fn save_text(&self) -> String {
        let mut w = Writer(Vec::new());
        w.u64(self.next_id); w.float(self.dirty);
        w.flag(self.set_out.is_some());
        if let Some(v) = &self.set_out { w.u64(v.len() as u64); for &i in v { w.u64(i as u64); } }
        w.flag(self.last.is_some());
        if let Some((v, f)) = &self.last { w.parts(v); w.float(*f); }
        w.parts(&self.residue); w.float(self.residue_amount);
        w.u64(self.heaps.len() as u64);
        for h in &self.heaps {
            w.u64(h.id); w.flag(h.name.is_some());
            if let Some(n) = &h.name { w.u64(n.len() as u64); w.0.extend_from_slice(n.as_bytes()); }
            w.parts(&h.parts);
            for f in [h.given_sum, h.medium, h.solvent, h.oil_rate, h.thinner] { w.float(f); }
            w.parts(&h.dirt);
            for f in [h.changed, h.added, h.scraped] { w.flag(f); }
        }
        w.0.iter().map(|b| format!("{b:02x}")).collect()
    }
    #[cfg(feature = "replay")]
    pub(crate) fn read_text(text: &str) -> Result<Self, String> {
        let bad = || "the save's board isn't hex".to_string();
        if text.len() % 2 != 0 || !text.is_ascii() { return Err(bad()); }
        let bytes = (0..text.len()).step_by(2).map(|i| u8::from_str_radix(&text[i..i+2], 16).map_err(|_| bad())).collect::<Result<Vec<_>, _>>()?;
        let mut r = Reader(&bytes);
        let next_id = r.u64()?; let dirty = r.float()?;
        let set_out = if r.flag()? { Some((0..r.count()?).map(|_| usize::try_from(r.u64()?).map_err(|_| "the save's board tube overflows".to_string())).collect::<Result<_, _>>()?) } else { None };
        let last = if r.flag()? { Some((r.parts()?, r.float()?)) } else { None };
        let residue = r.parts()?; let residue_amount = r.float()?;
        let mut heaps = Vec::new();
        for _ in 0..r.count()? {
            let id = r.u64()?;
            let name = if r.flag()? { let n = r.count()?; Some(String::from_utf8(r.take(n)?.to_vec()).map_err(|_| "the save's board name isn't UTF-8")?) } else { None };
            heaps.push(Heap { id, name, parts: r.parts()?, given_sum: r.float()?, medium: r.float()?, solvent: r.float()?, oil_rate: r.float()?, thinner: r.float()?, dirt: r.parts()?, changed: r.flag()?, added: r.flag()?, scraped: r.flag()? });
        }
        if !r.0.is_empty() { return Err("the save's board has trailing data".into()); }
        Ok(Board { heaps, next_id, dirty, set_out, last, residue, residue_amount })
    }
}
