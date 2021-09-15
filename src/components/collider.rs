use parry3d::shape::Cuboid;
use specs::{Component, HashMapStorage};

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Collider(pub Cuboid);
