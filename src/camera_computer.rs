use crate::player::Player;

#[allow(unused)]
type Point3 = cgmath::Point3<f32>;
#[allow(unused)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(unused)]
type Matrix4 = cgmath::Matrix4<f32>;

pub struct CameraComputer {}

impl CameraComputer {
    pub fn new() -> CameraComputer {
        CameraComputer {}
    }

    pub fn compute_view_matrix(&self, player: &Player) -> Matrix4 {
        Matrix4::look_at_rh(
            player.position(),
            player.position() + player.front(),
            player.up(),
        )
    }
}
