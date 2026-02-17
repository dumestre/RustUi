use crate::renderer::Color;

#[derive(Clone, Copy, Debug)]
pub struct Modifier {
    pub padding: f32,
    pub background: Option<Color>,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl Modifier {
    pub const DEFAULT: Modifier = Modifier {
        padding: 0.0,
        background: None,
        width: None,
        height: None,
    };
    pub fn new() -> Self {
        Self::DEFAULT
    }

    // Zen Methods (p, b, s) para encadeamento rápido
    pub fn p(self, v: f32) -> Self {
        self.padding(v)
    }
    pub fn b(self, c: Color) -> Self {
        self.background(c)
    }
    pub fn s(self, w: f32, h: f32) -> Self {
        self.size(w, h)
    }

    pub fn padding(mut self, val: f32) -> Self {
        self.padding = val;
        self
    }
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }
    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = Some(w);
        self.height = Some(h);
        self
    }
}
