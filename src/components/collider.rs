use parry3d::shape::Cuboid;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Collider(pub Cuboid);
