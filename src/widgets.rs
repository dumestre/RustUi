use crate::config::{components, font_size, render, spacing, text_alpha};
use crate::layout::Rect;
use crate::modifier::Modifier;
use crate::renderer::{
    Color, draw_icon_chart, draw_icon_plus, draw_rounded_rect, draw_shadow, draw_text_smooth,
};
use crate::ui_context::{Ui, AnimatedValue};



// ============================================================================
// SCROLL VIEW COM SUPORTE A SCROLL
// ============================================================================

pub fn scroll_view(ui: &mut Ui, modifier: Modifier, content: impl FnOnce(&mut Ui)) {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    let padding = modifier.padding;
    let container_h = modifier.height.unwrap_or(200.0);
    let container_w = modifier.width.unwrap_or(ui.cursor.w);

    // Background
    if let Some(bg) = modifier.background {
        draw_rounded_rect(
            ui.frame,
            ui.cursor.x,
            ui.cursor.y,
            container_w,
            container_h,
            components::CARD_BORDER_RADIUS,
            bg,
            ui.width,
            ui.height,
        );
    }

    // Check hover antes de modificar scroll
    let is_hovered = ui.is_hovered(Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w: container_w,
        h: container_h,
    });

    // Setup scroll state
    ui.scroll.viewport_height = container_h - padding * 2.0;
    ui.scroll.is_hovered = is_hovered;

    ui.handle_scroll();

    // Clip rect para conteúdo scrollable
    let old_clip = ui.clip_rect.clone();
    ui.clip_rect = Some(Rect {
        x: ui.cursor.x + padding,
        y: ui.cursor.y + padding,
        w: container_w - padding * 2.0 - components::SCROLLBAR_WIDTH,
        h: container_h - padding * 2.0,
    });

    let mut sub_ui = Ui {
        frame: &mut *ui.frame,
        width: ui.width,
        height: ui.height,
        font: ui.font,
        atlas: ui.atlas,
        state: ui.state.clone(),
        input: ui.input,
        cursor: Rect {
            x: ui.cursor.x + padding,
            y: ui.cursor.y + padding - ui.scroll.offset,
            w: container_w - padding * 2.0 - components::SCROLLBAR_WIDTH,
            h: 0.0,
        },
        clip_rect: ui.clip_rect.clone(),
        scroll: ui.scroll.clone(),
        depth: ui.depth + 1,
        widget_id_counter: ui.widget_id_counter,
    };

    content(&mut sub_ui);

    // Atualiza content height
    let content_h = sub_ui.cursor.y - (ui.cursor.y + padding) + ui.scroll.offset;
    ui.scroll.content_height = content_h;
    ui.scroll.viewport_height = container_h - padding * 2.0;
    ui.widget_id_counter = sub_ui.widget_id_counter;

    // Restore clip
    ui.clip_rect = old_clip;

    // Draw scrollbar
    ui.draw_scrollbar(
        ui.cursor.x + container_w - components::SCROLLBAR_WIDTH,
        ui.cursor.y + padding,
    );

    ui.cursor.y += container_h + spacing::SM;
    ui.pop_id();
}

// ============================================================================
// COLUMN COM LAYOUT FLEXBOX SIMPLES
// ============================================================================

pub fn column(ui: &mut Ui, modifier: Modifier, content: impl FnOnce(&mut Ui)) {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    let padding = modifier.padding;
    let old_h = ui.cursor.h;
    let container_h = modifier.height.unwrap_or(0.0);

    let mut sub_ui = Ui {
        frame: &mut *ui.frame,
        width: ui.width,
        height: ui.height,
        font: ui.font,
        atlas: ui.atlas,
        state: ui.state.clone(),
        input: ui.input,
        cursor: Rect {
            x: ui.cursor.x + padding,
            y: ui.cursor.y + padding,
            w: modifier
                .width
                .unwrap_or(ui.cursor.w - padding * 2.0)
                .max(0.0),
            h: 0.0,
        },
        clip_rect: ui.clip_rect.clone(),
        scroll: ui.scroll.clone(),
        depth: ui.depth + 1,
        widget_id_counter: ui.widget_id_counter,
    };

    if let Some(bg) = modifier.background {
        draw_rounded_rect(
            sub_ui.frame,
            ui.cursor.x,
            ui.cursor.y,
            ui.cursor.w,
            modifier.height.unwrap_or(old_h),
            0.0,
            bg,
            sub_ui.width,
            sub_ui.height,
        );
    }

    content(&mut sub_ui);

    ui.widget_id_counter = sub_ui.widget_id_counter;

    let final_h = if container_h > 0.0 {
        container_h
    } else {
        sub_ui.cursor.y - (ui.cursor.y + padding)
    };
    ui.cursor.y += final_h + padding;
    ui.pop_id();
}

