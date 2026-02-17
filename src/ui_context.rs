use crate::core::{InputState, StateStore};
use crate::layout::Rect;
use crate::renderer::FontAtlas;
use ab_glyph::FontArc;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Ui<'a> {
    pub frame: &'a mut [u8],
    pub width: u32,
    pub height: u32,
    pub cursor: Rect,
    pub font: &'a FontArc,
    pub atlas: &'a mut FontAtlas,
    pub state: Rc<RefCell<StateStore>>,
    pub input: &'a InputState,
}

pub struct StateHandle<T> {
    id: u64,
    store: Rc<RefCell<StateStore>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: 'static + Clone> StateHandle<T> {
    pub fn get(&self) -> T {
        let store = self.store.borrow();
        store
            .states
            .get(&self.id)
            .and_then(|any| any.downcast_ref::<T>())
            .cloned()
            .expect("Estado não encontrado ou tipo inválido!")
    }

    pub fn set(&self, val: T) {
        let mut store = self.store.borrow_mut();
        store.states.insert(self.id, Box::new(val));
    }

    pub fn update(&self, f: impl FnOnce(T) -> T) {
        let current = self.get();
        self.set(f(current));
    }
}

impl<'a> Ui<'a> {
    pub fn new(
        frame: &'a mut [u8],
        width: u32,
        height: u32,
        font: &'a FontArc,
        atlas: &'a mut FontAtlas,
        state: Rc<RefCell<StateStore>>,
        input: &'a InputState,
    ) -> Self {
        Self {
            frame,
            width,
            height,
            font,
            atlas,
            state,
            input,
            cursor: Rect {
                x: 0.0,
                y: 0.0,
                w: width as f32,
                h: height as f32,
            },
        }
    }

    pub fn use_state<T: 'static + Clone>(&mut self, init: impl FnOnce() -> T) -> StateHandle<T> {
        let id = {
            let mut store = self.state.borrow_mut();
            let id = store.current_index;
            store.current_index += 1;
            if !store.states.contains_key(&id) {
                store.states.insert(id, Box::new(init()));
            }
            id
        };

        StateHandle {
            id,
            store: self.state.clone(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn is_hovered(&self, rect: Rect) -> bool {
        let (mx, my) = self.input.mouse_pos;
        mx >= rect.x && mx <= rect.x + rect.w && my >= rect.y && my <= rect.y + rect.h
    }
}
