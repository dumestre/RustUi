use crate::config::Theme;
use crate::renderer::FontAtlas;
use ab_glyph::FontArc;
use pixels::{Pixels, SurfaceTexture};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;
use winit::{
    event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// ============================================================================
// INPUT STATE EXPANDIDO
// ============================================================================

#[derive(Clone, Copy)]
pub struct InputState {
    pub mouse_pos: (f32, f32),
    pub mouse_clicked: bool,
    pub mouse_just_clicked: bool,
    pub keys_pressed: [bool; 256],
    pub keys_just_pressed: [bool; 256],
    pub char_input: Option<char>,
    pub scroll_delta: f32,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            mouse_pos: (0.0, 0.0),
            mouse_clicked: false,
            mouse_just_clicked: false,
            keys_pressed: [false; 256],
            keys_just_pressed: [false; 256],
            char_input: None,
            scroll_delta: 0.0,
        }
    }
}

impl InputState {
    pub fn key_down(&self, key: VirtualKeyCode) -> bool {
        self.keys_pressed.get(key as usize).copied().unwrap_or(false)
    }

    pub fn key_just_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys_just_pressed.get(key as usize).copied().unwrap_or(false)
    }

    pub fn ctrl(&self) -> bool {
        self.key_down(VirtualKeyCode::LControl) || self.key_down(VirtualKeyCode::RControl)
    }

    pub fn shift(&self) -> bool {
        self.key_down(VirtualKeyCode::LShift) || self.key_down(VirtualKeyCode::RShift)
    }

    pub fn alt(&self) -> bool {
        self.key_down(VirtualKeyCode::LAlt) || self.key_down(VirtualKeyCode::RAlt)
    }
}

// ============================================================================
// WIDGET ID SYSTEM (para hooks estáveis)
// ============================================================================

/// Stack de IDs para widgets - permite IDs únicos baseados em path
#[derive(Default)]
pub struct WidgetIdStack {
    stack: Vec<u64>,
    current: u64,
}

impl WidgetIdStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(32),
            current: 0,
        }
    }

    pub fn push(&mut self, id: u64) {
        self.stack.push(id);
        self.current = self.current.wrapping_mul(31).wrapping_add(id);
    }

    pub fn pop(&mut self) -> Option<u64> {
        let id = self.stack.pop();
        self.current = self.stack.iter().fold(0u64, |acc, &x| acc.wrapping_mul(31).wrapping_add(x));
        id
    }

    pub fn current_id(&self) -> u64 {
        self.current
    }

    pub fn make_id(&self, local_id: u64) -> u64 {
        self.current.wrapping_mul(31).wrapping_add(local_id)
    }

    pub fn reset(&mut self) {
        self.stack.clear();
        self.current = 0;
    }
}

// ============================================================================
// STATE STORE COM SUPORTE A IDS ESTÁVEIS
// ============================================================================

pub struct StateStore {
    pub states: HashMap<u64, Box<dyn Any>>,
    pub widget_stack: WidgetIdStack,
    pub theme: Theme,
    pub current_index: u64,
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            widget_stack: WidgetIdStack::new(),
            theme: Theme::default(),
            current_index: 0,
        }
    }

    pub fn get_widget_id(&mut self, local_id: u64) -> u64 {
        self.widget_stack.make_id(local_id)
    }

    pub fn push_widget(&mut self, id: u64) {
        self.widget_stack.push(id);
    }

    pub fn pop_widget(&mut self) {
        self.widget_stack.pop();
    }

    pub fn reset_frame(&mut self) {
        self.widget_stack.reset();
        self.current_index = 0;
    }
}

// ============================================================================
// APP TRAIT EXPANDIDA
// ============================================================================

pub trait App {
    fn update(&mut self, input: &InputState);
    fn draw(
        &self,
        frame: &mut [u8],
        width: u32,
        height: u32,
        font: &FontArc,
        atlas: &mut FontAtlas,
        state: Rc<RefCell<StateStore>>,
        input: &InputState,
    );
}

// ============================================================================
// DEBUG INFO
// ============================================================================

#[derive(Default)]
pub struct DebugInfo {
    pub fps: f64,
    pub frame_time_ms: f64,
    pub draw_calls: u32,
    pub widget_count: u32,
    pub show_overlay: bool,
}

// ============================================================================
// MAIN LOOP
// ============================================================================

pub fn run(mut app: impl App + 'static, width: u32, height: u32, font: FontArc) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("RustUI - Reactive Pro ⚡")
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width, height, surface_texture).unwrap()
    };

    let mut input = InputState::default();
    let mut atlas = FontAtlas::new();
    let state_store = Rc::new(RefCell::new(StateStore::new()));

    // Debug timing
    let mut _last_frame = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();
    let mut debug_info = DebugInfo::default();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Reset just-pressed states
        input.mouse_just_clicked = false;
        input.keys_just_pressed.fill(false);
        input.char_input = None;
        input.scroll_delta = 0.0;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                pixels.resize_surface(size.width, size.height).unwrap();
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                input.mouse_pos = (position.x as f32, position.y as f32);
                window.request_redraw();
            }

            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state,
                        button: MouseButton::Left,
                        ..
                    },
                ..
            } => {
                if state == ElementState::Pressed {
                    input.mouse_clicked = true;
                    input.mouse_just_clicked = true;
                } else {
                    input.mouse_clicked = false;
                }
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input: key_input, .. },
                ..
            } => {
                if let Some(virtual_keycode) = key_input.virtual_keycode {
                    let idx = virtual_keycode as usize;
                    if idx < 256 {
                        if key_input.state == ElementState::Pressed {
                            if !input.keys_pressed[idx] {
                                input.keys_just_pressed[idx] = true;
                            }
                            input.keys_pressed[idx] = true;

                            // Toggle debug overlay com F3
                            if virtual_keycode == VirtualKeyCode::F3 {
                                debug_info.show_overlay = !debug_info.show_overlay;
                            }
                        } else {
                            input.keys_pressed[idx] = false;
                        }
                    }
                }
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter(ch),
                ..
            } => {
                input.char_input = Some(ch);
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => {
                        input.scroll_delta = y * 20.0;
                    }
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        input.scroll_delta = pos.y as f32;
                    }
                }
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                let frame_start = Instant::now();

                // Reset widget stack para novos IDs
                state_store.borrow_mut().reset_frame();

                app.update(&input);
                app.draw(
                    pixels.frame_mut(),
                    width,
                    height,
                    &font,
                    &mut atlas,
                    state_store.clone(),
                    &input,
                );

                if let Err(err) = pixels.render() {
                    log::error!("Erro ao renderizar frame: {:?}", err);
                }

                // Debug timing
                let frame_time = frame_start.elapsed();
                frame_count += 1;

                if fps_timer.elapsed().as_secs_f64() >= 1.0 {
                    debug_info.fps = frame_count as f64 / fps_timer.elapsed().as_secs_f64();
                    debug_info.frame_time_ms = frame_time.as_secs_f64() * 1000.0;
                    frame_count = 0;
                    fps_timer = Instant::now();
                }

                _last_frame = frame_start;
            }

            _ => (),
        }
    });
}
