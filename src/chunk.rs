use std::collections::HashMap;

use cgmath::Point3;
use cgmath::Vector3;
use gl::Gl;

use crate::{
    block::Block, buffer_builder::BufferBuilder, texture::texture_atlas::TextureUV, vertex::Vertex,
};

pub struct Chunk {
    blocks: Vec<Option<Block>>,
    position: Point3<i32>,
}

impl Chunk {
    pub fn new(position: Point3<i32>) -> Chunk {
        Chunk {
            blocks: std::iter::repeat_with(|| None).take(16 * 16 * 16).collect(),
            position,
        }
    }

    pub fn set_block(&mut self, block: &Block, x: i32, y: i32, z: i32) {
        let index = (16 * 16 * y + 16 * z + x) as usize;
        let _old = std::mem::replace(&mut self.blocks[index], Some(*block));
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<Block> {
        let index = (16 * 16 * y + 16 * z + x) as usize;
        self.blocks[index]
    }

    pub fn generate_vertex_obj(&self, gl: &Gl, textures: &HashMap<&str, TextureUV>) -> Vertex {
        let mut buffer_builder = BufferBuilder::with_capacity(100); //TODO: 100は適当。6 * 16^3 なら確実
        let chunk_position: Vector3<f32> = Vector3::<f32> {
            x: (self.position.x * 16) as f32,
            y: (self.position.y * 16) as f32,
            z: (self.position.z * 16) as f32,
        };

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let index = (16 * 16 * y + 16 * z + x) as usize;
                    let block = self.blocks[index];
                    if block.is_none() {
                        continue;
                    }
                    let block = block.unwrap();
                    let block_pos = Point3::<f32> {
                        x: x as f32,
                        y: y as f32,
                        z: z as f32,
                    };
                    buffer_builder.add_block(&(block_pos + chunk_position), &block, textures);
                }
            }
        }

        buffer_builder.generate_vertex_obj(gl)
    }
}
