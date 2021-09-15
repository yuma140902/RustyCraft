use nalgebra::Vector3;
use specs::{Component, VecStorage};

use crate::ecs_resources::DeltaTick;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Force {
    pub vec: Vector3<f32>,
    pub ticks: DeltaTick,
}
