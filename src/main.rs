use ab_glyph::FontArc;
use rustui::core::{App, InputState, StateStore, run};
use rustui::renderer::{Color, FontAtlas, clear};
use rustui::widgets::{button, card, column, divider, sidebar_item, stat_card, text};
use rustui::{Ui, bg, pad, sz};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

struct MyApp {}

impl App for MyApp {
    fn update(&mut self, _i: &InputState) {}

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
        clear(frame, Color::SLATE_900);
        let mut ui = Ui::new(frame, w, h, font, atlas, state, input);

        // ESTADO REATIVO (HOOKS 🧠💎)
        // Usando Handles para evitar conflitos de borrow checker
        let revenue = ui.use_state(|| 142400);
        let active_tab = ui.use_state(|| 0);
        let revenue_val = revenue.get();
        let tab_val = active_tab.get();

        // Sidebar
        ui.cursor.w = 260.0;
        column(&mut ui, bg(Color::SLATE_800).p(25.0), |ui| {
            text(ui, "RUSTUI REACTIVE");
            divider(ui);
            ui.cursor.y += 20.0;

            if sidebar_item(ui, "DASHBOARD", tab_val == 0) {
                active_tab.set(0);
            }
            if sidebar_item(ui, "ANALYTICS", tab_val == 1) {
                active_tab.set(1);
            }
            sidebar_item(ui, "SETTINGS", false);

            ui.cursor.y += 40.0;
            // Botão Interativo com Hook
            if button(ui, bg(Color::BLUE).s(210.0, 45.0), "BOOST SALES +") {
                revenue.set(revenue_val + 1500);
            }
        });

        // Content Area
        ui.cursor.x = 300.0;
        ui.cursor.y = 40.0;
        ui.cursor.w = (w as f32 - 340.0).max(100.0);

        column(&mut ui, pad(0.0), |ui| {
            let title = if tab_val == 0 {
                "SYSTEM OVERVIEW"
            } else {
                "DETAILED ANALYTICS"
            };
            text(ui, title);
            divider(ui);
            ui.cursor.y += 30.0;

            let rev_str = format!("$ {},000", revenue_val / 1000);
            stat_card(ui, "TOTAL REVENUE", &rev_str, Color::GREEN);
            ui.cursor.x += 280.0;
            stat_card(ui, "ACTIVE USERS", "4,821", Color::BLUE);

            ui.cursor.x = 300.0;
            ui.cursor.y += 140.0;
            ui.cursor.y += 20.0;
            text(ui, "RECENT ACTIVITY LOGS");
            card(ui, sz(ui.cursor.w, 80.0).p(10.0), |ui| {
                text(ui, "System background check: OK");
            });
        });
    }
}

fn main() {
    env_logger::init();

    // Arial para visual Premium
    let font_path = "C:\\Windows\\Fonts\\arial.ttf";
    let font_data = fs::read(font_path).expect("Falha ao carregar fonte do sistema.");
    let font = FontArc::try_from_vec(font_data).unwrap();

    run(MyApp {}, 1100, 800, font);
}
