use crate::mymath::Deg;
use nalgebra::{Point3, Vector2, Vector3};
use sdl2::keyboard::Scancode;
use specs::{Component, HashMapStorage, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point3<f32>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector3<f32>);

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Angle2 {
    pitch: Deg,
    yaw: Deg,
    front: Vector3<f32>,
    right: Vector3<f32>,
    up: Vector3<f32>,
}

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Input {
    pub mouse_delta: Vector2<i32>,
    pub pressed_keys: Vec<Scancode>,
}

impl Position {
    pub fn new(point: Point3<f32>) -> Self {
        Self { 0: point }
    }
}

impl Velocity {
    pub fn new(velocity: Vector3<f32>) -> Self {
        Self { 0: velocity }
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_delta: Vector2::<i32>::new(0, 0),
            pressed_keys: Vec::new(),
        }
    }
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

    pub fn set(&mut self, pitch: Deg, yaw: Deg) {
        let (front, right, up) = Self::calc_front_right_up(pitch, yaw);
        self.pitch = pitch;
        self.yaw = yaw;
        self.front = front;
        self.right = right;
        self.up = up;
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

    pub fn pitch(&self) -> &Deg {
        &self.pitch
    }

    pub fn yaw(&self) -> &Deg {
        &self.yaw
    }

    pub fn front(&self) -> &Vector3<f32> {
        &self.front
    }

    pub fn right(&self) -> &Vector3<f32> {
        &self.right
    }

    pub fn up(&self) -> &Vector3<f32> {
        &self.up
    }
}
