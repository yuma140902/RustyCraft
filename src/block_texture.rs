use re::types::Const;
use re::CuboidTextures;
use re::TextureUV;
use reverie_engine as re;

pub struct BlockTextures {
    pub side: TextureUV<Const<64>, Const<64>, Const<256>, Const<256>>,
    pub top: TextureUV<Const<64>, Const<64>, Const<256>, Const<256>>,
    pub bottom: TextureUV<Const<64>, Const<64>, Const<256>, Const<256>>,
}

pub fn generate_cuboid_texture<'a>(
    block_textures: &'a BlockTextures,
) -> CuboidTextures<'a, TextureUV<Const<64>, Const<64>, Const<256>, Const<256>>> {
    CuboidTextures {
        top: &block_textures.top,
        bottom: &block_textures.bottom,
        south: &block_textures.side,
        north: &block_textures.side,
        west: &block_textures.side,
        east: &block_textures.side,
    }
}
