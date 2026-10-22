use crate::api::Studio;
use paint::Mask;

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
