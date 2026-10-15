use super::Rect;

pub(crate) const CONTACT_CAPACITY: usize = 1024;

pub(crate) struct Window {
    pub interior: Rect,
    pub read_halo: Rect,
    pub generation: u64,
    pub weights: [f32; CONTACT_CAPACITY],
    pub indices: [usize; CONTACT_CAPACITY],
}
