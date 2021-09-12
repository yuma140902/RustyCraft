use std::collections::HashMap;

use gl::Gl;
use parry3d::bounding_volume::AABB;

use crate::mymath::BlockPosInChunk;
use crate::mymath::BlockPosInWorld;
use crate::mymath::ChunkPos;
use crate::{
    block::Block, buffer_builder::BufferBuilder, texture::texture_atlas::TextureUV, vertex::Vertex,
};

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

    pub fn generate_vertex_obj(&self, gl: &Gl, textures: &HashMap<&str, TextureUV>) -> Vertex {
        let mut buffer_builder = BufferBuilder::with_capacity(100); //TODO: 100は適当。6 * 16^3 なら確実

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
                    buffer_builder.add_block(
                        &BlockPosInWorld::from_chunk_pos(&self.position, &block_pos),
                        &block,
                        textures,
                    );
                }
            }
        }

        buffer_builder.generate_vertex_obj(gl)
    }
}
