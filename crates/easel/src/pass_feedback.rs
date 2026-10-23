use crate::api::Studio;
use paint::{Mask, Stage};
use std::collections::{HashMap, hash_map::Entry};

#[derive(Default)]
pub(crate) struct DryingFeedback {
    queries: Vec<(f32, f32, Stage)>,
    query_index: HashMap<(u32, u32), usize>,
    lines: usize,
    omitted: usize,
    #[cfg(test)]
    disabled: bool,
}

impl DryingFeedback {
    pub(crate) fn begin(&mut self) {
        self.queries.clear();
        self.query_index.clear();
        self.lines = 0;
        self.omitted = 0;
    }

    pub(crate) fn queried(&mut self, x: f32, y: f32, stage: Stage) {
        #[cfg(test)]
        if self.disabled { return; }
        if x.is_nan() || y.is_nan() {
            self.queries.push((x, y, stage));
            return;
        }
        let key = (if x == 0.0 { 0 } else { x.to_bits() }, if y == 0.0 { 0 } else { y.to_bits() });
        match self.query_index.entry(key) {
            Entry::Occupied(entry) => self.queries[*entry.get()].2 = stage,
            Entry::Vacant(entry) => {
                entry.insert(self.queries.len());
                self.queries.push((x, y, stage));
            }
        }
    }
}

pub(crate) fn drying_pass(s: &mut Studio, mask: &Mask, clip: Option<&Mask>, (x0, y0, x1, y1): (usize, usize, usize, usize), verb: &str) {
    #[cfg(test)]
    if s.drying_feedback.disabled { return; }
    let Some(c) = s.canvas.as_ref() else { return };
    let f = mask.f;
    let mut weights = [0.0f64; 4];
    for y in y0..y1 {
        for x in x0..x1 {
            let i = y * f.w + x;
            let w = mask.data[i] * clip.map_or(1.0, |m| m.data[i]);
            if w > 0.0 {
                weights[c.drying_at(f.ux(x), f.uy(y)) as usize] += w as f64;
            }
        }
    }
    let lines_before = s.drying_feedback.lines;
    let mut omitted = false;
    let total = weights.iter().sum::<f64>();
    if total > 0.0 && weights[1] + weights[2] >= total * 0.1 {
        if s.drying_feedback.lines < 3 {
            let setting = weights[1] * 100.0 / total;
            let tacky = weights[2] * 100.0 / total;
            s.out.push_str(&format!("{verb}: {:.0}% of this pass went onto setting or tacky paint (setting {setting:.0}%, tacky {tacky:.0}%).\n", setting + tacky));
            s.drying_feedback.lines += 1;
        } else {
            omitted = true;
        }
    }
    for &(x, y, stage) in &s.drying_feedback.queries {
        let i = f.index(x, y);
        if !f.holds(x, y) || mask.data[i] * clip.map_or(1.0, |m| m.data[i]) <= 0.0 || c.drying_at(x, y) != stage {
            continue;
        }
        if s.drying_feedback.lines < 3 {
            let name = match stage { Stage::Open => "open", Stage::Setting => "setting", Stage::Tacky => "tacky", Stage::Dry => "dry" };
            s.out.push_str(&format!("{verb}: the paint at {x},{y} was {name} when this pass went over it (drying() said so earlier in this chunk).\n"));
            s.drying_feedback.lines += 1;
        } else {
            omitted |= matches!(stage, Stage::Setting | Stage::Tacky);
        }
    }
    s.drying_feedback.omitted += usize::from(omitted && s.drying_feedback.lines == lines_before);
}

pub(crate) fn finish(s: &mut Studio) {
    if s.drying_feedback.omitted > 0 {
        s.out.push_str(&format!("(and {} more passes on setting or tacky paint)\n", s.drying_feedback.omitted));
    }
}

