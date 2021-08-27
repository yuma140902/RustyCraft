use nalgebra::Matrix4;

use crate::components::{Angle2, Position};

pub struct CameraComputer {}

impl CameraComputer {
    pub fn new() -> CameraComputer {
        CameraComputer {}
    }

    pub fn compute_view_matrix(&self, angle: &Angle2, pos: &Position) -> Matrix4<f32> {
        Matrix4::<f32>::look_at_rh(&pos.0, &(pos.0 + angle.front()), &angle.up())
    }
}
