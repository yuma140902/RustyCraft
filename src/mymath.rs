use std::ops::Deref;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BlockPosInChunk(cgmath::Point3<u32>);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ChunkPos(cgmath::Point3<i32>);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BlockPosInWorld(cgmath::Point3<i32>);
pub type BlockPos = BlockPosInWorld;

impl BlockPosInChunk {
    pub fn new(x: u32, y: u32, z: u32) -> Option<Self> {
        if x < 16 && y < 16 && z < 16 {
            Some(Self {
                0: cgmath::Point3::<u32> { x, y, z },
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
    type Target = cgmath::Point3<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ChunkPos {
    pub fn new(pos: cgmath::Point3<i32>) -> Self {
        Self { 0: pos }
    }
}
impl Deref for ChunkPos {
    type Target = cgmath::Point3<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BlockPosInWorld {
    pub fn new(point: cgmath::Point3<i32>) -> Self {
        Self { 0: point }
    }

    pub fn from_chunk_pos(chunk_pos: &ChunkPos, block_pos: &BlockPosInChunk) -> Self {
        let chunk_pos: cgmath::Vector3<i32> = cgmath::Vector3::<i32> {
            x: (chunk_pos.x * 16) as i32,
            y: (chunk_pos.y * 16) as i32,
            z: (chunk_pos.z * 16) as i32,
        };
        let block_pos: cgmath::Point3<i32> = cgmath::Point3::<i32> {
            x: block_pos.x as i32,
            y: block_pos.y as i32,
            z: block_pos.z as i32,
        };

        return Self::new(block_pos + chunk_pos);
    }
}
impl Deref for BlockPosInWorld {
    type Target = cgmath::Point3<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
