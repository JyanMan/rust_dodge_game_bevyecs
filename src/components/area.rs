#[derive(Debug, Copy, Clone, Default)]
pub struct Area {
    pub x: f32, pub y: f32,
    pub w: f32, pub h: f32,
}

impl Area {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            x: x, y: y, w: w, h: h
        }
    }
}
