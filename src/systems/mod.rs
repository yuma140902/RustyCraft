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

pub struct VelocityUpdater;

impl<'a> System<'a> for VelocityUpdater {
    type SystemData = (
        Read<'a, DeltaTick>,
        ReadStorage<'a, Acceleration>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (delta, acc, mut vel): Self::SystemData) {
        for (acc, vel) in (&acc, &mut vel).join() {
            vel.0 += acc.0 * delta.0 as f32;
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
        ReadStorage<'a, OnGround>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (input, angle, is_on_ground, mut vel): Self::SystemData) {
        for (_input, _angle, _is_on_ground, vel) in (&input, &angle, &is_on_ground, &mut vel).join()
        {
            let velocity = Vector3::<f32>::new(0.0, 0.0, 0.0);

            vel.0.x = 0f32;
            vel.0.z = 0f32;
            vel.0 += velocity;
        }
    }
}

/// 当たり判定を行い、VelocityやOnGroundを更新する
pub struct CollisionHandler;

impl<'a> System<'a> for CollisionHandler {
    type SystemData = (
        Read<'a, DeltaTick>,
        ReadExpect<'a, GameWorld>,
        ReadStorage<'a, Collider>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, OnGround>,
    );

    fn run(&mut self, (delta, world, collider, pos, mut vel, mut is_on_ground): Self::SystemData) {
        for (collider, pos, vel, is_on_ground) in
            (&collider, &pos, &mut vel, &mut is_on_ground).join()
        {
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

            while CollisionHandler::get_adjusted_vel(
                &aabbs_and_extended_aabbs,
                &pos,
                vel,
                &delta,
                &entity_aabb,
            ) {}

            let mut on_ground_test_vel = Velocity(Vector3::new(0.0f32, -0.001f32, 0.0f32));

            if CollisionHandler::get_adjusted_vel(
                &aabbs_and_extended_aabbs,
                &Position(pos.0 + vel.0 * delta.0 as f32),
                &mut on_ground_test_vel,
                &delta,
                &entity_aabb,
            ) {
                is_on_ground.0 = true;
            } else {
                is_on_ground.0 = false;
            }
        }
    }
}

impl CollisionHandler {
    /// 戻り値: entity_velが更新されたかどうか
    fn get_adjusted_vel(
        aabbs_and_extended_aabbs: &Vec<(AABB, AABB)>,
        entity_pos: &Position,
        entity_vel: &mut Velocity,
        delta: &DeltaTick,
        entity_aabb: &AABB,
    ) -> bool {
        // 現在のエンティティのAABBと、次のフレームでのエンティティのAABBを両方とも含むようなAABB
        let extended_entity_aabb = entity_aabb.merged(&AABB::from_half_extents(
            entity_pos.0 + entity_vel.0 * delta.0 as f32,
            entity_aabb.half_extents(),
        ));

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
