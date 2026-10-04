#[path = "@SOURCE@/crates/easel/src/api.rs"] mod api;
#[path = "@SOURCE@/crates/easel/src/depth.rs"] mod depth;
#[path = "@SOURCE@/crates/easel/src/draw_edges.rs"] mod draw_edges;
#[path = "@SOURCE@/crates/easel/src/draw_outline.rs"] mod draw_outline;
#[path = "@SOURCE@/crates/easel/src/form.rs"] mod form;
#[path = "@SOURCE@/crates/easel/src/look.rs"] mod look;
#[path = "@SOURCE@/crates/easel/src/save.rs"] mod save;
#[path = "@SOURCE@/crates/easel/src/session.rs"] mod session;
#[path = "@SOURCE@/crates/easel/src/time.rs"] mod time;
#[path = "@SOURCE@/crates/easel/src/world.rs"] mod world;fn snap(s:&session::Session)->(f64,f64){let g=s.st.borrow();(g.clock,g.canvas.as_ref().unwrap().hand_owed_secs())}
fn main(){
 let mut s=session::Session::with_box(64,paint::Palette::named_box("tube box").unwrap()).unwrap();
 s.run(r#"canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}};p=pile{{"red earth",1}};b=brush("round",4);b:load(p)"#).unwrap();
 let start=snap(&s);println!("start {:?}",start);
 let mut crossed=false;
 for i in 1..300 {let before=snap(&s);s.lua.load("b:touch(100,80)").exec().unwrap();let after=snap(&s);if i==1 || after.0>before.0 {println!("touch {i} before={before:?} after={after:?}");}if after.0>before.0 {assert!(before.1<60.0);assert_eq!(after.1,0.0);assert!(after.0-before.0>=1.0);crossed=true;break;}else {assert!(after.1<60.0);}}
 assert!(crossed);
 s.lua.load("b:touch(100,80)").exec().unwrap();let before=snap(&s);assert!(before.1>0.0);s.lua.load("drying(100,80)").exec().unwrap();let after=snap(&s);println!("query {before:?} -> {after:?}");assert_eq!(after.1,0.0);assert!(after.0>before.0);
 s.lua.load("b:touch(100,80)").exec().unwrap();let before=snap(&s);s.lua.load("wait(0)").exec().unwrap();let after=snap(&s);println!("wait0 {before:?} -> {after:?}");assert_eq!(after.1,0.0);assert!(after.0>before.0);
 let before=snap(&s);s.run("b:touch(100,80)").unwrap();let after=snap(&s);println!("chunk {before:?} -> {after:?}");assert_eq!(after.1,0.0);assert!(after.0>before.0);
}
