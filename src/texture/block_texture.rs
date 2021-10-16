use std::collections::HashMap;

use crate::block::{Block, Side};

use re::texture::texture_atlas::TextureUV;
use re::vao::vao_builder::CuboidTextures;
use reverie_engine as re;

pub type BlockTextures = HashMap<&'static str, TextureUV>;

pub fn get_textures_in_atlas(atlas_width: u32, atlas_height: u32) -> BlockTextures {
    let mut dic = HashMap::new();
    dic.insert(
        "grass_side",
        TextureUV::of_atlas(0, 0, 64, 64, atlas_width, atlas_height),
    );
    dic.insert(
        "grass_top",
        TextureUV::of_atlas(0, 1, 64, 64, atlas_width, atlas_height),
    );
    dic.insert(
        "grass_bottom",
        TextureUV::of_atlas(0, 2, 64, 64, atlas_width, atlas_height),
    );
    dic
}

// TODO: jsonで宣言したり、luaで計算したりさせたい
pub fn get_texture_name(block: &Block, side: Side) -> &str {
    match block {
        Block::GrassBlock => match side {
            Side::TOP => "grass_top",
            Side::BOTTOM => "grass_bottom",
            _ => "grass_side",
        },
    }
}

pub fn generate_cuboid_texture<'a>(
    block: &Block,
    block_textures: &'a BlockTextures,
) -> CuboidTextures<'a> {
    CuboidTextures {
        top: block_textures
            .get(get_texture_name(block, Side::TOP))
            .unwrap(),
        bottom: block_textures
            .get(get_texture_name(block, Side::BOTTOM))
            .unwrap(),
        south: block_textures
            .get(get_texture_name(block, Side::SOUTH))
            .unwrap(),
        north: block_textures
            .get(get_texture_name(block, Side::NORTH))
            .unwrap(),
        west: block_textures
            .get(get_texture_name(block, Side::WEST))
            .unwrap(),
        east: block_textures
            .get(get_texture_name(block, Side::EAST))
            .unwrap(),
    }
}
