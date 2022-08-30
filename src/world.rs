use re::gl::Gl;
use re::Vao;
use re::VaoBuffer;
use re::VaoBuilder3DGeometry;
use re::VaoConfig;
use reverie_engine as re;

use crate::block_texture;
use crate::block_texture::BlockTextures;
use crate::mymath::BlockPos;

pub struct World {
    blocks: Vec<bool>,
}

impl World {
    pub fn new() -> World {
        World {
            blocks: std::iter::repeat(false).take(16 * 16 * 16).collect(),
        }
    }

    pub fn set_block(&mut self, pos: &BlockPos) {
        let _old = std::mem::replace(&mut self.blocks[pos.index()], true);
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
                    if !block {
                        continue;
                    }
                    let block_pos = BlockPos::new(x, y, z).unwrap();
                    add_block(&mut buffer_builder, &block_pos, textures);
                }
            }
        }

        buffer_builder.build(gl, config)
    }
}

type Vector3 = nalgebra::Vector3<f32>;

const BLOCK_SIZE: Vector3 = Vector3::new(1.0, 1.0, 1.0);

fn add_block(builder: &mut VaoBuffer, begin: &BlockPos, textures: &BlockTextures) {
    let begin = begin.cast::<f32>();
    builder.add_cuboid(
        &begin,
        &(begin + BLOCK_SIZE),
        &block_texture::generate_cuboid_texture(textures),
    );
}
