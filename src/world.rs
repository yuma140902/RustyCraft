use std::collections::HashMap;

use crate::mymath::ChunkPos;

use super::chunk::Chunk;

pub struct GameWorld {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl GameWorld {
    pub fn new() -> GameWorld {
        GameWorld {
            chunks: HashMap::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk: Chunk) -> Result<(), ()> {
        if self.chunks.contains_key(chunk.position()) {
            return Err(());
        }

        self.chunks.insert(*chunk.position(), chunk);

        Ok(())
    }

    pub fn get_chunk(&self, pos: &ChunkPos) -> Option<&Chunk> {
        self.chunks.get(pos)
    }
}