pub(crate) fn pass(s: &mut Studio, original: &Mask, mask: &Mask, clip: Option<&Mask>, options: &str, verb: &str, marks: u64) {
    #[cfg(test)]
    if s.pass_feedback_disabled {
        return;
    }
    let (mut total, mut kept) = (0.0, 0.0);
    for (i, &v) in original.data.iter().enumerate() {
        total += v.max(0.0) as f64;
        kept += (mask.data[i] * clip.map_or(1.0, |c| c.data[i])).max(0.0) as f64;
    }
    let scale2 = mask.f.scale as f64 * mask.f.scale as f64;
    let area = kept / scale2;
    if marks == 0 && kept == 0.0 {
        s.out.push_str(&format!("{verb}: nothing to paint: the mask covers 0 sq units (m:area() is 0), so this pass laid no paint.\n"));
    } else if marks == 0 && area < 25.0 {
        s.out.push_str(&format!("{verb}: almost nothing to paint: the mask covers only {area:.0} sq units, so this pass laid almost no paint.\n"));
    }
    if !options.is_empty() && total > 0.0 && kept < total * 0.05 {
        let original_area = total / scale2;
        s.out.push_str(&format!("{verb}: {options} kept {:.0}% of the mask to paint ({area:.0} of its {original_area:.0} sq units).\n", 100.0 * kept / total));
    }
}

#[cfg(test)]
mod tests {
    use crate::session::{Session, box_for};

    const CANVAS: &str = r#"canvas{size=200,aspect=1.25,linen=15,seed=7,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}
        p=pile{{"lead white",6},{"smalt",1},medium=0.2}"#;

    fn same(a: &Session, b: &Session) {
        let mut aa = Vec::new();
        let mut bb = Vec::new();
        a.canvas().unwrap().write_state(&mut aa, "").unwrap();
        b.canvas().unwrap().write_state(&mut bb, "").unwrap();
        assert_eq!(aa, bb, "feedback changed the canvas, paint state or clock");
        assert_eq!(a.st.borrow().calls, b.st.borrow().calls);
        assert_eq!(a.st.borrow().mark_effects, b.st.borrow().mark_effects);
        assert_eq!(a.st.borrow().clock, b.st.borrow().clock);
    }

    #[test]
    fn drying_queries_keep_first_notice_order_and_latest_stage() {
        let mut feedback = super::DryingFeedback::default();
        feedback.queried(0.0, 3.0, paint::Stage::Open);
        feedback.queried(2.0, 1.0, paint::Stage::Dry);
        feedback.queried(-0.0, 3.0, paint::Stage::Tacky);
        feedback.queried(2.0, 1.0, paint::Stage::Setting);
        assert_eq!(feedback.queries, vec![(0.0, 3.0, paint::Stage::Tacky), (2.0, 1.0, paint::Stage::Setting)]);
        feedback.queried(f32::NAN, 1.0, paint::Stage::Open);
        feedback.queried(f32::NAN, 1.0, paint::Stage::Dry);
        assert_eq!(feedback.queries.len(), 4);
        assert_eq!(feedback.query_index.len(), 2);
        feedback.begin();
        feedback.queried(2.0, 1.0, paint::Stage::Open);
        assert_eq!(feedback.queries, vec![(2.0, 1.0, paint::Stage::Open)]);
    }

