use nalgebra::Vector2;
use sdl2::keyboard::Scancode;
use specs::{Component, HashMapStorage};

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Input {
    pub mouse_delta: Vector2<i32>,
    pub pressed_keys: Vec<Scancode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_delta: Vector2::<i32>::new(0, 0),
            pressed_keys: Vec::new(),
        }
    }
}