// ============================================================================
// ROW PARA LAYOUT HORIZONTAL
// ============================================================================

pub fn row(ui: &mut Ui, modifier: Modifier, content: impl FnOnce(&mut Ui)) {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    let padding = modifier.padding;
    let container_h = modifier.height.unwrap_or(0.0);

    let mut sub_ui = Ui {
        frame: &mut *ui.frame,
        width: ui.width,
        height: ui.height,
        font: ui.font,
        atlas: ui.atlas,
        state: ui.state.clone(),
        input: ui.input,
        cursor: Rect {
            x: ui.cursor.x + padding,
            y: ui.cursor.y + padding,
            w: 0.0,
            h: container_h,
        },
        clip_rect: ui.clip_rect.clone(),
        scroll: ui.scroll.clone(),
        depth: ui.depth + 1,
        widget_id_counter: ui.widget_id_counter,
    };

    if let Some(bg) = modifier.background {
        draw_rounded_rect(
            sub_ui.frame,
            ui.cursor.x,
            ui.cursor.y,
            modifier.width.unwrap_or(ui.cursor.w),
            container_h,
            components::CARD_BORDER_RADIUS,
            bg,
            sub_ui.width,
            sub_ui.height,
        );
    }

    content(&mut sub_ui);

    ui.widget_id_counter = sub_ui.widget_id_counter;

    let final_w = sub_ui.cursor.x - (ui.cursor.x + padding);
    let final_h = if container_h > 0.0 {
        container_h
    } else {
        sub_ui.cursor.h
    };

    ui.cursor.x += final_w + padding;
    ui.cursor.h = final_h.max(ui.cursor.h);
    ui.pop_id();
}

// ============================================================================
// BUTTON COM ANIMAÇÃO
// ============================================================================

pub fn button(ui: &mut Ui, modifier: Modifier, label: &str) -> bool {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    let w = modifier.width.unwrap_or(140.0);
    let h = modifier.height.unwrap_or(components::BUTTON_HEIGHT);
    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w,
        h,
    };

    // Hover state com animação
    let hover_state = ui.use_state_with_id(widget_id, || AnimatedValue::new(0.0));
    let hovered = ui.is_hovered(rect);

    // Atualiza animação hover
    {
        let mut anim = hover_state.get();
        if hovered {
            anim.set(1.0, render::ANIMATION_DURATION_MS);
        } else {
            anim.set(0.0, render::ANIMATION_DURATION_MS);
        }
        hover_state.set(anim);
    }

    let hover_t = hover_state.get().update();

    // Click state
    let clicked = hovered && ui.input.mouse_just_clicked;
    let is_pressed = hovered && ui.input.mouse_clicked;

    // Interpola cor baseada no hover
    let base_col = modifier.background.unwrap_or(ui.theme().colors.primary);
    let hover_col = ui.theme().colors.primary_hover;

    let col = Color {
        r: (base_col.r as f32 + (hover_col.r as f32 - base_col.r as f32) * hover_t) as u8,
        g: (base_col.g as f32 + (hover_col.g as f32 - base_col.g as f32) * hover_t) as u8,
        b: (base_col.b as f32 + (hover_col.b as f32 - base_col.b as f32) * hover_t) as u8,
        a: if is_pressed { 200 } else { base_col.a },
    };

    draw_rounded_rect(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        w,
        h,
        components::BUTTON_BORDER_RADIUS,
        col,
        ui.width,
        ui.height,
    );

    // Render label ou ícone
    if label == "+" {
        draw_icon_plus(
            ui.frame,
            (ui.cursor.x + w / 2.0 - 7.0) as i32,
            (ui.cursor.y + h / 2.0 - 7.0) as i32,
            14,
            Color::WHITE,
            ui.width,
            ui.height,
        );
    } else {
        let tx = ui.cursor.x + (w - label.len() as f32 * 9.0) / 2.0;
        let ty = ui.cursor.y + (h - 18.0) / 2.0;
        draw_text_smooth(
            ui.frame,
            ui.atlas,
            ui.font,
            font_size::LG,
            tx,
            ty,
            label,
            Color::WHITE,
            ui.width,
            ui.height,
        );
    }

    ui.cursor.y += h + spacing::SM;
    ui.pop_id();

    clicked
}

