use nalgebra::{Matrix4, Point3};

use crate::player::Angle2;

pub struct CameraComputer {}

impl CameraComputer {
    pub fn new() -> CameraComputer {
        CameraComputer {}
    }

    pub fn compute_view_matrix(&self, angle: &Angle2, pos: &Point3<f32>) -> Matrix4<f32> {
        Matrix4::<f32>::look_at_rh(&(pos), &(pos + angle.front()), &angle.up())
    }
}
