use nalgebra::Vector3;
use specs::{Component, VecStorage};

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector3<f32>);

impl Velocity {
    pub fn new(velocity: Vector3<f32>) -> Self {
        Self { 0: velocity }
    }
}