// ============================================================================
// TEXT INPUT COM SUPORTE A TECLADO
// ============================================================================

pub fn text_input(ui: &mut Ui, modifier: Modifier, placeholder: &str) -> String {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    // Estado do input
    let text_state = ui.use_state_with_id(widget_id, || String::new());
    let focus_state = ui.use_state_with_id(widget_id + 1000, || false);

    let w = modifier.width.unwrap_or(200.0);
    let h = modifier.height.unwrap_or(components::BUTTON_HEIGHT);
    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w,
        h,
    };

    let hovered = ui.is_hovered(rect);
    let focused = focus_state.get();
    let text = text_state.get();

    // Click para focar
    if hovered && ui.input.mouse_just_clicked {
        focus_state.set(!focused);
    }

    // Perde foco ao clicar fora
    if focused && ui.input.mouse_just_clicked && !hovered {
        focus_state.set(false);
    }

    // Input de texto
    if focused {
        if let Some(ch) = ui.input.char_input {
            if ch == '\x08' {
                // Backspace
                let mut new_text = text.clone();
                new_text.pop();
                text_state.set(new_text);
            } else if ch >= ' ' && ch != '\x7f' {
                let mut new_text = text.clone();
                new_text.push(ch);
                text_state.set(new_text);
            }
        }

        // Ctrl+A para selecionar tudo (futuro)
        if ui.input.key_just_pressed(winit::event::VirtualKeyCode::A) && ui.input.ctrl() {
            // Select all logic here
        }
    }

    // Cores baseadas no estado
    let bg_col = if focused {
        ui.theme().colors.surface
    } else if hovered {
        ui.theme().colors.surface_hover
    } else {
        ui.theme().colors.surface
    };

    let border_col = if focused {
        ui.theme().colors.primary
    } else if hovered {
        ui.theme().colors.border
    } else {
        ui.theme().colors.border.alpha(100)
    };

    // Background e border
    draw_rounded_rect(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        w,
        h,
        components::BUTTON_BORDER_RADIUS,
        bg_col,
        ui.width,
        ui.height,
    );
    draw_rounded_rect(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        w,
        h,
        components::BUTTON_BORDER_RADIUS,
        border_col.alpha(80),
        ui.width,
        ui.height,
    );

    // Texto ou placeholder
    let display_text = if text.is_empty() && !focused {
        (placeholder, ui.theme().colors.text_muted)
    } else {
        (text.as_str(), ui.theme().colors.text_primary)
    };

    draw_text_smooth(
        ui.frame,
        ui.atlas,
        ui.font,
        font_size::MD,
        ui.cursor.x + spacing::MD,
        ui.cursor.y + 12.0,
        display_text.0,
        display_text.1,
        ui.width,
        ui.height,
    );

    // Cursor de texto piscando
    if focused {
        let blink_t = (ui.cursor.y as f64 * 0.005).sin();
        if blink_t > 0.0 {
            let caret_x = ui.cursor.x + spacing::MD + text.len() as f32 * 9.0;
            let caret_color = ui.theme().colors.text_primary;
            crate::renderer::draw_rect(
                ui.frame,
                caret_x as i32,
                (ui.cursor.y + 12.0) as i32,
                2,
                20,
                caret_color,
                ui.width,
                ui.height,
            );
        }
    }

    ui.cursor.y += h + spacing::SM;
    ui.pop_id();

    text
}

