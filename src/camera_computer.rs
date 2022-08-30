use nalgebra::{Matrix4, Point3};

use crate::player::Angle2;

pub struct CameraComputer {}

impl CameraComputer {
    pub fn new() -> CameraComputer {
        CameraComputer {}
    }

    const PLAYER_EYE_DIFF: nalgebra::Vector3<f32> = nalgebra::Vector3::new(0.0, 0.3, 0.0);

    pub fn compute_view_matrix(&self, angle: &Angle2, pos: &Point3<f32>) -> Matrix4<f32> {
        Matrix4::<f32>::look_at_rh(
            &(pos + CameraComputer::PLAYER_EYE_DIFF),
            &(pos + angle.front()),
            &angle.up(),
        )
    }
}
