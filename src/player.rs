use nalgebra::{Point3, Vector3};

use crate::mymath::deg_to_rad;

pub struct Player {
    pub pos: Point3<f32>,
    pub angle: Angle2,
    pub on_ground: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Point3::new(4.0, 3.6, 4.0),
            angle: Angle2::new(225.0_f32, -30.0_f32),
            on_ground: false,
        }
    }
}

#[derive(Debug)]
pub struct Angle2 {
    front: Vector3<f32>,
    up: Vector3<f32>,
}

impl Angle2 {
    pub fn new(pitch_deg: f32, yaw_deg: f32) -> Self {
        let (front, _right, up) = Self::calc_front_right_up(pitch_deg, yaw_deg);
        Self { front, up }
    }

    fn calc_front_right_up(
        pitch_deg: f32,
        yaw_deg: f32,
    ) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
        let pitch_rad = deg_to_rad(pitch_deg);
        let yaw_rad = deg_to_rad(yaw_deg);
        let front = Vector3::new(
            yaw_rad.cos() * pitch_rad.sin(),
            yaw_rad.sin(),
            yaw_rad.cos() * pitch_rad.cos(),
        )
        .normalize();

        let right_rad = deg_to_rad(pitch_deg - 90.0f32);
        // 右方向のベクトル
        let right = Vector3::new(
            right_rad.sin(),
            0.0f32, /* ロールは0なので常に床と水平 */
            right_rad.cos(),
        )
        .normalize();

        // 上方向のベクトル
        let up = right.cross(&front);

        (front, right, up)
    }

    pub fn front(&self) -> &Vector3<f32> {
        &self.front
    }

    pub fn up(&self) -> &Vector3<f32> {
        &self.up
    }
}
