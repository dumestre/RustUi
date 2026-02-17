pub mod config;
pub mod core;
pub mod layout;
pub mod modifier;
pub mod renderer;
pub mod ui_context;
pub mod widgets;

// Core exports
pub use config::{get_system_font, get_font_with_fallback, Theme, ThemeColors};
pub use core::{App, run, DebugInfo, InputState};
pub use layout::Rect;
pub use modifier::Modifier;
pub use renderer::Color;
pub use ui_context::{Ui, AnimatedValue, ScrollState};
pub use widgets::*;

// Zen shortcuts
pub fn pad(v: f32) -> Modifier {
    Modifier::DEFAULT.p(v)
}

pub fn bg(c: Color) -> Modifier {
    Modifier::DEFAULT.b(c)
}

pub fn sz(w: f32, h: f32) -> Modifier {
    Modifier::DEFAULT.s(w, h)
}

// ============================================================================
// MACROS PARA DSL ZEN
// ============================================================================

/// Macro para criar UI de forma concisa
#[macro_export]
macro_rules! ui {
    ($ui:expr => { $($t:tt)* }) => {
        $crate::column($ui, $crate::pad(0.0), |ui| {
            $crate::ui_content!(ui => $($t)*);
        });
    };
}

#[macro_export]
macro_rules! ui_content {
    ($ui:expr => text $text:expr; $($rest:tt)*) => {
        $crate::text($ui, $text);
        $crate::ui_content!($ui => $($rest)*);
    };
    ($ui:expr => button $label:expr, $on_click:expr; $($rest:tt)*) => {
        if $crate::button($ui, $crate::bg($ui.theme().colors.primary), $label) {
            $on_click();
        }
        $crate::ui_content!($ui => $($rest)*);
    };
    ($ui:expr => card { $($inner:tt)* } $($rest:tt)*) => {
        $crate::card($ui, $crate::pad($crate::config::components::CARD_PADDING), |ui| {
            $crate::ui_content!(ui => $($inner)*);
        });
        $crate::ui_content!($ui => $($rest)*);
    };
    ($ui:expr => row { $($inner:tt)* } $($rest:tt)*) => {
        $crate::row($ui, $crate::pad(0.0), |ui| {
            $crate::ui_content!(ui => $($inner)*);
        });
        $crate::ui_content!($ui => $($rest)*);
    };
    ($ui:expr => ) => {};
}

/// Macro para definir tema customizado
#[macro_export]
macro_rules! theme {
    (name: $name:expr, primary: $primary:expr) => {
        Theme {
            name: $name,
            colors: ThemeColors {
                primary: $primary,
                ..Theme::dark().colors
            },
            ..Theme::dark()
        }
    };
    (dark) => { Theme::dark() };
    (light) => { Theme::light() };
}
