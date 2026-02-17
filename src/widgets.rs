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

pub fn scroll_view(ui: &mut Ui, modifier: Modifier, content: impl FnOnce(&mut Ui)) -> Rect {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    let padding = modifier.padding;
    let container_h = modifier.height.unwrap_or(200.0);
    let container_w = modifier.width.unwrap_or(ui.cursor.w);

    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w: container_w,
        h: container_h,
    };

    // Background
    if let Some(bg) = modifier.background {
        crate::renderer::draw_rounded_rect(
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
        max_y_seen: ui.cursor.y + padding - ui.scroll.offset, // Initialize max_y_seen for sub_ui
        max_x_seen: ui.cursor.x + padding, // Initialize max_x_seen for sub_ui
    };

    content(&mut sub_ui);

    // Atualiza content height
    let content_h = sub_ui.max_y_seen - (ui.cursor.y + padding) + ui.scroll.offset; // Use sub_ui.max_y_seen
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

    ui.pop_id();
    rect
}

// ============================================================================
// COLUMN COM LAYOUT FLEXBOX SIMPLES
// ============================================================================

pub fn column(ui: &mut Ui, modifier: Modifier, content: impl FnOnce(&mut Ui)) -> Rect {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    let padding = modifier.padding;
    let container_h = modifier.height.unwrap_or(0.0);
    let container_w = modifier
        .width
        .unwrap_or(ui.cursor.w - padding * 2.0)
        .max(0.0);

    let initial_x = ui.cursor.x;
    let initial_y = ui.cursor.y;

    let rect_for_parent = Rect { // This is the rect that the column itself occupies in the parent UI
        x: initial_x,
        y: initial_y,
        w: container_w + padding * 2.0,
        h: container_h, // Placeholder, will be updated after content
    };

    let mut sub_ui = Ui {
        frame: &mut *ui.frame,
        width: ui.width,
        height: ui.height,
        font: ui.font,
        atlas: ui.atlas,
        state: ui.state.clone(),
        input: ui.input,
        cursor: Rect {
            x: initial_x + padding,
            y: initial_y + padding,
            w: container_w, // Available width for children
            h: 0.0,
        },
        clip_rect: ui.clip_rect.clone(),
        scroll: ui.scroll.clone(),
        depth: ui.depth + 1,
        widget_id_counter: ui.widget_id_counter,
        max_y_seen: initial_y + padding, // Initialize max_y_seen for sub_ui
        max_x_seen: initial_x + padding, // Initialize max_x_seen for sub_ui
    };

    content(&mut sub_ui);

    ui.widget_id_counter = sub_ui.widget_id_counter;

    let actual_content_height = sub_ui.max_y_seen - (initial_y + padding);
    let final_h_for_column = if container_h > 0.0 {
        container_h
    } else {
        actual_content_height + padding // Account for bottom padding of the column
    };
    
    // Draw background if present, now that we have final_h_for_column
    if let Some(bg) = modifier.background {
        crate::renderer::draw_rounded_rect(
            ui.frame,
            rect_for_parent.x,
            rect_for_parent.y,
            rect_for_parent.w,
            final_h_for_column,
            0.0, // Assuming no rounded corners for column background
            bg,
            ui.width,
            ui.height,
        );
    }


    // Update parent ui's cursor
    ui.cursor.y = initial_y + final_h_for_column;
    ui.cursor.x = initial_x; // Reset x for next sibling
    ui.max_y_seen = ui.max_y_seen.max(ui.cursor.y);
    ui.max_x_seen = ui.max_x_seen.max(rect_for_parent.x + rect_for_parent.w);


    ui.pop_id();
    Rect { x: initial_x, y: initial_y, w: rect_for_parent.w, h: final_h_for_column } // Return actual rect consumed
}

// ============================================================================
// ROW PARA LAYOUT HORIZONTAL
// ============================================================================

pub fn row(ui: &mut Ui, modifier: Modifier, content: impl FnOnce(&mut Ui)) -> Rect {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    let padding = modifier.padding;
    let container_h = modifier.height.unwrap_or(0.0);
    let initial_x = ui.cursor.x;
    let initial_y = ui.cursor.y;

    let rect_for_parent = Rect { // This is the rect that the row itself occupies in the parent UI
        x: initial_x,
        y: initial_y,
        w: ui.cursor.w, // Row will take parent's width, children will arrange within
        h: container_h, // Placeholder, will be updated
    };

    let mut sub_ui = Ui {
        frame: &mut *ui.frame,
        width: ui.width,
        height: ui.height,
        font: ui.font,
        atlas: ui.atlas,
        state: ui.state.clone(),
        input: ui.input,
        cursor: Rect {
            x: initial_x + padding,
            y: initial_y + padding,
            w: 0.0, // Available width for children is handled by their own modifier or parent width
            h: container_h,
        },
        clip_rect: ui.clip_rect.clone(),
        scroll: ui.scroll.clone(),
        depth: ui.depth + 1,
        widget_id_counter: ui.widget_id_counter,
        max_y_seen: initial_y + padding, // Initialize max_y_seen for sub_ui
        max_x_seen: initial_x + padding, // Initialize max_x_seen for sub_ui
    };

    content(&mut sub_ui);

    ui.widget_id_counter = sub_ui.widget_id_counter;

    let actual_content_width = sub_ui.max_x_seen - (initial_x + padding);
    let actual_content_height = sub_ui.max_y_seen - (initial_y + padding);

    let final_w_for_row = if modifier.width.is_some() {
        modifier.width.unwrap()
    } else {
        actual_content_width + padding // Account for right padding
    };
    
    let final_h_for_row = if container_h > 0.0 {
        container_h
    } else {
        actual_content_height + padding // Account for bottom padding
    };

    // Draw background if present, now that we have final_w_for_row and final_h_for_row
    if let Some(bg) = modifier.background {
        crate::renderer::draw_rounded_rect(
            ui.frame,
            rect_for_parent.x,
            rect_for_parent.y,
            final_w_for_row,
            final_h_for_row,
            components::CARD_BORDER_RADIUS, // Assuming rows can have rounded corners too
            bg,
            ui.width,
            ui.height,
        );
    }

    // Update parent ui's cursor
    ui.cursor.y = initial_y + final_h_for_row; // Row consumes vertical space in parent
    ui.cursor.x = initial_x; // Reset x for next sibling
    ui.max_y_seen = ui.max_y_seen.max(ui.cursor.y);
    ui.max_x_seen = ui.max_x_seen.max(initial_x + final_w_for_row);

    ui.pop_id();
    Rect { x: initial_x, y: initial_y, w: final_w_for_row, h: final_h_for_row } // Return actual rect consumed
}

