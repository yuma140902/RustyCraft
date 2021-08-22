use std::collections::{hash_map::Iter, HashMap};

use crate::block::Block;

pub struct World {
    blocks: HashMap<(i32, i32, i32), Block>,
}

impl World {
    pub fn new() -> World {
        let mut blocks = HashMap::new();
        blocks.insert((0, 0, 0), Block::PLAIN);
        World { blocks }
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<&Block> {
        self.blocks.get(&(x, y, z))
    }

    pub fn blocks(&self) -> Iter<(i32, i32, i32), Block> {
        self.blocks.iter()
    }
}
