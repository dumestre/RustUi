use ab_glyph::FontArc;
use rustui::config::Theme;
use rustui::core::{App, InputState, StateStore, run};
use rustui::renderer::{FontAtlas, clear};
use rustui::widgets::{
    button, card, column, divider, row, scroll_view, sidebar_item, spacer, stat_card, text,
    text_heading, text_input, text_muted,
};
use rustui::{bg, pad, sz, Ui};
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
        column(&mut ui, bg(self.theme.colors.surface).p(25.0), |ui| {
            text_heading(ui, "RUSTUI PRO ⚡");
            text_muted(ui, "v2.0 - Todas Features");
            divider(ui);
            spacer(ui, 20.0);

            if sidebar_item(ui, "📊 Dashboard", tab_val == 0) {
                active_tab.set(0);
            }
            if sidebar_item(ui, "📈 Analytics", tab_val == 1) {
                active_tab.set(1);
            }
            if sidebar_item(ui, "⚙️ Settings", tab_val == 2) {
                active_tab.set(2);
            }

            spacer(ui, 40.0);

            // Input de busca
            let _search = text_input(ui, sz(210.0, 40.0), "Buscar...");

            spacer(ui, 20.0);

            // Botão com animação
            if button(ui, bg(self.theme.colors.primary).s(210.0, 45.0), "🚀 Boost Sales") {
                revenue.set(revenue_val + 1500);
            }

            // Botão para adicionar usuários
            if button(ui, bg(self.theme.colors.success).s(210.0, 45.0), "+ Add Users") {
                users.set(users.get() + 100);
            }
        });

        // Content Area
        ui.cursor.x = 300.0;
        ui.cursor.y = 40.0;
        ui.cursor.w = (w as f32 - 340.0).max(100.0);

        // ScrollView com conteúdo dinâmico
        let content_width = ui.cursor.w;
        scroll_view(&mut ui, bg(self.theme.colors.surface).s(content_width, 500.0), |ui| {
            column(ui, pad(20.0), |ui| {
                // Header
                let title = match tab_val {
                    0 => "📊 SYSTEM OVERVIEW",
                    1 => "📈 DETAILED ANALYTICS",
                    2 => "⚙️ SETTINGS",
                    _ => "DASHBOARD",
                };
                text_heading(ui, title);
                divider(ui);
                spacer(ui, 30.0);

                // Stats cards em row
                row(ui, pad(0.0), |ui| {
                    let rev_str = format!("$ {}K", revenue_val / 1000);
                    stat_card(ui, "TOTAL REVENUE", &rev_str, self.theme.colors.success);
                    hspacer(ui, 20.0);
                    stat_card(ui, "ACTIVE USERS", &format!("{}", users.get()), self.theme.colors.primary);
                    hspacer(ui, 20.0);
                    stat_card(ui, "CONVERSION", "12.5%", self.theme.colors.error);
                });

                spacer(ui, 40.0);

                // Cards de conteúdo
                text_heading(ui, "📋 RECENT ACTIVITY");
                spacer(ui, 15.0);

                for i in 0..10 {
                    card(ui, sz(ui.cursor.w, 60.0).p(15.0), |ui| {
                        row(ui, pad(0.0), |ui| {
                            text(ui, &format!("✅ Activity #{} - System check completed", i + 1));
                            hspacer(ui, 20.0);
                            text_muted(ui, &format!("{}m ago", (i + 1) * 5));
                        });
                    });
                    spacer(ui, 10.0);
                }

                spacer(ui, 40.0);

                // Settings section
                if tab_val == 2 {
                    text_heading(ui, "⚙️ CONFIGURAÇÕES");
                    spacer(ui, 20.0);

                    card(ui, sz(ui.cursor.w, 200.0).p(20.0), |ui| {
                        text(ui, "🎨 Aparência");
                        spacer(ui, 10.0);
                        text_muted(ui, "Pressione F2 para alternar entre temas");
                        spacer(ui, 15.0);
                        text_muted(ui, &format!("Tema atual: {}", self.theme.name));
                    });

                    spacer(ui, 20.0);

                    card(ui, sz(ui.cursor.w, 150.0).p(20.0), |ui| {
                        text(ui, "📊 Debug Info");
                        spacer(ui, 10.0);
                        text_muted(ui, "Pressione F3 para toggle do debug overlay");
                    });
                }

                spacer(ui, 100.0); // Espaço extra para scroll
            });
        });
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
