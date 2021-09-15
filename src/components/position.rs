use nalgebra::Point3;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point3<f32>);
