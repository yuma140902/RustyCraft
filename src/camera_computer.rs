use nalgebra::Matrix4;

use crate::player::{calc_front_right_up, Player};

pub struct CameraComputer {}

impl CameraComputer {
    pub fn new() -> CameraComputer {
        CameraComputer {}
    }

    pub fn compute_view_matrix(&self, player: &Player) -> Matrix4<f32> {
        let (front, _right, up) = calc_front_right_up(player.pitch_rad, player.yaw_rad);
        Matrix4::<f32>::look_at_rh(&player.pos, &(player.pos + front), &up)
    }
}
