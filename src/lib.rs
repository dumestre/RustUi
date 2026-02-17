pub mod core;
pub mod layout;
pub mod modifier;
pub mod renderer;
pub mod ui_context;
pub mod widgets;

pub use core::{App, run};
pub use layout::Rect;
pub use modifier::Modifier;
pub use renderer::Color;
pub use ui_context::Ui;
pub use widgets::*;

// Atalhos globais mapeados para os métodos Zen
pub fn pad(v: f32) -> Modifier {
    Modifier::DEFAULT.p(v)
}
pub fn bg(c: Color) -> Modifier {
    Modifier::DEFAULT.b(c)
}
pub fn sz(w: f32, h: f32) -> Modifier {
    Modifier::DEFAULT.s(w, h)
}
