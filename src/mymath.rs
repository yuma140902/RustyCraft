use std::ops::{Deref, DerefMut};

use nalgebra::{Point3, Vector3};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct BlockPosInChunk(nalgebra::Point3<u32>);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ChunkPos(nalgebra::Point3<i32>);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct BlockPosInWorld(nalgebra::Point3<i32>);
pub type BlockPos = BlockPosInWorld;

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Deg(pub f32);

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Rad(pub f32);

impl BlockPosInChunk {
    pub fn new(x: u32, y: u32, z: u32) -> Option<Self> {
        if x < 16 && y < 16 && z < 16 {
            Some(Self {
                0: Point3::<u32>::new(x, y, z),
            })
        } else {
            None
        }
    }

    pub fn index(&self) -> usize {
        (16 * 16 * self.y + 16 * self.z + self.x) as usize
    }
}
impl Deref for BlockPosInChunk {
    type Target = Point3<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ChunkPos {
    pub fn new(pos: Point3<i32>) -> Self {
        Self { 0: pos }
    }
}
impl Deref for ChunkPos {
    type Target = Point3<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BlockPosInWorld {
    pub fn new(point: Point3<i32>) -> Self {
        Self { 0: point }
    }

    pub fn from_chunk_pos(chunk_pos: &ChunkPos, block_pos: &BlockPosInChunk) -> Self {
        let chunk_pos: Vector3<i32> = Vector3::<i32>::new(
            (chunk_pos.x * 16) as i32,
            (chunk_pos.y * 16) as i32,
            (chunk_pos.z * 16) as i32,
        );
        let block_pos: Point3<i32> =
            Point3::<i32>::new(block_pos.x as i32, block_pos.y as i32, block_pos.z as i32);

        return Self::new(block_pos + chunk_pos);
    }
}
impl Deref for BlockPosInWorld {
    type Target = Point3<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deg {
    pub fn rad(&self) -> Rad {
        Rad(self.rad_f32())
    }

    #[inline]
    fn rad_f32(&self) -> f32 {
        self.0 * std::f32::consts::PI / 180f32
    }

    pub fn sin(&self) -> f32 {
        self.rad_f32().sin()
    }

    pub fn cos(&self) -> f32 {
        self.rad_f32().cos()
    }
}
impl Deref for Deg {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Deg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Rad {
    pub fn deg(&self) -> Deg {
        Deg(self.0 * 180f32 / std::f32::consts::PI)
    }
}
impl Deref for Rad {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
