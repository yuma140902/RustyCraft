use sdl2::keyboard::{KeyboardState, Scancode};
use sdl2::mouse::MouseState;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

use cgmath::Angle;
use cgmath::Deg;
use cgmath::Zero;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

pub struct CameraComputer {
    position: Point3,
    pitch: Deg<f32>,
    yaw: Deg<f32>,
    rotate_speed: f32,
    move_speed: f32,
    front: Vector3,
    right: Vector3,
    up: Vector3,
    updated: bool,
}

impl CameraComputer {
    pub fn new() -> CameraComputer {
        CameraComputer {
            position: Point3 {
                x: 2.0,
                y: 0.5,
                z: 2.0,
            },
            pitch: Deg(90.0f32),
            yaw: Deg(90.0f32),
            rotate_speed: 0.6f32,
            move_speed: 0.1f32,
            front: Vector3::zero(),
            right: Vector3::zero(),
            up: Vector3::zero(),
            updated: false,
        }
    }

    pub fn pitch(&self) -> Deg<f32> {
        self.pitch
    }

    pub fn yaw(&self) -> Deg<f32> {
        self.yaw
    }

    pub fn position(&self) -> Point3 {
        self.position
    }

    pub fn update(&mut self, sdl: &Sdl, window: &Window, e: &EventPump) {
        /* 参考:
        チュートリアル６：キーボードとマウス | http://www.opengl-tutorial.org/jp/beginners-tutorials/tutorial-6-keyboard-and-mouse
        ogl/controls.cpp at master · opengl-tutorials/ogl | https://github.com/opengl-tutorials/ogl/blob/master/common/controls.cpp
        */

        let mouse = MouseState::new(e);

        let (width, height) = window.drawable_size();
        let center_x: i32 = width as i32 / 2;
        let center_y: i32 = height as i32 / 2;

        self.pitch += Deg(self.rotate_speed * (center_x - mouse.x()) as f32);
        self.yaw += Deg(self.rotate_speed * (center_y - mouse.y()) as f32);

        if self.yaw < Deg(-90f32) {
            self.yaw = Deg(-90f32);
        }
        if self.yaw > Deg(90f32) {
            self.yaw = Deg(90f32);
        }
        if self.pitch < Deg(0f32) {
            self.pitch += Deg(360f32);
        }
        if self.pitch > Deg(360f32) {
            self.pitch -= Deg(360f32);
        }

        // カメラの前方向のベクトル
        self.front = Vector3 {
            x: self.yaw.cos() * self.pitch.sin(),
            y: self.yaw.sin(),
            z: self.yaw.cos() * self.pitch.cos(),
        };

        let right: Deg<f32> = self.pitch - Deg(90.0f32);
        // カメラの右方向のベクトル
        self.right = Vector3 {
            x: right.sin(),
            y: 0.0f32, // ロールは0なので常に床と水平
            z: right.cos(),
        };

        // カメラの上方向のベクトル
        self.up = self.right.cross(self.front);

        let keyboard = KeyboardState::new(e);
        let front_on_ground = Vector3 {
            x: self.front.x,
            y: 0.0,
            z: self.front.z,
        };
        let up_on_ground = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        if keyboard.is_scancode_pressed(Scancode::W) {
            self.position += front_on_ground * self.move_speed;
        }
        if keyboard.is_scancode_pressed(Scancode::S) {
            self.position -= front_on_ground * self.move_speed;
        }
        if keyboard.is_scancode_pressed(Scancode::D) {
            self.position += self.right * self.move_speed;
        }
        if keyboard.is_scancode_pressed(Scancode::A) {
            self.position -= self.right * self.move_speed;
        }
        if keyboard.is_scancode_pressed(Scancode::Space) {
            self.position += up_on_ground * self.move_speed;
        }
        if keyboard.is_scancode_pressed(Scancode::LShift) {
            self.position -= up_on_ground * self.move_speed;
        }

        // マウスを中心に戻す
        sdl.mouse().warp_mouse_in_window(window, center_x, center_y);
        self.updated = true;
    }

    pub fn compute_view_matrix(&self) -> Matrix4 {
        if !self.updated {
            panic!("Update first");
        }
        Matrix4::look_at_rh(self.position, self.position + self.front, self.up)
    }
}
