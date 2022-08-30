use nalgebra::{Point3, Vector3};
use parry3d::shape::Cuboid;

use crate::mymath::Deg;

pub struct Player {
    pub pos: Point3<f32>,
    pub angle: Angle2,
    pub collider: Cuboid,
    pub on_ground: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Point3::new(4.0, 2.5, 4.0),
            angle: Angle2::new(Deg(225.0_f32), Deg(0.0_f32)),
            collider: Cuboid::new(Vector3::new(0.15, 0.45, 0.15)),
            on_ground: false,
        }
    }
}

#[derive(Debug)]
pub struct Angle2 {
    pitch: Deg,
    yaw: Deg,
    front: Vector3<f32>,
    right: Vector3<f32>,
    up: Vector3<f32>,
}

impl Angle2 {
    pub fn new(pitch: Deg, yaw: Deg) -> Self {
        let (front, right, up) = Self::calc_front_right_up(pitch, yaw);
        Self {
            pitch,
            yaw,
            front,
            right,
            up,
        }
    }

    fn calc_front_right_up(pitch: Deg, yaw: Deg) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
        let front =
            Vector3::new(yaw.cos() * pitch.sin(), yaw.sin(), yaw.cos() * pitch.cos()).normalize();

        let right: Deg = Deg(*pitch - 90.0f32);
        // 右方向のベクトル
        let right = Vector3::new(
            right.sin(),
            0.0f32, /* ロールは0なので常に床と水平 */
            right.cos(),
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