    #[test]
    #[ignore]
    fn drying_full_canvas_mask_scales_linearly() {
        let mut previous = None;
        for width in [600, 1200, 2400] {
            let mut times = Vec::new();
            for _ in 0..3 {
                let mut s = Session::with_box(width, box_for(None).unwrap()).unwrap();
                s.run(CANVAS).unwrap();
                let t0 = std::time::Instant::now();
                s.run(r#"m=mask(function(x,y) return drying(x,y)=="dry" and 1 or 0 end)"#).unwrap();
                times.push(t0.elapsed().as_secs_f64());
                let st = s.st.borrow();
                let f = st.canvas.as_ref().unwrap().frame();
                assert_eq!(st.drying_feedback.queries.len(), f.w * f.h);
                assert_eq!(st.drying_feedback.query_index.len(), f.w * f.h);
            }
            times.sort_by(f64::total_cmp);
            let median = times[1];
            eprintln!("DRYING_MASK width={width} median_seconds={median:.6} samples={times:?}");
            if let Some(previous) = previous {
                let ratio = median / previous;
                eprintln!("DRYING_MASK four_times_pixels_ratio={ratio:.3}");
                assert!(ratio < 8.0, "four times the pixels took {ratio:.3} times as long");
            }
            previous = Some(median);
        }
    }

    #[test]
    fn drying_notices_only_change_feedback() {
        let mut on = Session::with_box(100, box_for(None).unwrap()).unwrap();
        let mut off = Session::with_box(100, box_for(None).unwrap()).unwrap();
        off.st.borrow_mut().drying_feedback.disabled = true;
        for s in [&mut on, &mut off] {
            s.run(CANVAS).unwrap();
            s.run(r#"m=rect(100,100,100,100); work(m,{pile=p,coverage=3,fill=true,clip=true,seed=1})"#).unwrap();
        }
        same(&on, &off);
        let src = r#"print(drying(150,150)); while drying(150,150)=="open" or drying(150,150)=="setting" do wait(10) end
            assert(drying(150,150)=="tacky")
            work(m,{pile=p,hand="glaze",coverage=0.5,clip=true,seed=2})"#;
        let out = on.run(src).unwrap().out;
        off.run(src).unwrap();
        assert!(out.contains("of this pass went onto setting or tacky paint"));
        assert!(out.contains("work: the paint at 150,150 was tacky when this pass went over it (drying() said so earlier in this chunk)."));
        same(&on, &off);
        let src = r#"for i=1,10 do blend(m,{coverage=0,seed=i}) end"#;
        let out = on.run(src).unwrap().out;
        off.run(src).unwrap();
        assert_eq!(out.lines().count(), 4, "{out}");
        assert!(out.ends_with("(and 7 more passes on setting or tacky paint)\n"), "{out}");
        assert!(!out.contains("drying()"));
        same(&on, &off);
    }

    #[test]
    fn drying_feedback_respects_weights_masks_and_query_age() {
        let mut on = Session::with_box(100, box_for(None).unwrap()).unwrap();
        let mut off = Session::with_box(100, box_for(None).unwrap()).unwrap();
        off.st.borrow_mut().drying_feedback.disabled = true;
        for s in [&mut on, &mut off] {
            s.run(CANVAS).unwrap();
            let out = s.run(r#"m=rect(100,100,100,100); work(m,{pile=p,coverage=3,fill=true,clip=true,seed=1})"#).unwrap().out;
            assert_eq!(out, "");
        }
        for (point, mask, stage) in [
            ("800,600", "rect(790,590,20,20)", "dry"),
            ("150,150", "m", "open"),
        ] {
            let src = format!(r#"for i=1,10 do assert(drying({point})=="{stage}"); work({mask},{{pile=p,coverage=0}}) end"#);
            let out = on.run(&src).unwrap().out;
            off.run(&src).unwrap();
            assert_eq!(out.lines().count(), 3, "{out}");
            assert!(out.lines().all(|line| line.contains(&format!("was {stage} when"))), "{out}");
            same(&on, &off);
        }
        let mut probe = on.canvas().unwrap().clone();
        assert_eq!(probe.drying_at(150.0, 150.0), paint::Stage::Open);
        let mut minutes = 0;
        while probe.drying_at(150.0, 150.0) != paint::Stage::Tacky {
            probe.wait(10.0);
            minutes += 10;
            assert!(minutes < 1440);
        }
        let src = format!(r#"assert(drying(150,150)=="open"); wait({minutes}); work(m,{{pile=p,coverage=0}})"#);
        let out = on.run(&src).unwrap().out;
        off.run(&src).unwrap();
        assert!(out.contains("of this pass went onto setting or tacky paint"), "{out}");
        assert!(!out.contains("drying()"), "{out}");
        same(&on, &off);
        let src = r#"drying(800,600)
            for i=1,2 do work(rect(790,590,20,20),{pile=p,coverage=0}) end
            assert(drying(150,150)=="tacky"); work(m,{pile=p,coverage=0})"#;
        let out = on.run(src).unwrap().out;
        off.run(src).unwrap();
        assert_eq!(out.lines().count(), 3, "{out}");
        assert!(out.lines().last().unwrap().contains("of this pass went onto setting or tacky paint"), "{out}");
        same(&on, &off);
        for src in [
            r#"stipple(m,{pile=p,coverage=0})"#,
            r#"blend(m,{coverage=0})"#,
            r#"assert(drying(150,150)=="tacky"); work(m,{pile=p,coverage=0,clip=rect(700,600,20,20)})"#,
            r#"assert(drying(800,600)=="dry"); work(m,{pile=p,coverage=0})"#,
        ] {
            let out = on.run(src).unwrap().out;
            off.run(src).unwrap();
            assert!(!out.contains("drying()"), "{out}");
            if src.starts_with("stipple") { assert!(out.starts_with("stipple: "), "{out}"); }
            if src.starts_with("blend") { assert!(out.starts_with("blend: "), "{out}"); }
            if src.contains("clip=") { assert!(!out.contains("of this pass"), "{out}"); }
            same(&on, &off);
        }
        let c = on.canvas().unwrap();
        let f = c.frame();
        let stages = c.stages();
        drop(c);
        let index = |stage| stages.iter().position(|&s| s == stage).unwrap();
        let (dry, setting, tacky) = (index(paint::Stage::Dry), index(paint::Stage::Setting), index(paint::Stage::Tacky));
        let mut mask = paint::Mask::empty(f);
        mask.data[dry] = 0.4;
        mask.data[setting] = 0.35;
        mask.data[tacky] = 0.25;
        let report = |mask: &paint::Mask, clip: Option<&paint::Mask>| {
            let mut s = on.st.borrow_mut();
            s.begin(99);
            super::drying_pass(&mut s, mask, clip, crate::api::support_with_pixels(Some(mask), f, 200.0).1, "work");
            s.out.clone()
        };
        assert_eq!(report(&mask, None), "work: 60% of this pass went onto setting or tacky paint (setting 35%, tacky 25%).\n");
        let mut clip = paint::Mask::full(f);
        clip.data[tacky] = 0.0;
        assert_eq!(report(&mask, Some(&clip)), "work: 47% of this pass went onto setting or tacky paint (setting 47%, tacky 0%).\n");
        mask.data[tacky] = 0.0;
        mask.data[setting] = 0.1;
        mask.data[dry] = 0.9;
        assert_eq!(report(&mask, None), "work: 10% of this pass went onto setting or tacky paint (setting 10%, tacky 0%).\n");
        mask.data[setting] = 0.099;
        assert_eq!(report(&mask, None), "");
    }

    #[test]
    #[ignore]
    fn drying_feedback_cost_200_small_passes_at_2400() {
        let src = r#"for i=1,200 do work(m,{pile=p,hand="glaze",coverage=0.5,clip=true,seed=i}) end"#;
        let (mut on, mut off) = (Vec::new(), Vec::new());
        let mut state = None;
        for pair in 1..=3 {
            for enabled in [true, false] {
                let mut s = Session::with_box(2400, box_for(None).unwrap()).unwrap();
                s.st.borrow_mut().drying_feedback.disabled = !enabled;
                s.run(CANVAS).unwrap();
                s.run(r#"m=rect(400,290,20,20)
                    work(rect(390,280,40,40),{pile=p,coverage=3,fill=true,clip=true,seed=7})
                    while drying(410,300)=="open" do wait(10) end
                    assert(drying(410,300)=="setting")"#).unwrap();
                let c = s.canvas().unwrap();
                assert_eq!((c.frame().w, c.frame().h), (2400, 1920));
                assert_eq!(c.drying_at(410.0, 300.0), paint::Stage::Setting);
                drop(c);
                let t0 = std::time::Instant::now();
                let out = s.run(src).unwrap().out;
                let elapsed = t0.elapsed().as_secs_f64();
                if enabled { on.push(elapsed); } else { off.push(elapsed); }
                assert_eq!(out.contains("of this pass went onto setting or tacky paint"), enabled, "{out}");
                eprintln!("DRYING_COST pair={pair} enabled={enabled} seconds={elapsed:.6}");
                let got = crate::state_digest_line(&s, 3, 0.0);
                if let Some(want) = &state { assert_eq!(&got, want); } else { state = Some(got); }
            }
        }
        on.sort_by(f64::total_cmp);
        off.sort_by(f64::total_cmp);
        let overhead = (on[1] / off[1] - 1.0) * 100.0;
        eprintln!("DRYING_COST median_on={:.6} median_off={:.6} overhead_percent={overhead:.3}", on[1], off[1]);
        eprintln!("DRYING_STATE identical across all six runs: {}", state.unwrap().trim());
        assert!(overhead < 5.0, "drying feedback overhead {overhead:.3}% exceeds 5%");
    }

    #[test]
    fn selection_notices_only_change_feedback() {
        let mut on = Session::with_box(1000, box_for(None).unwrap()).unwrap();
        let mut off = Session::with_box(1000, box_for(None).unwrap()).unwrap();
        off.st.borrow_mut().pass_feedback_disabled = true;
        for s in [&mut on, &mut off] {
            s.run(&CANVAS.replace("aspect=1.25", "aspect=4")).unwrap();
            s.run("w=world{horizon=120,eye=1.6,fov=50}; v=w:view()").unwrap();
        }
        for (src, expected) in [
            ("work(rect(0,0,0,0),{pile=p})", "work: nothing to paint: the mask covers 0 sq units (m:area() is 0), so this pass laid no paint.\n"),
            ("local m=rect(20,20,30,30); work(m-m,{pile=p})", "work: nothing to paint: the mask covers 0 sq units (m:area() is 0), so this pass laid no paint.\n"),
            ("work(rect(20,20,3,3),{pile=p,coverage=0})", "work: almost nothing to paint: the mask covers only 9 sq units, so this pass laid almost no paint.\n"),
            ("work(rect(20,20,3,4),{pile=p,coverage=0})", "work: almost nothing to paint: the mask covers only 12 sq units, so this pass laid almost no paint.\n"),
            ("work(rect(20,20,3,3),{pile=p})", ""),
            ("work(rect(20,20,3,4),{pile=p})", ""),
            ("work(rect(20,20,5,5),{pile=p})", ""),
            ("work(rect(20,20,100,100),{pile=p})", ""),
            ("local m=rect(100,20,44,30); work(m,{pile=p,behind=m-rect(100,20,2,20)})", "work: behind= kept 3% of the mask to paint (40 of its 1320 sq units).\n"),
            ("work(rect(100,20,44,30),{pile=p,clip=rect(100,20,2,20)})", "work: clip= kept 3% of the mask to paint (40 of its 1320 sq units).\n"),
            ("work(rect(100,20,100,100),{pile=p,clip=rect(100,20,5,100)})", ""),
            ("stipple(rect(0,0,0,0),{pile=p})", "stipple: nothing to paint: the mask covers 0 sq units (m:area() is 0), so this pass laid no paint.\n"),
            ("blend(rect(20,20,3,3),{coverage=0})", "blend: almost nothing to paint: the mask covers only 9 sq units, so this pass laid almost no paint.\n"),
            ("blend(rect(20,20,3,3),{})", ""),
            ("stipple(rect(100,100,3,3),{pile=p,width=20,coverage=100,seed=7})", ""),
        ] {
            assert_eq!(on.run(src).unwrap().out, expected, "{src}");
            assert_eq!(off.run(src).unwrap().out, "");
            same(&on, &off);
        }
        on = Session::with_box(200, box_for(None).unwrap()).unwrap();
        off = Session::with_box(200, box_for(None).unwrap()).unwrap();
        off.st.borrow_mut().pass_feedback_disabled = true;
        for s in [&mut on, &mut off] {
            s.run(CANVAS).unwrap();
            s.run("w=world{horizon=120,eye=1.6,fov=50}; v=w:view()").unwrap();
        }
        for (src, expected) in [
            ("stipple(rect(100,100,100,100),{pile=p,visible=mask(function() return 0.2 end)})", ""),
            ("stipple(rect(100,100,100,100),{pile=p,visible=mask(function() return 0.02 end)})", "stipple: visible= kept 2% of the mask to paint (200 of its 10000 sq units).\n"),
            ("stipple(rect(100,100,100,100),{pile=p,visible=mask(function() return 0.2 end),clip=mask(function() return 0.2 end)})", "stipple: visible=/clip= kept 4% of the mask to paint (400 of its 10000 sq units).\n"),
            ("work(rect(100,100,100,100),{pile=p,visible=mask(function() return 0.2 end)})", ""),
            ("blend(rect(100,100,100,100),{visible=mask(function() return 0.2 end)})", ""),
        ] {
            assert_eq!(on.run(src).unwrap().out, expected, "{src}");
            assert_eq!(off.run(src).unwrap().out, "");
            same(&on, &off);
        }
        on = Session::with_box(200, box_for(None).unwrap()).unwrap();
        off = Session::with_box(200, box_for(None).unwrap()).unwrap();
        off.st.borrow_mut().pass_feedback_disabled = true;
        for s in [&mut on, &mut off] {
            s.run(&CANVAS.replace("aspect=1.25", "aspect=1")).unwrap();
        }
        let before = on.canvas().unwrap().seen();
        let marks = on.canvas().unwrap().tally();
        let src = "work(rect(0,0,500,1000),{pile=p,hand=\"broad\",clip=rect(500,0,500,1000)})";
        assert_eq!(on.run(src).unwrap().out, "work: clip= kept 0% of the mask to paint (0 of its 500000 sq units).\n");
        assert_eq!(off.run(src).unwrap().out, "");
        assert!(on.canvas().unwrap().tally().since(&marks).strokes > 0);
        assert!(on.canvas().unwrap().seen() != before, "broad strokes must deposit paint across the clip boundary");
        same(&on, &off);
    }

    #[test]
    fn pile_shares_keep_the_given_parts_and_paint() {
        let mut s = Session::with_box(32, box_for(None).unwrap()).unwrap();
        s.run(CANVAS).unwrap();
        let before = s.st.borrow().clock;
        let out = s.run(r#"print(p); q=pile{{"lead white",0.6},{"smalt",0.1},medium=0.2}; print(q)
            local parts=p:parts(); assert(parts[1][2]==6 and parts[2][2]==1)
            print(p:add{{"smalt",5},medium=0.2}); print(p)"#).unwrap().out;
        assert_eq!(out, "pile(lead white 6 (86%), smalt 1 (14%); medium 0.2)\npile(lead white 0.6 (86%), smalt 0.1 (14%); medium 0.2)\npile(lead white 6 (50%), smalt 6 (50%); medium 0.2)\npile(lead white 6 (50%), smalt 6 (50%); medium 0.2)\n");
        assert!(s.st.borrow().clock > before);
        let mut off = Session::with_box(32, box_for(None).unwrap()).unwrap();
        off.run(CANVAS).unwrap();
        off.run(r#"q=pile{{"lead white",0.6},{"smalt",0.1},medium=0.2}; p:add{{"smalt",5},medium=0.2}"#).unwrap();
        same(&s, &off);
    }
}
