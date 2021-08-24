pub enum Side {
    TOP,
    BOTTOM,
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

pub mod blocks {
    use super::*;

    pub const GRASS_BLOCK: GrassBlock = GrassBlock {};
}

pub trait Block {
    fn get_texture_uv(&self, side: Side) -> &str;
}

pub struct GrassBlock {}

impl Block for GrassBlock {
    fn get_texture_uv(&self, side: Side) -> &str {
        match side {
            Side::TOP => "grass_top",
            Side::BOTTOM => "grass_bottom",
            _ => "grass_side",
        }
    }
}
