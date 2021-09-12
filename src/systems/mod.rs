use nalgebra::{Isometry3, Point3, Vector3};
use parry3d::bounding_volume::{BoundingVolume, AABB};
use parry3d::query::{Ray, RayCast};
use specs::ReadExpect;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::game_config;
use crate::mymath::ChunkPos;
use crate::world::GameWorld;

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

pub struct VelocityAdjusterForCollisions;

impl<'a> System<'a> for VelocityAdjusterForCollisions {
    type SystemData = (
        Read<'a, DeltaTick>,
        ReadExpect<'a, GameWorld>,
        ReadStorage<'a, Collider>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (delta, world, collider, pos, mut vel): Self::SystemData) {
        for (collider, pos, vel) in (&collider, &pos, &mut vel).join() {
            let aabbs = {
                let mut aabbs: Vec<AABB> = Vec::new();
                /* TODO: エンティティの周りのチャンクを取得する処理 */
                let nearby_chunks = vec![world
                    .get_chunk(&ChunkPos::new(Point3::<i32>::new(0, 0, 0)))
                    .unwrap()];
                for chunk in nearby_chunks {
                    aabbs.append(&mut chunk.aabbs_for_collision());
                }
                aabbs
            };

            let entity_aabb = collider
                .0
                .aabb(&Isometry3::new(pos.0.coords, Vector3::zeros()));

            // 現在のエンティティのAABBと、次のフレームでのエンティティのAABBを両方とも含むようなAABB
            let extended_entity_aabb = entity_aabb.merged(&AABB::from_half_extents(
                pos.0 + delta.0 as f32 * vel.0,
                collider.0.half_extents,
            ));

            let aabbs_and_extended_aabbs: Vec<(AABB, AABB)> = aabbs
                .iter()
                .map(|aabb| {
                    // extended_aabbは対象のAABBと中心が同じで、エンティティの大きさの分大きくなったもの。
                    // エンティティと対象のAABBとの当たり判定ではなく、
                    // エンティティの中心点とextended_aabbとの当たり判定を行えば良い。
                    let extended_aabb = AABB::from_half_extents(
                        aabb.center(),
                        aabb.half_extents() + entity_aabb.half_extents(),
                    );
                    (*aabb, extended_aabb)
                })
                .collect();

            while VelocityAdjusterForCollisions::get_adjusted_vel(
                &aabbs_and_extended_aabbs,
                &pos,
                vel,
                &extended_entity_aabb,
            ) {}
        }
    }
}

impl VelocityAdjusterForCollisions {
    /// 戻り値: entity_velが更新されたかどうか
    fn get_adjusted_vel(
        aabbs_and_extended_aabbs: &Vec<(AABB, AABB)>,
        entity_pos: &Position,
        entity_vel: &mut Velocity,
        extended_entity_aabb: &AABB,
    ) -> bool {
        let mut nearest_toi = std::f32::INFINITY;
        let mut nearest_normal: Option<Vector3<f32>> = None;
        for (aabb, extended_aabb) in aabbs_and_extended_aabbs {
            // エンティティが対象のAABBの中にいるときは当たり判定を行わない
            if extended_aabb.contains_local_point(&entity_pos.0) {
                continue;
            }

            // エンティティの行き先が対象のAABBと重ならないときは当たり判定を行わない
            if !extended_entity_aabb.intersects(&aabb) {
                continue;
            }

            if let Some(result) = extended_aabb.cast_local_ray_and_get_normal(
                &Ray::new(entity_pos.0, entity_vel.0),
                50f32, /*適当な値*/
                true,  /*意味が分からない*/
            ) {
                if result.toi < nearest_toi {
                    nearest_toi = result.toi;
                    nearest_normal = Some(result.normal);
                }
            }
        }

        // 壁ずりベクトルを求める
        if let Some(nearest_normal) = nearest_normal {
            entity_vel.0 = entity_vel.0 - entity_vel.0.dot(&nearest_normal) * nearest_normal;
            true
        } else {
            false
        }
    }
}
