# RustUI - Zen Framework 🚀

RustUI é um framework de interface de usuário ultra-rápido, de baixo nível e com sintaxe **Zen Mode**, projetado para ser mais conciso que o Jetpack Compose e o Flutter, rodando inteiramente sobre um motor proprietário de pixels.

## 📦 Dependências
Adicione isto ao seu `Cargo.toml`:

```toml
[dependencies]
winit = "0.28"
pixels = "0.13"
glam = "0.29"
log = "0.4"
env_logger = "0.11"
ab_glyph = "0.2"
num-format = "0.4.4" # Para formatação de números com separadores de milhares
rustui = { path = "D:\\Dev\\Porjetos\\rustui" } # Se estiver usando localmente
```

## ✨ Funcionalidades Premium

### 1. Smart Modifiers
Em vez de `Modifier::new().padding(10).background(...)`, use atalhos:
- `pad(10.)`: Adiciona espaçamento interno.
- `bg(COLOR)`: Define a cor de fundo.
- `sz(w, h)`: Define o tamanho fixo.
- **Chaining**: Você pode encadear: `sz(100., 50.).bg(Color::RED).pad(5.)`.

### 2. Rendering SDF (Smooth Graphics)
- Bordas arredondadas perfeitamente lisas (Anti-aliasing).
- Sombras projetadas (Drop shadows) para profundidade.
- Gradientes lineares nativos.

### 3. Texto Real
- Sistema de fonte bitmap integrado. Fim de retângulos brancos como placeholders.

## 📐 Layout Engine
O framework utiliza um sistema de **Z-Index implícito** e **Auto-advance**. Se você colocar dois `text()` dentro de um `column()`, o segundo aparecerá automaticamente abaixo do primeiro com o espaçamento correto.

---

## 🚀 Começando com RustUI (Exemplo Completo)

Este é um exemplo completo de um arquivo `main.rs` que mostra como configurar e usar o RustUI para criar uma interface de usuário simples com estado reativo.

```rust
use ab_glyph::FontArc;
use rustui::config::{Theme};
use rustui::core::{App, InputState, StateStore, run};
use rustui::renderer::{FontAtlas, clear};
use rustui::widgets::{
    button, column, spacer, text, text_heading, text_muted,
};
use rustui::{bg, pad, sz, Ui};
use num_format::{Locale, ToFormattedString};
use std::cell::RefCell;
use std::rc::Rc;

// A estrutura principal da sua aplicação.
// Ela contém o estado global ou configurações que sua UI precisa.
struct MyExampleApp {
    theme: Theme,
}

// Implementa o trait `App` para sua aplicação.
// `update` é para lógica de negócios/estado que não é UI.
// `draw` é onde toda a sua UI é definida para cada frame.
impl App for MyExampleApp {
    fn update(&mut self, _input: &InputState) {
        // Lógica de atualização da aplicação aqui (ex: mover personagens em um jogo)
    }

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
        // Limpa o frame com a cor de fundo do tema.
        clear(frame, self.theme.colors.background);

        // Inicializa o contexto da UI para o frame atual.
        let mut ui = Ui::new(frame, w, h, font, atlas, state, input);

        // --- Exemplo de Estado Reativo (Contador de Cliques) ---
        // `use_state` cria uma variável de estado que persiste entre frames
        // e dispara uma re-renderização quando seu valor é alterado.
        let click_count = ui.use_state(|| 0);

        // Layout principal em coluna, centralizado na tela
        column(&mut ui, pad(50.0).bg(self.theme.colors.surface), |ui| {
            text_heading(ui, "Bem-vindo ao RustUI!");
            text_muted(ui, "Um framework UI zen e performático.");
            spacer(ui, 30.0);

            // Exibe o contador de cliques
            text(ui, &format!("Cliques: {}", click_count.get().to_formatted_string(&Locale::en)));
            spacer(ui, 20.0);

            // Botão que incrementa o contador
            if button(ui, bg(self.theme.colors.primary).s(200.0, 45.0), "Clique-me!") {
                click_count.set(click_count.get() + 1);
            }
            spacer(ui, 10.0);

            // Botão para resetar o contador
            if button(ui, bg(self.theme.colors.error).s(200.0, 45.0), "Resetar") {
                click_count.set(0);
            }
        });
    }
}

fn main() {
    // Inicializa o logger (útil para depuração)
    env_logger::init();

    // Carrega a fonte padrão (Arial como fallback)
    let font_data = rustui::get_font_with_fallback(Some("Arial"))
        .expect("Falha ao carregar fonte. Instale uma fonte TrueType como Arial.");
    let font = FontArc::try_from_vec(font_data).unwrap();

    // Cria uma instância da sua aplicação
    let app = MyExampleApp {
        theme: Theme::dark(), // Define o tema inicial como escuro
    };

    // Inicia o loop principal da aplicação RustUI
    // Define a janela com 800x600 pixels.
    run(app, 800, 600, font);
}
```