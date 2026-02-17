use ab_glyph::FontArc;
use rustui::config::{Theme, spacing};
use rustui::core::{App, InputState, StateStore, run};
use rustui::renderer::{FontAtlas, clear};
use rustui::widgets::{
    button, card, column, divider, row, scroll_view, sidebar_item, spacer, stat_card, text,
    text_heading, text_input, text_muted,
};
use rustui::{bg, pad, sz, Ui};
use num_format::{Locale, ToFormattedString};
use std::cell::RefCell;

use std::rc::Rc;

struct MyApp {
    theme: Theme,
}

impl App for MyApp {
    fn update(&mut self, _input: &InputState) {}

    fn draw(
        &self,
        frame: &mut [u8],
        w: u32,
        h: u32,
        font: &FontArc,
        atlas: &mut FontAtlas,
        state: Rc<RefCell<StateStore>>,
        input: &InputState,
    ) {
        clear(frame, self.theme.colors.background);
        let mut ui = Ui::new(frame, w, h, font, atlas, state, input);

        // Toggle de tema com F2
        if input.key_just_pressed(winit::event::VirtualKeyCode::F2) {
            let mut store = ui.state.borrow_mut();
            store.theme = if store.theme.name == "Dark" {
                Theme::light()
            } else {
                Theme::dark()
            };
        }

        // ESTADO REATIVO COM IDS ESTÁVEIS
        let revenue = ui.use_state(|| 142400);
        let users = ui.use_state(|| 4821);
        let active_tab = ui.use_state(|| 0);
        let _search_text = ui.use_state(|| String::new());

        let revenue_val = revenue.get();
        let tab_val = active_tab.get();

        // Sidebar
        ui.cursor.w = 260.0;
        let sidebar_rect = column(&mut ui, bg(self.theme.colors.surface).p(25.0), |ui| {
            let heading_rect = text_heading(ui, "RUSTUI PRO ⚡");
            ui.cursor.y = heading_rect.y + heading_rect.h; // Advance cursor
            let muted_rect = text_muted(ui, "v2.0 - Todas Features");
            ui.cursor.y = muted_rect.y + muted_rect.h; // Advance cursor
            let divider_rect = divider(ui);
            ui.cursor.y = divider_rect.y + divider_rect.h; // Advance cursor
            let spacer_rect = spacer(ui, 20.0);
            ui.cursor.y = spacer_rect.y + spacer_rect.h; // Advance cursor

            let (clicked_dashboard, dashboard_rect) = sidebar_item(ui, "📊 Dashboard", tab_val == 0);
            if clicked_dashboard { active_tab.set(0); }
            ui.cursor.y = dashboard_rect.y + dashboard_rect.h; // Advance cursor

            let (clicked_analytics, analytics_rect) = sidebar_item(ui, "📈 Analytics", tab_val == 1);
            if clicked_analytics { active_tab.set(1); }
            ui.cursor.y = analytics_rect.y + analytics_rect.h; // Advance cursor

            let (clicked_settings, settings_rect) = sidebar_item(ui, "⚙️ Settings", tab_val == 2);
            if clicked_settings { active_tab.set(2); }
            ui.cursor.y = settings_rect.y + settings_rect.h; // Advance cursor

            let spacer_rect_2 = spacer(ui, 40.0);
            ui.cursor.y = spacer_rect_2.y + spacer_rect_2.h; // Advance cursor

            // Input de busca
            let (_search_text_val, search_rect) = text_input(ui, sz(210.0, 40.0), "Buscar...");
            ui.cursor.y = search_rect.y + search_rect.h; // Advance cursor

            let spacer_rect_3 = spacer(ui, 20.0);
            ui.cursor.y = spacer_rect_3.y + spacer_rect_3.h; // Advance cursor

            // Botão com animação
            let (clicked_boost, boost_btn_rect) = button(ui, bg(self.theme.colors.primary).s(210.0, 45.0), "🚀 Boost Sales");
            if clicked_boost { revenue.set(revenue_val + 1500); }
            ui.cursor.y = boost_btn_rect.y + boost_btn_rect.h; // Advance cursor

            // Botão para adicionar usuários
            let (clicked_add_users, add_users_btn_rect) = button(ui, bg(self.theme.colors.success).s(210.0, 45.0), "+ Add Users");
            if clicked_add_users { users.set(users.get() + 100); }
            ui.cursor.y = add_users_btn_rect.y + add_users_btn_rect.h; // Advance cursor
        });
        ui.cursor.y = sidebar_rect.y + sidebar_rect.h; // Column consumes vertical space
        ui.cursor.x = sidebar_rect.x + sidebar_rect.w; // Column consumes horizontal space (its own width)

        // Content Area
        ui.cursor.x = 300.0;
        ui.cursor.y = 40.0;
        ui.cursor.w = (w as f32 - 340.0).max(100.0);

        // ScrollView com conteúdo dinâmico
        let content_width = ui.cursor.w;
        let scroll_view_rect = scroll_view(&mut ui, bg(self.theme.colors.surface).s(content_width, 500.0), |ui| {
            let inner_column_rect = column(ui, pad(20.0), |ui| {
                // Header
                let title = match tab_val {
                    0 => "📊 SYSTEM OVERVIEW",
                    1 => "📈 DETAILED ANALYTICS",
                    2 => "⚙️ SETTINGS",
                    _ => "DASHBOARD",
                };
                let heading_rect = text_heading(ui, title);
                ui.cursor.y = heading_rect.y + heading_rect.h; // Advance cursor
                let divider_rect = divider(ui);
                ui.cursor.y = divider_rect.y + divider_rect.h; // Advance cursor
                let spacer_rect = spacer(ui, 30.0);
                ui.cursor.y = spacer_rect.y + spacer_rect.h; // Advance cursor

                // Stats cards em row (manually positioned)
                let original_cursor_x_for_row = ui.cursor.x;
                let original_cursor_y_for_row = ui.cursor.y;

                let rev_str = format!("$ {}K", (revenue_val / 1000).to_formatted_string(&Locale::en));
                let stat_card1_rect = stat_card(ui, "TOTAL REVENUE", &rev_str, self.theme.colors.success);
                ui.cursor.x = original_cursor_x_for_row + stat_card1_rect.w + spacing::LG;
                ui.cursor.y = original_cursor_y_for_row; // Reset Y for next card in "row"

                let stat_card2_rect = stat_card(ui, "ACTIVE USERS", &users.get().to_formatted_string(&Locale::en), self.theme.colors.primary);
                ui.cursor.x = original_cursor_x_for_row + stat_card1_rect.w + spacing::LG + stat_card2_rect.w + spacing::LG;
                ui.cursor.y = original_cursor_y_for_row; // Reset Y for next card in "row"

                let stat_card3_rect = stat_card(ui, "CONVERSION", "12.5%", self.theme.colors.error);
                
                // After the "row" of stat cards, advance Y by max height of the cards, and reset X
                let max_stat_card_height = stat_card1_rect.h.max(stat_card2_rect.h).max(stat_card3_rect.h);
                ui.cursor.x = original_cursor_x_for_row;
                ui.cursor.y = original_cursor_y_for_row + max_stat_card_height + spacing::MD;

                let spacer_rect_1 = spacer(ui, 40.0);
                ui.cursor.y = spacer_rect_1.y + spacer_rect_1.h; // Advance cursor

                // Cards de conteúdo
                let heading_rect_2 = text_heading(ui, "📋 RECENT ACTIVITY");
                ui.cursor.y = heading_rect_2.y + heading_rect_2.h; // Advance cursor
                let spacer_rect_2 = spacer(ui, 15.0);
                ui.cursor.y = spacer_rect_2.y + spacer_rect_2.h; // Advance cursor

                for i in 0..10 {
                    let card_rect = card(ui, sz(ui.cursor.w, 60.0).p(15.0), |ui_card_content| {
                        // Original row inside card is okay for now, as text/hspacer don't advance y directly anymore
                        let row_rect = row(ui_card_content, pad(0.0), |ui_row_content| {
                            let text_rect_in_card = text(ui_row_content, &format!("✅ Activity #{} - System check completed", i + 1));
                            ui_row_content.cursor.x = text_rect_in_card.x + text_rect_in_card.w;
                            ui_row_content.cursor.y = text_rect_in_card.y; // Reset Y

                            let hspacer_rect_in_card = hspacer(ui_row_content, 20.0);
                            ui_row_content.cursor.x = hspacer_rect_in_card.x + hspacer_rect_in_card.w;
                            ui_row_content.cursor.y = hspacer_rect_in_card.y; // Reset Y

                            let text_muted_rect_in_card = text_muted(ui_row_content, &format!("{}m ago", (i + 1) * 5));
                            ui_row_content.cursor.x = text_muted_rect_in_card.x + text_muted_rect_in_card.w;
                            ui_row_content.cursor.y = text_muted_rect_in_card.y; // Reset Y
                        });
                        ui_card_content.cursor.x = row_rect.x + row_rect.w;
                        ui_card_content.cursor.y = row_rect.y + row_rect.h; // Advance Y for next content in card
                    });
                    ui.cursor.y = card_rect.y + card_rect.h; // Advance cursor after card
                    let spacer_rect_4 = spacer(ui, 10.0);
                    ui.cursor.y = spacer_rect_4.y + spacer_rect_4.h; // Advance cursor
                }

                let spacer_rect_5 = spacer(ui, 40.0);
                ui.cursor.y = spacer_rect_5.y + spacer_rect_5.h; // Advance cursor

                // Settings section
                if tab_val == 2 {
                    let heading_rect_3 = text_heading(ui, "⚙️ CONFIGURAÇÕES");
                    ui.cursor.y = heading_rect_3.y + heading_rect_3.h; // Advance cursor
                    let spacer_rect_6 = spacer(ui, 20.0);
                    ui.cursor.y = spacer_rect_6.y + spacer_rect_6.h; // Advance cursor

                    let card_rect_2 = card(ui, sz(ui.cursor.w, 200.0).p(20.0), |ui_card_content| {
                        let text_rect_in_card = text(ui_card_content, "🎨 Aparência");
                        ui_card_content.cursor.y = text_rect_in_card.y + text_rect_in_card.h; // Advance cursor
                        let spacer_rect_in_card = spacer(ui_card_content, 10.0);
                        ui_card_content.cursor.y = spacer_rect_in_card.y + spacer_rect_in_card.h; // Advance cursor
                        let text_muted_rect_in_card = text_muted(ui_card_content, "Pressione F2 para alternar entre temas");
                        ui_card_content.cursor.y = text_muted_rect_in_card.y + text_muted_rect_in_card.h; // Advance cursor
                        let spacer_rect_in_card_2 = spacer(ui_card_content, 15.0);
                        ui_card_content.cursor.y = spacer_rect_in_card_2.y + spacer_rect_in_card_2.h; // Advance cursor
                        let text_muted_rect_in_card_2 = text_muted(ui_card_content, &format!("Tema atual: {}", self.theme.name));
                        ui_card_content.cursor.y = text_muted_rect_in_card_2.y + text_muted_rect_in_card_2.h; // Advance cursor
                    });
                    ui.cursor.y = card_rect_2.y + card_rect_2.h; // Advance cursor after card
                    let spacer_rect_7 = spacer(ui, 20.0);
                    ui.cursor.y = spacer_rect_7.y + spacer_rect_7.h; // Advance cursor

                    let card_rect_3 = card(ui, sz(ui.cursor.w, 150.0).p(20.0), |ui_card_content| {
                        let text_rect_in_card = text(ui_card_content, "📊 Debug Info");
                        ui_card_content.cursor.y = text_rect_in_card.y + text_rect_in_card.h; // Advance cursor
                        let spacer_rect_in_card = spacer(ui_card_content, 10.0);
                        ui_card_content.cursor.y = spacer_rect_in_card.y + spacer_rect_in_card.h; // Advance cursor
                        let text_muted_rect_in_card = text_muted(ui_card_content, "Pressione F3 para toggle do debug overlay");
                        ui_card_content.cursor.y = text_muted_rect_in_card.y + text_muted_rect_in_card.h; // Advance cursor
                    });
                    ui.cursor.y = card_rect_3.y + card_rect_3.h; // Advance cursor after card
                }

                let spacer_rect_8 = spacer(ui, 100.0); // Espaço extra para scroll
                ui.cursor.y = spacer_rect_8.y + spacer_rect_8.h; // Advance cursor
            }); // End of inner column
            ui.cursor.x = inner_column_rect.x + inner_column_rect.w; // Advance parent ui's x by column's width
            ui.cursor.y = inner_column_rect.y + inner_column_rect.h; // Advance parent ui's y by column's height
        }); // End of scroll_view
        ui.cursor.x = scroll_view_rect.x + scroll_view_rect.w; // Advance parent ui's x by scroll_view's width
        ui.cursor.y = scroll_view_rect.y + scroll_view_rect.h; // Advance parent ui's y by scroll_view's height
    }
}

fn main() {
    env_logger::init();

    // Carrega fonte com fallback
    let font_data = rustui::get_font_with_fallback(Some("Arial"))
        .expect("Falha ao carregar fonte. Instale uma fonte TrueType.");
    let font = FontArc::try_from_vec(font_data).unwrap();

    let app = MyApp {
        theme: Theme::dark(),
    };

    run(app, 1200, 900, font);
}

// Importa macros
use rustui::hspacer;
