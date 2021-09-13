use nalgebra::Matrix4;

use crate::components::{Angle2, Position};

pub struct CameraComputer {}

impl CameraComputer {
    pub fn new() -> CameraComputer {
        CameraComputer {}
    }

    const PLAYER_EYE_DIFF: nalgebra::Vector3<f32> = nalgebra::Vector3::new(0.0, 0.3, 0.0);

    pub fn compute_view_matrix(&self, angle: &Angle2, pos: &Position) -> Matrix4<f32> {
        Matrix4::<f32>::look_at_rh(
            &(pos.0 + CameraComputer::PLAYER_EYE_DIFF),
            &(pos.0 + angle.front()),
            &angle.up(),
        )
    }
}
