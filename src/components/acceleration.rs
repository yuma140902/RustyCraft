use nalgebra::Vector3;
use specs::{Component, VecStorage};

use crate::game_config;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Acceleration(pub Vector3<f32>);

impl Acceleration {
    pub fn gravity() -> Acceleration {
        Acceleration(Vector3::new(0.0, -game_config::GRAVITY, 0.0))
    }
}
