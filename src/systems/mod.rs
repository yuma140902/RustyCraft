use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::game_config;

use super::components::*;
use super::ecs_resources::*;

pub struct PositionUpdater;

impl<'a> System<'a> for PositionUpdater {
    type SystemData = (
        Read<'a, DeltaTick>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (delta, vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.0 += vel.0 * delta.0 as f32;
        }
    }
}

pub struct AngleController;
impl<'a> System<'a> for AngleController {
    type SystemData = (
        Read<'a, DeltaTick>,
        ReadStorage<'a, Input>,
        WriteStorage<'a, Angle2>,
    );

    fn run(&mut self, (delta, input, mut angle): Self::SystemData) {
        use crate::mymath::Deg;

        for (input, angle) in (&input, &mut angle).join() {
            let mut pitch = angle.pitch().clone();
            let mut yaw = angle.yaw().clone();

            *pitch += *Deg(game_config::ROTATE_SPEED * delta.0 as f32 * input.mouse_delta.x as f32);
            *yaw += *Deg(game_config::ROTATE_SPEED * delta.0 as f32 * input.mouse_delta.y as f32);

            if yaw < Deg(-90f32) {
                yaw = Deg(-90f32);
            }
            if yaw > Deg(90f32) {
                yaw = Deg(90f32);
            }
            if pitch < Deg(0f32) {
                *pitch += *Deg(360f32);
            }
            if pitch > Deg(360f32) {
                *pitch -= *Deg(360f32);
            }

            angle.set(pitch, yaw);
        }
    }
}

pub struct VelocityController;

impl<'a> System<'a> for VelocityController {
    type SystemData = (
        ReadStorage<'a, Input>,
        ReadStorage<'a, Angle2>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (input, angle, mut vel): Self::SystemData) {
        use nalgebra::Vector3;
        use sdl2::keyboard::Scancode;

        for (input, angle, vel) in (&input, &angle, &mut vel).join() {
            let front_on_ground =
                Vector3::<f32>::new(angle.front().x, 0.0, angle.front().z).normalize();
            let up_on_ground = Vector3::<f32>::new(0.0, 1.0, 0.0);

            let mut velocity = Vector3::<f32>::new(0.0, 0.0, 0.0);

            if input.pressed_keys.contains(&Scancode::W) {
                velocity += front_on_ground * game_config::MOVE_SPEED;
            }
            if input.pressed_keys.contains(&Scancode::S) {
                velocity -= front_on_ground * game_config::MOVE_SPEED;
            }
            if input.pressed_keys.contains(&Scancode::D) {
                velocity += angle.right() * game_config::MOVE_SPEED;
            }
            if input.pressed_keys.contains(&Scancode::A) {
                velocity -= *angle.right() * game_config::MOVE_SPEED;
            }
            if input.pressed_keys.contains(&Scancode::Space) {
                velocity += up_on_ground * game_config::MOVE_SPEED;
            }
            if input.pressed_keys.contains(&Scancode::LShift) {
                velocity -= up_on_ground * game_config::MOVE_SPEED;
            }

            vel.0 = velocity;
        }
    }
}
