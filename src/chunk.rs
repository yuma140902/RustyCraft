use parry3d::bounding_volume::AABB;

use re::gl::Gl;
use re::Vao;
use re::VaoBuffer;
use re::VaoBuilder3DGeometry;
use re::VaoConfig;
use reverie_engine as re;

use crate::block::Block;
use crate::block_texture;
use crate::block_texture::BlockTextures;
use crate::mymath::BlockPosInChunk;
use crate::mymath::BlockPosInWorld;
use crate::mymath::ChunkPos;

pub struct Chunk {
    blocks: Vec<Option<Block>>,
    position: ChunkPos,
}

impl Chunk {
    pub fn new(position: ChunkPos) -> Chunk {
        Chunk {
            blocks: std::iter::repeat_with(|| None).take(16 * 16 * 16).collect(),
            position,
        }
    }

    pub fn position(&self) -> &ChunkPos {
        &self.position
    }

    pub fn set_block(&mut self, block: &Block, pos: &BlockPosInChunk) {
        let _old = std::mem::replace(&mut self.blocks[pos.index()], Some(*block));
    }

    pub fn get_block(&self, pos: &BlockPosInChunk) -> Option<Block> {
        self.blocks[pos.index()]
    }

    pub fn aabbs_for_collision(&self) -> Vec<AABB> {
        let mut vec = Vec::<AABB>::new();
        /* ToDo: BlockPosInChunk のイテレータ */
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let pos = BlockPosInChunk::new(x, y, z).unwrap();
                    let index = pos.index();
                    if let Some(block) = self.blocks[index] {
                        let pos_in_world = BlockPosInWorld::from_chunk_pos(self.position(), &pos);
                        vec.append(&mut crate::block::get_block_aabbs(&block, &pos_in_world));
                    }
                }
            }
        }
        vec
    }

    pub fn generate_vertex_obj<'a>(
        &self,
        gl: &Gl,
        textures: &BlockTextures,
        config: &'a VaoConfig,
    ) -> Vao<'a> {
        let mut buffer_builder = VaoBuffer::new();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let index = (16 * 16 * y + 16 * z + x) as usize;
                    let block = self.blocks[index];
                    if block.is_none() {
                        continue;
                    }
                    let block = block.unwrap();
                    let block_pos = BlockPosInChunk::new(x, y, z).unwrap();
                    add_block(
                        &mut buffer_builder,
                        &BlockPosInWorld::from_chunk_pos(&self.position, &block_pos),
                        &block,
                        textures,
                    );
                }
            }
        }

        buffer_builder.build(gl, config)
    }
}

type Vector3 = nalgebra::Vector3<f32>;

pub const UP: Vector3 = Vector3::new(0.0, 1.0, 0.0);
pub const DOWN: Vector3 = Vector3::new(0.0, -1.0, 0.0);
pub const NORTH: Vector3 = Vector3::new(1.0, 0.0, 0.0);
pub const SOUTH: Vector3 = Vector3::new(-1.0, 0.0, 0.0);
pub const WEST: Vector3 = Vector3::new(0.0, 0.0, 1.0);
pub const EAST: Vector3 = Vector3::new(0.0, 0.0, -1.0);

const BLOCK_SIZE: Vector3 = Vector3::new(1.0, 1.0, 1.0);

fn add_block(
    builder: &mut VaoBuffer,
    begin: &BlockPosInWorld,
    block: &Block,
    textures: &BlockTextures,
) {
    let begin = begin.cast::<f32>();
    builder.add_cuboid(
        &begin,
        &(begin + BLOCK_SIZE),
        &block_texture::generate_cuboid_texture(block, textures),
    );
}
