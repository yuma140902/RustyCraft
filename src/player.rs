use nalgebra::{Point3, Vector3};
use parry3d::shape::Cuboid;

use crate::{
    components::{Acceleration, Angle2, Collider, OnGround, Position, Velocity},
    mymath::Deg,
};

pub struct Player {
    pub pos: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub angle: Angle2,
    pub collider: Collider,
    pub on_ground: OnGround,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Position(Point3::new(4.0, 2.5, 4.0)),
            velocity: Velocity::default(),
            acceleration: Acceleration::gravity(),
            angle: Angle2::new(Deg(225.0_f32), Deg(0.0_f32)),
            collider: Collider(Cuboid::new(Vector3::new(0.15, 0.45, 0.15))),
            on_ground: OnGround(false),
        }
    }
}
