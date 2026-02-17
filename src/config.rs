/// Configurações globais do RustUI
/// Centraliza paths, constantes de layout, temas e configurações de renderização

use crate::renderer::Color;
use std::path::PathBuf;

// ============================================================================
// 1. FONT PATH MULTI-PLATAFORMA
// ============================================================================

/// Retorna o path de uma fonte do sistema baseada na plataforma
pub fn get_system_font(font_name: &str) -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let font_path = PathBuf::from("C:\\Windows\\Fonts")
            .join(format!("{}.ttf", font_name));
        if font_path.exists() {
            return Some(font_path);
        }
        let arial = PathBuf::from("C:\\Windows\\Fonts\\arial.ttf");
        if arial.exists() {
            return Some(arial);
        }
    }

    #[cfg(target_os = "macos")]
    {
        let font_path = PathBuf::from("/Library/Fonts")
            .join(format!("{}.ttf", font_name));
        if font_path.exists() {
            return Some(font_path);
        }
        let helvetica = PathBuf::from("/Library/Fonts/Helvetica.ttc");
        if helvetica.exists() {
            return Some(helvetica);
        }
    }

    #[cfg(target_os = "linux")]
    {
        let paths = [
            PathBuf::from("/usr/share/fonts/truetype")
                .join(format!("{}.ttf", font_name)),
            PathBuf::from("/usr/share/fonts/TTF")
                .join(format!("{}.ttf", font_name)),
            PathBuf::from("/usr/local/share/fonts")
                .join(format!("{}.ttf", font_name)),
        ];
        for path in &paths {
            if path.exists() {
                return Some(path.clone());
            }
        }
    }

    None
}

pub const DEFAULT_FONT_NAME: &str = "Arial";

// ============================================================================
// 2. SISTEMA DE TEMAS
// ============================================================================

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: &'static str,
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
    pub radius: ThemeRadius,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            name: "Dark",
            colors: ThemeColors {
                background: Color::SLATE_900,
                surface: Color::SLATE_800,
                surface_hover: Color { r: 51, g: 65, b: 85, a: 255 },
                primary: Color::BLUE,
                primary_hover: Color { r: 99, g: 102, b: 241, a: 255 },
                success: Color::GREEN,
                error: Color::RED,
                text_primary: Color::WHITE,
                text_secondary: Color { r: 148, g: 163, b: 184, a: 255 },
                text_muted: Color { r: 71, g: 85, b: 105, a: 255 },
                border: Color { r: 51, g: 65, b: 85, a: 255 },
                shadow: Color { r: 0, g: 0, b: 0, a: 80 },
            },
            spacing: ThemeSpacing::default(),
            radius: ThemeRadius::default(),
        }
    }

    pub fn light() -> Self {
        Self {
            name: "Light",
            colors: ThemeColors {
                background: Color { r: 248, g: 250, b: 252, a: 255 },
                surface: Color::WHITE,
                surface_hover: Color { r: 241, g: 245, b: 249, a: 255 },
                primary: Color { r: 59, g: 130, b: 246, a: 255 },
                primary_hover: Color { r: 37, g: 99, b: 235, a: 255 },
                success: Color { r: 22, g: 163, b: 74, a: 255 },
                error: Color { r: 220, g: 38, b: 38, a: 255 },
                text_primary: Color { r: 15, g: 23, b: 42, a: 255 },
                text_secondary: Color { r: 71, g: 85, b: 105, a: 255 },
                text_muted: Color { r: 148, g: 163, b: 184, a: 255 },
                border: Color { r: 226, g: 232, b: 240, a: 255 },
                shadow: Color { r: 0, g: 0, b: 0, a: 25 },
            },
            spacing: ThemeSpacing::default(),
            radius: ThemeRadius::default(),
        }
    }

    pub fn set_colors(&mut self, colors: ThemeColors) {
        self.colors = colors;
    }
}