// ============================================================================
// SIDEBAR ITEM
// ============================================================================

pub fn sidebar_item(ui: &mut Ui, label: &str, active: bool) -> bool {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    let h = components::SIDEBAR_ITEM_HEIGHT;
    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w: ui.cursor.w,
        h,
    };

    // Hover com animação
    let hover_state = ui.use_state_with_id(widget_id, || AnimatedValue::new(0.0));
    let hovered = ui.is_hovered(rect);

    // Atualiza animação hover
    {
        let mut anim = hover_state.get();
        if hovered {
            anim.set(1.0, render::ANIMATION_DURATION_MS);
        } else {
            anim.set(0.0, render::ANIMATION_DURATION_MS);
        }
        hover_state.set(anim);
    }

    let hover_t = hover_state.get().update();
    let clicked = hovered && ui.input.mouse_just_clicked;

    // Interpola background
    let base_alpha = if active { 60 } else { 0 };
    let hover_alpha = 20;
    let alpha = (base_alpha as f32 + (hover_alpha as f32 - base_alpha as f32) * hover_t) as u8;

    let bg_color = if active {
        ui.theme().colors.primary.alpha(60)
    } else if hovered {
        ui.theme().colors.surface_hover
    } else {
        ui.theme().colors.background
    };

    let text_color = if active {
        ui.theme().colors.text_primary
    } else if hovered {
        ui.theme().colors.text_secondary
    } else {
        ui.theme().colors.text_secondary.alpha(text_alpha::SECONDARY)
    };

    draw_rounded_rect(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        ui.cursor.w,
        h,
        components::SIDEBAR_ITEM_BORDER_RADIUS,
        bg_color.alpha(alpha.max(base_alpha)),
        ui.width,
        ui.height,
    );
    draw_text_smooth(
        ui.frame,
        ui.atlas,
        ui.font,
        font_size::MD,
        ui.cursor.x + spacing::MD,
        ui.cursor.y + 12.0,
        label,
        text_color,
        ui.width,
        ui.height,
    );
    ui.cursor.y += h + spacing::SM;

    ui.pop_id();
    clicked
}

// ============================================================================
// STAT CARD
// ============================================================================

pub fn stat_card(ui: &mut Ui, label: &str, value: &str, color: Color) {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    let w = components::STAT_CARD_WIDTH;
    let h = components::STAT_CARD_HEIGHT;
    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w,
        h,
    };

    // Hover com animação de scale (simulado com shadow)
    let hover_state = ui.use_state_with_id(widget_id, || AnimatedValue::new(0.0));
    let hovered = ui.is_hovered(rect);

    // Atualiza animação hover
    {
        let mut anim = hover_state.get();
        if hovered {
            anim.set(1.0, render::ANIMATION_DURATION_MS);
        } else {
            anim.set(0.0, render::ANIMATION_DURATION_MS);
        }
        hover_state.set(anim);
    }

    let hover_t = hover_state.get().update();
    let shadow_offset = 4.0 + hover_t * 4.0;

    draw_shadow(
        ui.frame,
        ui.cursor.x + 2.0,
        ui.cursor.y + shadow_offset,
        w,
        h,
        components::CARD_BORDER_RADIUS,
        4.0,
        ui.width,
        ui.height,
    );

    let theme = ui.theme();
    let surface_color = theme.colors.surface;
    let text_secondary = theme.colors.text_secondary.alpha(text_alpha::TERTIARY);
    let text_primary = theme.colors.text_primary;
    drop(theme);

    draw_rounded_rect(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        w,
        h,
        components::CARD_BORDER_RADIUS,
        surface_color,
        ui.width,
        ui.height,
    );
    draw_text_smooth(
        ui.frame,
        ui.atlas,
        ui.font,
        font_size::SM,
        ui.cursor.x + spacing::MD,
        ui.cursor.y + spacing::LG,
        label,
        text_secondary,
        ui.width,
        ui.height,
    );
    draw_text_smooth(
        ui.frame,
        ui.atlas,
        ui.font,
        font_size::XXL,
        ui.cursor.x + spacing::MD,
        ui.cursor.y + 45.0,
        value,
        text_primary,
        ui.width,
        ui.height,
    );
    draw_icon_chart(
        ui.frame,
        (ui.cursor.x + 195.0) as i32,
        (ui.cursor.y + 55.0) as i32,
        30,
        color,
        ui.width,
        ui.height,
    );

    ui.cursor.y += h + spacing::MD;
    ui.pop_id();
}

