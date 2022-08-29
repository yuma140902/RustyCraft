use nalgebra::Vector2;
use specs::{Component, HashMapStorage};

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Input {
    pub mouse_delta: Vector2<i32>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_delta: Vector2::<i32>::new(0, 0),
        }
    }
}