// ============================================================================
// BUTTON COM ANIMAÇÃO
// ============================================================================

pub fn button(ui: &mut Ui, modifier: Modifier, label: &str) -> (bool, Rect) {
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

    ui.pop_id();

    (clicked, rect)
}

// ============================================================================
// TEXT INPUT COM SUPORTE A TECLADO
// ============================================================================

pub fn text_input(ui: &mut Ui, modifier: Modifier, placeholder: &str) -> (String, Rect) {
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

    ui.pop_id();

    (text, rect)
}

// ============================================================================
// SIDEBAR ITEM
// ============================================================================

pub fn sidebar_item(ui: &mut Ui, label: &str, active: bool) -> (bool, Rect) {

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



    ui.pop_id();

    (clicked, rect)

}

// ============================================================================
// STAT CARD
// ============================================================================

pub fn stat_card(ui: &mut Ui, label: &str, value: &str, color: Color) -> Rect {
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

    ui.pop_id();
    rect
}

// ============================================================================
// CARD
// ============================================================================

pub fn card(ui: &mut Ui, modifier: Modifier, content: impl FnOnce(&mut Ui)) -> Rect {
    let widget_id = ui.next_widget_id();
    ui.push_id(widget_id);

    let bg_color = modifier
        .background
        .unwrap_or(ui.theme().colors.surface.alpha(240));
    let w = modifier.width.unwrap_or(ui.cursor.w);
    let h = modifier.height.unwrap_or(80.0);

    let rect = Rect { // Define rect here
        x: ui.cursor.x,
        y: ui.cursor.y,
        w,
        h,
    };

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
        max_y_seen: ui.cursor.y + components::CARD_PADDING, // Initialize max_y_seen for sub_ui
        max_x_seen: ui.cursor.x + components::CARD_PADDING, // Initialize max_x_seen for sub_ui
    };

    content(&mut sub_ui);

    ui.widget_id_counter = sub_ui.widget_id_counter; // This line was missing before!
    ui.pop_id();

    rect // Return rect
}

// ============================================================================
// TEXT
// ============================================================================

pub fn text(ui: &mut Ui, content: &str) -> Rect {
    let theme = ui.theme();
    let color = theme.colors.text_primary;
    drop(theme);
    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w: content.len() as f32 * font_size::LG * 0.5, // Approximate width
        h: font_size::LG + spacing::SM, // Approximate height
    };
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
    rect
}

pub fn text_muted(ui: &mut Ui, content: &str) -> Rect {
    let theme = ui.theme();
    let color = theme.colors.text_secondary;
    drop(theme);
    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w: content.len() as f32 * font_size::MD * 0.5, // Approximate width
        h: font_size::MD + spacing::SM, // Approximate height
    };
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
    rect
}

pub fn text_heading(ui: &mut Ui, content: &str) -> Rect {
    let theme = ui.theme();
    let color = theme.colors.text_primary;
    drop(theme);
    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w: content.len() as f32 * font_size::XL * 0.5, // Approximate width
        h: font_size::XL + spacing::MD, // Approximate height
    };
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
    rect
}

// ============================================================================
// DIVIDER
// ============================================================================

pub fn divider(ui: &mut Ui) -> Rect {
    let theme = ui.theme();
    let color = theme.colors.border.alpha(50);
    drop(theme);
    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w: ui.cursor.w,
        h: components::DIVIDER_HEIGHT,
    };
    crate::renderer::draw_rect(
        ui.frame,
        rect.x as i32,
        rect.y as i32,
        rect.w as i32,
        rect.h as i32,
        color,
        ui.width,
        ui.height,
    );
    rect
}

// ============================================================================
// SPACER
// ============================================================================

pub fn spacer(ui: &mut Ui, height: f32) -> Rect {
    Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w: ui.cursor.w, // Full available width
        h: height,
    }
}

pub fn hspacer(ui: &mut Ui, width: f32) -> Rect {
    Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w: width,
        h: ui.cursor.h, // Full available height
    }
}
