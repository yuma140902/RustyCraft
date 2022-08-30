use nalgebra::{Point3, Vector3};

use crate::mymath::deg_to_rad;

pub struct Player {
    pub pos: Point3<f32>,
    pub pitch_rad: f32,
    pub yaw_rad: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Point3::new(4.0, 3.6, 4.0),
            pitch_rad: deg_to_rad(225.0),
            yaw_rad: deg_to_rad(-30.0),
        }
    }
}

pub fn calc_front_right_up(
    pitch_rad: f32,
    yaw_rad: f32,
) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    let front = Vector3::new(
        yaw_rad.cos() * pitch_rad.sin(),
        yaw_rad.sin(),
        yaw_rad.cos() * pitch_rad.cos(),
    )
    .normalize();

    let right_rad = pitch_rad - deg_to_rad(90.0f32);
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
