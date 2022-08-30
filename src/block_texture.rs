use std::collections::HashMap;

use crate::block::Side;

use re::types::Const;
use re::CuboidTextures;
use re::TextureAtlasPos;
use re::TextureUV;
use reverie_engine as re;

pub type BlockTextures =
    HashMap<&'static str, TextureUV<Const<64>, Const<64>, Const<256>, Const<256>>>;

pub fn get_textures_in_atlas() -> BlockTextures {
    let mut dic = HashMap::new();
    dic.insert(
        "grass_side",
        TextureUV::of_atlas(&TextureAtlasPos::new(0, 0)),
    );
    dic.insert(
        "grass_top",
        TextureUV::of_atlas(&TextureAtlasPos::new(0, 1)),
    );
    dic.insert(
        "grass_bottom",
        TextureUV::of_atlas(&TextureAtlasPos::new(0, 2)),
    );
    dic
}

// TODO: jsonで宣言したり、luaで計算したりさせたい
pub fn get_texture_name(side: Side) -> &'static str {
    match side {
        Side::TOP => "grass_top",
        Side::BOTTOM => "grass_bottom",
        _ => "grass_side",
    }
}

pub fn generate_cuboid_texture<'a>(
    block_textures: &'a BlockTextures,
) -> CuboidTextures<'a, TextureUV<Const<64>, Const<64>, Const<256>, Const<256>>> {
    CuboidTextures {
        top: block_textures.get(get_texture_name(Side::TOP)).unwrap(),
        bottom: block_textures.get(get_texture_name(Side::BOTTOM)).unwrap(),
        south: block_textures.get(get_texture_name(Side::SOUTH)).unwrap(),
        north: block_textures.get(get_texture_name(Side::NORTH)).unwrap(),
        west: block_textures.get(get_texture_name(Side::WEST)).unwrap(),
        east: block_textures.get(get_texture_name(Side::EAST)).unwrap(),
    }
}