// ============================================================================
// CARD
// ============================================================================

pub fn card(ui: &mut Ui, modifier: Modifier, content: impl FnOnce(&mut Ui)) {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    let bg_color = modifier
        .background
        .unwrap_or(ui.theme().colors.surface.alpha(240));
    let w = modifier.width.unwrap_or(ui.cursor.w);
    let h = modifier.height.unwrap_or(80.0);

    draw_shadow(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        w,
        h,
        components::CARD_BORDER_RADIUS,
        components::CARD_SHADOW_BLUR,
        ui.width,
        ui.height,
    );
    draw_rounded_rect(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        w,
        h,
        components::CARD_BORDER_RADIUS,
        bg_color,
        ui.width,
        ui.height,
    );

    let mut sub_ui = Ui {
        frame: &mut *ui.frame,
        width: ui.width,
        height: ui.height,
        font: ui.font,
        atlas: ui.atlas,
        state: ui.state.clone(),
        input: ui.input,
        cursor: Rect {
            x: ui.cursor.x + components::CARD_PADDING,
            y: ui.cursor.y + components::CARD_PADDING,
            w: w - components::CARD_PADDING * 2.0,
            h: h - components::CARD_PADDING * 2.0,
        },
        clip_rect: ui.clip_rect.clone(),
        scroll: ui.scroll.clone(),
        depth: ui.depth + 1,
        widget_id_counter: ui.widget_id_counter,
    };

    content(&mut sub_ui);

    ui.widget_id_counter = sub_ui.widget_id_counter;
    ui.cursor.y += h + spacing::LG;
    ui.pop_id();
}

// ============================================================================
// TEXT
// ============================================================================

pub fn text(ui: &mut Ui, content: &str) {
    let theme = ui.theme();
    let color = theme.colors.text_primary;
    drop(theme);
    draw_text_smooth(
        ui.frame,
        ui.atlas,
        ui.font,
        font_size::LG,
        ui.cursor.x,
        ui.cursor.y,
        content,
        color,
        ui.width,
        ui.height,
    );
    ui.cursor.y += font_size::LG + spacing::SM;
}

pub fn text_muted(ui: &mut Ui, content: &str) {
    let theme = ui.theme();
    let color = theme.colors.text_secondary;
    drop(theme);
    draw_text_smooth(
        ui.frame,
        ui.atlas,
        ui.font,
        font_size::MD,
        ui.cursor.x,
        ui.cursor.y,
        content,
        color,
        ui.width,
        ui.height,
    );
    ui.cursor.y += font_size::MD + spacing::SM;
}

pub fn text_heading(ui: &mut Ui, content: &str) {
    let theme = ui.theme();
    let color = theme.colors.text_primary;
    drop(theme);
    draw_text_smooth(
        ui.frame,
        ui.atlas,
        ui.font,
        font_size::XL,
        ui.cursor.x,
        ui.cursor.y,
        content,
        color,
        ui.width,
        ui.height,
    );
    ui.cursor.y += font_size::XL + spacing::MD;
}

// ============================================================================
// DIVIDER
// ============================================================================

pub fn divider(ui: &mut Ui) {
    let theme = ui.theme();
    let color = theme.colors.border.alpha(50);
    drop(theme);
    crate::renderer::draw_rect(
        ui.frame,
        ui.cursor.x as i32,
        ui.cursor.y as i32,
        ui.cursor.w as i32,
        components::DIVIDER_HEIGHT as i32,
        color,
        ui.width,
        ui.height,
    );
    ui.cursor.y += components::DIVIDER_MARGIN_Y;
}

// ============================================================================
// SPACER
// ============================================================================

pub fn spacer(ui: &mut Ui, height: f32) {
    ui.cursor.y += height;
}

pub fn hspacer(ui: &mut Ui, width: f32) {
    ui.cursor.x += width;
}
