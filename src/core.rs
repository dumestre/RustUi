use crate::renderer::FontAtlas;
use ab_glyph::FontArc;
use pixels::{Pixels, SurfaceTexture};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Clone, Copy)]
pub struct InputState {
    pub mouse_pos: (f32, f32),
    pub mouse_clicked: bool,
}

pub struct StateStore {
    pub states: HashMap<u64, Box<dyn Any>>,
    pub current_index: u64,
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            current_index: 0,
        }
    }
    pub fn reset_index(&mut self) {
        self.current_index = 0;
    }
}

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

    let mut input = InputState {
        mouse_pos: (0.0, 0.0),
        mouse_clicked: false,
    };
    let mut atlas = FontAtlas::new();
    let state_store = Rc::new(RefCell::new(StateStore::new()));

    event_loop.run(move |event, _, control_flow| {
        // OTIMIZAÇÃO: Redraw on demand (Wait) em vez de Poll (100% CPU)
        *control_flow = ControlFlow::Wait;

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
                window.request_redraw(); // Redesenha ao mover o mouse (interatividade)
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
                input.mouse_clicked = state == ElementState::Pressed;
                window.request_redraw(); // Redesenha ao clicar
            }
            Event::RedrawRequested(_) => {
                state_store.borrow_mut().reset_index();
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
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}
