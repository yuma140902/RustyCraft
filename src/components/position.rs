use nalgebra::Point3;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point3<f32>);

impl Position {
    pub fn new(point: Point3<f32>) -> Self {
        Self { 0: point }
    }
}