#[derive(Clone, Debug)]
pub struct ThemeColors {
    pub background: Color,
    pub surface: Color,
    pub surface_hover: Color,
    pub primary: Color,
    pub primary_hover: Color,
    pub success: Color,
    pub error: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub border: Color,
    pub shadow: Color,
}

#[derive(Clone, Debug)]
pub struct ThemeSpacing {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
}

impl Default for ThemeSpacing {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
            xxl: 48.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThemeRadius {
    pub none: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub full: f32,
}

impl Default for ThemeRadius {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            full: 9999.0,
        }
    }
}

// ============================================================================
// 3. CONSTANTES DE LAYOUT
// ============================================================================

pub mod spacing {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 16.0;
    pub const LG: f32 = 24.0;
    pub const XL: f32 = 32.0;
    pub const XXL: f32 = 48.0;
}

pub mod font_size {
    pub const XS: f32 = 12.0;
    pub const SM: f32 = 14.0;
    pub const MD: f32 = 16.0;
    pub const LG: f32 = 18.0;
    pub const XL: f32 = 24.0;
    pub const XXL: f32 = 32.0;
    pub const XXXL: f32 = 48.0;
}

pub mod components {
    pub const BUTTON_HEIGHT: f32 = 40.0;
    pub const BUTTON_HEIGHT_LARGE: f32 = 48.0;
    pub const BUTTON_HEIGHT_SMALL: f32 = 32.0;
    pub const BUTTON_PADDING_X: f32 = 16.0;
    pub const BUTTON_BORDER_RADIUS: f32 = 8.0;

    pub const CARD_PADDING: f32 = 16.0;
    pub const CARD_BORDER_RADIUS: f32 = 12.0;
    pub const CARD_SHADOW_BLUR: f32 = 4.0;
    pub const CARD_SHADOW_OFFSET_Y: f32 = 2.0;

    pub const SIDEBAR_WIDTH: f32 = 260.0;
    pub const SIDEBAR_PADDING: f32 = 25.0;
    pub const SIDEBAR_ITEM_HEIGHT: f32 = 45.0;
    pub const SIDEBAR_ITEM_BORDER_RADIUS: f32 = 8.0;

    pub const STAT_CARD_WIDTH: f32 = 260.0;
    pub const STAT_CARD_HEIGHT: f32 = 120.0;

    pub const DIVIDER_HEIGHT: f32 = 1.0;
    pub const DIVIDER_MARGIN_Y: f32 = 15.0;

    pub const SCROLLBAR_WIDTH: f32 = 8.0;
    pub const SCROLLBAR_MIN_HEIGHT: f32 = 32.0;
}

pub mod text_alpha {
    pub const PRIMARY: u8 = 255;
    pub const SECONDARY: u8 = 180;
    pub const TERTIARY: u8 = 140;
    pub const DISABLED: u8 = 100;
}

pub mod render {
    pub const TEXT_ALPHA_THRESHOLD: f32 = 0.01;
    pub const SDF_QUALITY: f32 = 1.0;
    pub const ANIMATION_DURATION_MS: f64 = 150.0;
}

// ============================================================================
// 4. FONTES EMBUTIDAS (fallback)
// ============================================================================

/// Retorna dados de fonte com fallback em cascata
pub fn get_font_with_fallback(font_name: Option<&str>) -> Option<Vec<u8>> {
    // Tenta carregar do sistema
    if let Some(name) = font_name {
        if let Some(path) = get_system_font(name) {
            if let Ok(data) = std::fs::read(&path) {
                log::info!("Fonte carregada do sistema: {:?}", path);
                return Some(data);
            }
        }
    }

    // Tenta paths alternativos
    let fallback_paths = [
        "C:\\Windows\\Fonts\\arial.ttf",
        "C:\\Windows\\Fonts\\segoeui.ttf",
    ];

    for path in &fallback_paths {
        if let Ok(data) = std::fs::read(path) {
            log::info!("Fonte fallback carregada: {}", path);
            return Some(data);
        }
    }

    log::warn!("Nenhuma fonte encontrada - o usuário precisa fornecer uma fonte");
    None
}
