use crate::layout::Rect;
use crate::modifier::Modifier;
use crate::renderer::{
    Color, draw_icon_chart, draw_icon_plus, draw_rounded_rect, draw_shadow, draw_text_smooth,
};
use crate::ui_context::Ui;

pub fn column(ui: &mut Ui, modifier: Modifier, content: impl FnOnce(&mut Ui)) {
    let padding = modifier.padding;
    let old_h = ui.cursor.h;
    let container_h = modifier.height.unwrap_or(0.0);

    let mut sub_ui = Ui {
        frame: &mut *ui.frame,
        width: ui.width,
        height: ui.height,
        font: ui.font,
        atlas: ui.atlas,
        state: ui.state.clone(), // CORREÇÃO: Clonar o Rc em vez de mover
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

    let final_h = if container_h > 0.0 {
        container_h
    } else {
        sub_ui.cursor.y - (ui.cursor.y + padding)
    };
    ui.cursor.y += final_h + padding;
}

pub fn sidebar_item(ui: &mut Ui, label: &str, active: bool) -> bool {
    let h = 45.0;
    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w: ui.cursor.w,
        h,
    };
    let hovered = ui.is_hovered(rect);
    let clicked = hovered && ui.input.mouse_clicked;

    let bg_color = if active {
        Color::BLUE.alpha(60)
    } else if hovered {
        Color::WHITE.alpha(20)
    } else {
        Color::TRANSPARENT
    };

    let text_color = if active {
        Color::WHITE
    } else {
        Color::WHITE.alpha(180)
    };

    draw_rounded_rect(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        ui.cursor.w,
        h,
        8.0,
        bg_color,
        ui.width,
        ui.height,
    );
    draw_text_smooth(
        ui.frame,
        ui.atlas,
        ui.font,
        16.0,
        ui.cursor.x + 15.0,
        ui.cursor.y + 12.0,
        label,
        text_color,
        ui.width,
        ui.height,
    );
    ui.cursor.y += h + 5.0;

    clicked
}

pub fn stat_card(ui: &mut Ui, label: &str, value: &str, color: Color) {
    let w = 260.0;
    let h = 120.0;
    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w,
        h,
    };
    let hovered = ui.is_hovered(rect);
    let bg_color = if hovered {
        Color::SLATE_800.alpha(255)
    } else {
        Color::SLATE_800
    };

    draw_shadow(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        w,
        h,
        12.0,
        4.0,
        ui.width,
        ui.height,
    );
    draw_rounded_rect(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        w,
        h,
        12.0,
        bg_color,
        ui.width,
        ui.height,
    );
    draw_text_smooth(
        ui.frame,
        ui.atlas,
        ui.font,
        14.0,
        ui.cursor.x + 15.0,
        ui.cursor.y + 20.0,
        label,
        Color::WHITE.alpha(140),
        ui.width,
        ui.height,
    );
    draw_text_smooth(
        ui.frame,
        ui.atlas,
        ui.font,
        32.0,
        ui.cursor.x + 15.0,
        ui.cursor.y + 45.0,
        value,
        Color::WHITE,
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
}

pub fn card(ui: &mut Ui, modifier: Modifier, content: impl FnOnce(&mut Ui)) {
    let bg_color = modifier.background.unwrap_or(Color::SLATE_800.alpha(240));
    let w = modifier.width.unwrap_or(ui.cursor.w);
    let h = modifier.height.unwrap_or(80.0);

    draw_shadow(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        w,
        h,
        12.0,
        4.0,
        ui.width,
        ui.height,
    );
    draw_rounded_rect(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        w,
        h,
        12.0,
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
        state: ui.state.clone(), // CORREÇÃO: Clonar o Rc
        input: ui.input,
        cursor: Rect {
            x: ui.cursor.x + 15.0,
            y: ui.cursor.y + 15.0,
            w: w - 30.0,
            h: h - 30.0,
        },
    };

    content(&mut sub_ui);
    ui.cursor.y += h + 15.0;
}

pub fn button(ui: &mut Ui, modifier: Modifier, label: &str) -> bool {
    let w = modifier.width.unwrap_or(140.0);
    let h = modifier.height.unwrap_or(40.0);
    let rect = Rect {
        x: ui.cursor.x,
        y: ui.cursor.y,
        w,
        h,
    };
    let hovered = ui.is_hovered(rect);
    let clicked = hovered && ui.input.mouse_clicked;

    let mut col = modifier.background.unwrap_or(Color::BLUE);
    if clicked {
        col = col.alpha(200);
    } else if hovered {
        col = col.alpha(230);
    }

    draw_rounded_rect(
        ui.frame,
        ui.cursor.x,
        ui.cursor.y,
        w,
        h,
        8.0,
        col,
        ui.width,
        ui.height,
    );

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
            18.0,
            tx,
            ty,
            label,
            Color::WHITE,
            ui.width,
            ui.height,
        );
    }
    ui.cursor.y += h + 8.0;

    clicked
}

pub fn text(ui: &mut Ui, content: &str) {
    let size = 18.0;
    draw_text_smooth(
        ui.frame,
        ui.atlas,
        ui.font,
        size,
        ui.cursor.x,
        ui.cursor.y,
        content,
        Color::WHITE,
        ui.width,
        ui.height,
    );
    ui.cursor.y += size + 10.0;
}

pub fn divider(ui: &mut Ui) {
    crate::renderer::draw_rect(
        ui.frame,
        ui.cursor.x as i32,
        ui.cursor.y as i32,
        ui.cursor.w as i32,
        1,
        Color::WHITE.alpha(30),
        ui.width,
        ui.height,
    );
    ui.cursor.y += 15.0;
}
