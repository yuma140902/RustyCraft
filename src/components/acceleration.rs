use nalgebra::Vector3;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Acceleration(pub Vector3<f32>);

impl Acceleration {
    pub fn gravity() -> Acceleration {
        Acceleration(Vector3::new(0.0, -0.0001, 0.0))
    }
}
