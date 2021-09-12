use nalgebra::Vector3;
use parry3d::bounding_volume::AABB;

use crate::mymath::BlockPosInWorld;

pub enum Side {
    TOP,
    BOTTOM,
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

// TODO:外部ファイルでブロックの一覧を宣言するようにしたい
#[derive(Clone, Copy)]
pub enum Block {
    GrassBlock,
}

pub fn get_block_aabbs(_block: &Block, pos: &BlockPosInWorld) -> Vec<AABB> {
    vec![AABB::new(
        pos.cast() * 0.5,
        pos.cast() * 0.5 + Vector3::new(0.5, 0.5, 0.5),
    )]
}
