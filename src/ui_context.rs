use crate::config::{components, Theme};
use crate::core::{InputState, StateStore};
use crate::layout::Rect;
use crate::renderer::FontAtlas;
use ab_glyph::FontArc;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

// ============================================================================
// ANIMATION SYSTEM
// ============================================================================

#[derive(Clone)]
pub struct AnimationState {
    pub start_value: f32,
    pub target_value: f32,
    pub start_time: Instant,
    pub duration_ms: f64,
}

impl AnimationState {
    pub fn new(start: f32, target: f32, duration_ms: f64) -> Self {
        Self {
            start_value: start,
            target_value: target,
            start_time: Instant::now(),
            duration_ms,
        }
    }

    pub fn value(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f64() * 1000.0;
        if elapsed >= self.duration_ms {
            return self.target_value;
        }
        let t = (elapsed / self.duration_ms) as f32;
        // Ease-out cubic
        let eased = 1.0 - (1.0 - t).powi(3);
        self.start_value + (self.target_value - self.start_value) * eased
    }

    pub fn is_complete(&self) -> bool {
        self.start_time.elapsed().as_secs_f64() * 1000.0 >= self.duration_ms
    }
}

// ============================================================================
// SCROLL STATE
// ============================================================================

#[derive(Clone, Default)]
pub struct ScrollState {
    pub offset: f32,
    pub content_height: f32,
    pub viewport_height: f32,
    pub is_hovered: bool,
    pub is_dragging: bool,
    pub drag_start_y: f32,
    pub drag_start_offset: f32,
}

impl ScrollState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scroll(&mut self, delta: f32) {
        let max_scroll = (self.content_height - self.viewport_height).max(0.0);
        self.offset = (self.offset - delta).clamp(0.0, max_scroll);
    }

    pub fn scroll_to(&mut self, y: f32) {
        let max_scroll = (self.content_height - self.viewport_height).max(0.0);
        self.offset = y.clamp(0.0, max_scroll);
    }

    pub fn can_scroll(&self) -> bool {
        self.content_height > self.viewport_height
    }

    pub fn scrollbar_rect(&self, x: f32, y: f32) -> Option<Rect> {
        if !self.can_scroll() {
            return None;
        }
        let ratio = self.viewport_height / self.content_height;
        let thumb_height = (ratio * self.viewport_height).max(components::SCROLLBAR_MIN_HEIGHT);
        let max_offset = self.viewport_height - thumb_height;
        let thumb_y = if self.content_height > self.viewport_height {
            y + (self.offset / (self.content_height - self.viewport_height)) * max_offset
        } else {
            y
        };
        Some(Rect {
            x,
            y: thumb_y,
            w: components::SCROLLBAR_WIDTH,
            h: thumb_height,
        })
    }
}

// ============================================================================
// ANIMATED VALUE HELPER
// ============================================================================

#[derive(Clone)]
pub struct AnimatedValue {
    state: Option<AnimationState>,
    current: f32,
}

impl AnimatedValue {
    pub fn new(initial: f32) -> Self {
        Self {
            state: None,
            current: initial,
        }
    }

    pub fn set(&mut self, target: f32, duration_ms: f64) {
        self.state = Some(AnimationState::new(self.current, target, duration_ms));
    }

    pub fn update(&mut self) -> f32 {
        if let Some(anim) = &self.state {
            self.current = anim.value();
            if anim.is_complete() {
                self.state = None;
            }
        }
        self.current
    }

    pub fn is_animating(&self) -> bool {
        self.state.is_some()
    }
}

// ============================================================================
// UI CONTEXT
// ============================================================================

