use std::collections::HashMap;
use glium::{Surface};
use glium::glutin::{ElementState, MouseButton};

// Mouse frame history size (assumed to always be 2):
const FRAME_HISTORY: usize = 2;

// [FIXME]: TODO: Consider changing 'is_stale' to 'is_fresh'. Currently being used as such.
pub struct MouseState {
    position: [(i32, i32); FRAME_HISTORY],
    left: ElementState,
    right: ElementState,
    middle: ElementState,
    other: HashMap<u8, ElementState>,
    frame: u8,
    is_stale: bool,
}

impl MouseState {
    pub fn new() -> MouseState {
        MouseState { 
            position: [(0, 0); FRAME_HISTORY as usize], 
            frame: 0,
            left: ElementState::Released,
            right: ElementState::Released,
            middle: ElementState::Released,
            other: HashMap::new(),
            // is_depressed: false,
            is_stale: false,
        }
    }

    pub fn position(&self) -> (i32, i32) {
        debug_assert!((self.frame as usize) < FRAME_HISTORY);
        self.position[self.frame as usize]
    }

    pub fn surface_position<S: Surface>(&self, target: &mut S) -> (f32, f32) {
        let (sw, sh) = target.get_dimensions();
        let (px, py) = self.position();

        (to_gl_dim(px, sw), -to_gl_dim(py, sh))
    }

    pub fn update_position(&mut self, new_pos: (i32, i32)) {
        self.frame = self.frame ^ 1;
        self.position[self.frame as usize] = new_pos;
        self.is_stale = false;
        // println!("                             {:?}", self.position[self.frame as usize]);
    }

    pub fn update_button(&mut self, button: MouseButton, state: ElementState) {
        match button {
            MouseButton::Left => self.left = state,            
            MouseButton::Right => self.right = state,
            MouseButton::Middle => self.middle = state,
            MouseButton::Other(b) => { self.other.insert(b, state); },
        }
    }

    pub fn set_stale(&mut self) {
        self.is_stale = true;
    }

    pub fn is_stale(&self) -> bool {
        self.is_stale
    }
}

// Convert position in pixels to OpenGL screen position [-1.0..1.0]:
fn to_gl_dim(p: i32, s: u32) -> f32 {
    ((p as f32 / s as f32) * 2.0) - 1.0
}