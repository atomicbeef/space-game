use bevy::prelude::*;

use super::block::Block;

pub const CHUNK_SIZE: u8 = 16;
pub const CHUNK_SIZE_CUBED: usize = CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize;

#[derive(Clone)]
pub struct Chunk {
    blocks: [Block; CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize],
}

impl Chunk {
    pub fn new(blocks: [Block; CHUNK_SIZE_CUBED]) -> Self {
        Self {
            blocks,
        }
    }

    pub fn pos_to_index(&self, x: u8, y: u8, z: u8) -> usize {
        assert!(x < CHUNK_SIZE);
        assert!(y < CHUNK_SIZE);
        assert!(z < CHUNK_SIZE);

        let chunk_size = CHUNK_SIZE as usize;
        let x = x as usize;
        let y = y as usize;
        let z = z as usize;

        chunk_size * chunk_size * z + y * chunk_size + x
    }

    pub fn get(&self, x: u8, y: u8, z: u8) -> Block {
        self.blocks[self.pos_to_index(x, y, z)]
    }

    pub fn set(&mut self, x: u8, y: u8, z: u8, block: Block) {
        self.blocks[self.pos_to_index(x, y, z)] = block;
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
}

#[derive(Event)]
pub struct ChunkChanged(pub Entity);