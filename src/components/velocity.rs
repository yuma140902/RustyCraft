use nalgebra::Vector3;
use specs::{Component, VecStorage};

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector3<f32>);