pub struct Ui<'a> {
    pub frame: &'a mut [u8],
    pub width: u32,
    pub height: u32,
    pub cursor: Rect,
    pub clip_rect: Option<Rect>,
    pub font: &'a FontArc,
    pub atlas: &'a mut FontAtlas,
    pub state: Rc<RefCell<StateStore>>,
    pub input: &'a InputState,
    pub scroll: ScrollState,
    pub depth: u32,
    pub widget_id_counter: u64,
    pub max_y_seen: f32,
    pub max_x_seen: f32,
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
            clip_rect: None,
            scroll: ScrollState::new(),
            depth: 0,
            widget_id_counter: 0,
            max_y_seen: 0.0,
            max_x_seen: 0.0,
        }
    }

    /// Acessa o tema atual
    pub fn theme(&self) -> std::cell::Ref<'_, Theme> {
        std::cell::Ref::map(self.state.borrow(), |s| &s.theme)
    }

    /// Gera um ID único para o widget atual
    pub fn next_widget_id(&mut self) -> u64 {
        let id = self.widget_id_counter;
        self.widget_id_counter += 1;
        id
    }

    /// Push de contexto para ID hierárquico
    pub fn push_id(&mut self, id: u64) {
        self.state.borrow_mut().push_widget(id);
    }

    /// Pop de contexto de ID
    pub fn pop_id(&mut self) {
        self.state.borrow_mut().pop_widget();
    }

    /// ID completo baseado no path hierárquico
    pub fn make_id(&self, local: u64) -> u64 {
        self.state.borrow().widget_stack.make_id(local)
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

    pub fn use_state_with_id<T: 'static + Clone>(
        &mut self,
        widget_id: u64,
        init: impl FnOnce() -> T,
    ) -> StateHandle<T> {
        let id = self.make_id(widget_id);
        let exists = {
            let store = self.state.borrow();
            store.states.contains_key(&id)
        };

        if !exists {
            let mut store = self.state.borrow_mut();
            store.states.insert(id, Box::new(init()));
        }

        StateHandle {
            id,
            store: self.state.clone(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn is_hovered(&self, rect: Rect) -> bool {
        let (mx, my) = self.input.mouse_pos;
        // Respeita clip rect se existir
        if let Some(clip) = &self.clip_rect {
            if mx < clip.x || mx > clip.x + clip.w || my < clip.y || my > clip.y + clip.h {
                return false;
            }
        }
        // Considera scroll offset
        let adjusted_y = rect.y - self.scroll.offset;
        mx >= rect.x && mx <= rect.x + rect.w && my >= adjusted_y && my <= adjusted_y + rect.h
    }

    pub fn is_hovered_absolute(&self, rect: Rect) -> bool {
        let (mx, my) = self.input.mouse_pos;
        mx >= rect.x && mx <= rect.x + rect.w && my >= rect.y && my <= rect.y + rect.h
    }

    pub fn handle_scroll(&mut self) {
        if self.scroll.is_hovered && self.input.scroll_delta != 0.0 {
            self.scroll.scroll(self.input.scroll_delta);
        }
    }

    pub fn draw_scrollbar(&mut self, x: f32, y: f32) {
        if !self.scroll.can_scroll() {
            return;
        }

                    if let Some(mut thumb_rect) = self.scroll.scrollbar_rect(x, y) {
                        let is_hovered_thumb = self.is_hovered_absolute(thumb_rect);
        
                        // Handle drag start
                        if is_hovered_thumb && self.input.mouse_just_clicked {
                            self.scroll.is_dragging = true;
                            self.scroll.drag_start_y = self.input.mouse_pos.1;
                            self.scroll.drag_start_offset = self.scroll.offset;
                        }
        
                        // Handle drag end
                        if self.scroll.is_dragging && !self.input.mouse_clicked {
                            self.scroll.is_dragging = false;
                        }
        
                        // Handle dragging movement
                        if self.scroll.is_dragging {
                            let mouse_delta_y = self.input.mouse_pos.1 - self.scroll.drag_start_y;
                            let scroll_range = self.scroll.content_height - self.scroll.viewport_height;
                            let thumb_travel_range = self.scroll.viewport_height - thumb_rect.h;
        
                            // Avoid division by zero and ensure scroll_range is positive for calculation
                            if thumb_travel_range > 0.0 && scroll_range > 0.0 {
                                let scroll_ratio = mouse_delta_y / thumb_travel_range;
                                let new_offset = self.scroll.drag_start_offset + scroll_ratio * scroll_range;
                                self.scroll.scroll_to(new_offset);
                            }
                            // Keep mouse_clicked true during drag to maintain state
                            // This is a common pattern in IMGUI for dragging, as mouse_clicked is reset each frame.
                            // However, directly setting it here might interfere with other widgets.
                            // A better approach is to not clear mouse_clicked at the start of the frame,
                            // but let the Winit event determine its state correctly.
                            // The fact that mouse_clicked is false when dragging stops is handled by the "Handle drag end" block.
                            // So, no need to set it true here.
                        }
        
                        // Update thumb_rect.y if dragging is active to reflect current position
                        // Recalculate thumb_y based on the new scroll.offset
                        let thumb_y = if self.scroll.content_height > self.scroll.viewport_height {
                            let max_thumb_y_offset = self.scroll.viewport_height - thumb_rect.h;
                            y + (self.scroll.offset / (self.scroll.content_height - self.scroll.viewport_height)) * max_thumb_y_offset
                        } else {
                            y
                        };
                        thumb_rect.y = thumb_y; // Update thumb_rect's y component
        
                        let theme = self.theme();
                        let color = if is_hovered_thumb || self.scroll.is_dragging {
                            theme.colors.text_secondary
                        } else {
                            theme.colors.text_muted
                        };
                        drop(theme);
        
                        crate::renderer::draw_rounded_rect(
                            self.frame,
                            thumb_rect.x,
                            thumb_rect.y,
                            thumb_rect.w,
                            thumb_rect.h,
                            components::SCROLLBAR_WIDTH / 2.0,
                            color.alpha(100),
                            self.width,
                            self.height,
                        );
                    }
        // Track da scrollbar
        let store = self.state.borrow();
        let border_color = store.theme.colors.border.alpha(50); // Extract color before store is dropped
        drop(store); // Explicitly drop the immutable borrow

        crate::renderer::draw_rect(
            self.frame,
            x as i32,
            y as i32,
            components::SCROLLBAR_WIDTH as i32,
            self.scroll.viewport_height as i32,
            border_color,
            self.width,
            self.height,
        );
    }
}
