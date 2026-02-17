#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn contains(&self, p: (f32, f32)) -> bool {
        p.0 >= self.x && p.0 <= self.x + self.w && p.1 >= self.y && p.1 <= self.y + self.h
    }
}

pub enum LayoutType {
    Column,
    Row,
}

pub struct LayoutNode {
    pub rect: Rect,
    pub layout_type: LayoutType,
    pub spacing: f32,
}
