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
rustui = { path = "../rustui" } # Se estiver usando localmente
```

## 🛠️ Como Usar (Sintaxe Zen)

A principal vantagem do RustUI é a eliminação de boilerplate. O layout avança automaticamente e os modificadores têm atalhos globais.

### Exemplo Minimalista

```rust
use rustui::core::{App, run, InputState};
use rustui::renderer::{clear, Color};
use rustui::widgets::{column, text, button};
use rustui::{pad, bg, sz, Ui};

struct MyApp;

impl App for MyApp {
    fn update(&mut self, _input: &InputState) {}

    fn draw(&self, frame: &mut [u8], w: u32, h: u32) {
        clear(frame, Color { r: 15, g: 15, b: 25, a: 255 });
        let mut ui = Ui::new(frame, w, h);

        // Layout Automático: O cursor se move sozinho!
        column(&mut ui, pad(30.), |ui| {
            text(ui, "BEM-VINDO AO RUSTUI");
            
            button(ui, sz(150., 40.).bg(Color::GREEN), "CLIQUE AQUI");
        });
    }
}

fn main() {
    run(MyApp, 800, 600);
}
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
*RustUI: Menos código para escrever, mais performance para rodar.*
