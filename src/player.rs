use sdl2::keyboard::{KeyboardState, Scancode};
use sdl2::mouse::MouseState;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, TimerSubsystem};

use cgmath::Deg;
use cgmath::Zero;
use cgmath::{Angle, InnerSpace};

use crate::game_config;

#[allow(unused)]
type Point3 = cgmath::Point3<f32>;
#[allow(unused)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(unused)]
type Matrix4 = cgmath::Matrix4<f32>;

pub struct Player {
    position: Point3,
    pitch: Deg<f32>,
    yaw: Deg<f32>,
    front: Vector3,
    right: Vector3,
    up: Vector3,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Point3 {
                x: 2.0,
                y: 0.5,
                z: 2.0,
            },
            pitch: Deg(90.0f32),
            yaw: Deg(90.0f32),
            front: Vector3::zero(),
            right: Vector3::zero(),
            up: Vector3::zero(),
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

    pub fn front(&self) -> Vector3 {
        self.front
    }

    pub fn right(&self) -> Vector3 {
        self.right
    }

    pub fn up(&self) -> Vector3 {
        self.up
    }
}

pub struct PlayerController {
    last_tick: u32,
}

impl PlayerController {
    pub fn new(time: &TimerSubsystem) -> PlayerController {
        PlayerController {
            last_tick: time.ticks(),
        }
    }

    pub fn update_player(
        &mut self,
        player: &mut Player,
        sdl: &Sdl,
        window: &Window,
        e: &EventPump,
        time: &TimerSubsystem,
    ) {
        /* 参考:
        チュートリアル６：キーボードとマウス | http://www.opengl-tutorial.org/jp/beginners-tutorials/tutorial-6-keyboard-and-mouse
        ogl/controls.cpp at master · opengl-tutorials/ogl | https://github.com/opengl-tutorials/ogl/blob/master/common/controls.cpp
        */

        let current_tick = time.ticks();
        let delta_tick: f32 = (current_tick - self.last_tick) as f32;
        self.last_tick = current_tick;

        let mouse = MouseState::new(e);

        let (width, height) = window.drawable_size();
        let center_x: i32 = width as i32 / 2;
        let center_y: i32 = height as i32 / 2;

        player.pitch += Deg(game_config::ROTATE_SPEED * delta_tick * (center_x - mouse.x()) as f32);
        player.yaw += Deg(game_config::ROTATE_SPEED * delta_tick * (center_y - mouse.y()) as f32);

        if player.yaw < Deg(-90f32) {
            player.yaw = Deg(-90f32);
        }
        if player.yaw > Deg(90f32) {
            player.yaw = Deg(90f32);
        }
        if player.pitch < Deg(0f32) {
            player.pitch += Deg(360f32);
        }
        if player.pitch > Deg(360f32) {
            player.pitch -= Deg(360f32);
        }

        // カメラの前方向のベクトル
        player.front = Vector3 {
            x: player.yaw.cos() * player.pitch.sin(),
            y: player.yaw.sin(),
            z: player.yaw.cos() * player.pitch.cos(),
        }
        .normalize();

        let right: Deg<f32> = player.pitch - Deg(90.0f32);
        // カメラの右方向のベクトル
        player.right = Vector3 {
            x: right.sin(),
            y: 0.0f32, // ロールは0なので常に床と水平
            z: right.cos(),
        }
        .normalize();

        // カメラの上方向のベクトル
        player.up = player.right.cross(player.front);

        let keyboard = KeyboardState::new(e);
        let front_on_ground = Vector3 {
            x: player.front.x,
            y: 0.0,
            z: player.front.z,
        }
        .normalize();
        let up_on_ground = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        if keyboard.is_scancode_pressed(Scancode::W) {
            player.position += front_on_ground * game_config::MOVE_SPEED * delta_tick;
        }
        if keyboard.is_scancode_pressed(Scancode::S) {
            player.position -= front_on_ground * game_config::MOVE_SPEED * delta_tick;
        }
        if keyboard.is_scancode_pressed(Scancode::D) {
            player.position += player.right * game_config::MOVE_SPEED * delta_tick;
        }
        if keyboard.is_scancode_pressed(Scancode::A) {
            player.position -= player.right * game_config::MOVE_SPEED * delta_tick;
        }
        if keyboard.is_scancode_pressed(Scancode::Space) {
            player.position += up_on_ground * game_config::MOVE_SPEED * delta_tick;
        }
        if keyboard.is_scancode_pressed(Scancode::LShift) {
            player.position -= up_on_ground * game_config::MOVE_SPEED * delta_tick;
        }

        // マウスを中心に戻す
        sdl.mouse().warp_mouse_in_window(window, center_x, center_y);
    }
}
