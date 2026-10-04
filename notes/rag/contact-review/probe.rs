// Temporary research instrumentation; excluded from the candidate patch.
use std::cell::RefCell;
pub struct Trace { pub contact: Vec<f32>, pub pickup: Vec<f32>, pub deposit: Vec<f32>, pub shapes: Vec<Vec<[f32;3]>>, step: usize }
thread_local! { static TRACE: RefCell<Option<Trace>> = const { RefCell::new(None) }; }
pub fn begin(n: usize) { put(Some(Trace { contact: vec![0.0;n], pickup: vec![0.0;n], deposit: vec![0.0;n], shapes: vec![], step: 0 })); }
pub fn take() -> Option<Trace> { TRACE.with(|s| s.borrow_mut().take()) }
pub fn put(t: Option<Trace>) { TRACE.with(|s| *s.borrow_mut() = t); }
// Diagnostic control: flatten only wet thickness, preserving all other fields.
pub fn uniform_film(c: &mut crate::Canvas, um: f32) {
    for v in &mut c.wet.vol { if *v > 0.0 { *v = um / crate::surface::COAT_UM; } }
    let f=c.window(); c.wet.touch(0,0,f.w,f.h);
}

pub fn shape(nodes: &[[f32;3]]) { TRACE.with(|s| { if let Some(t)=s.borrow_mut().as_mut() { if t.step % 88==0 { t.shapes.push(nodes.to_vec()); } t.step+=1; } }); }
