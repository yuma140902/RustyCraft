use std::collections::HashMap;

use crate::texture_atlas::TextureUV;

pub fn get_textures_in_atlas(
    atlas_width: u32,
    atlas_height: u32,
) -> HashMap<&'static str, TextureUV> {
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
