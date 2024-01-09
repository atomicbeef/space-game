use bevy::prelude::*;

use crate::raycast_selection::Selectable;

use super::{block::Block, ChunkPos};

pub const CHUNK_SIZE: u8 = 16;
pub const CHUNK_SIZE_CUBED: usize = CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlockPos {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Clone)]
pub struct Chunk {
    pub entity: Entity,
    blocks: [Block; CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize],
}

impl Chunk {
    pub fn new(entity: Entity, blocks: [Block; CHUNK_SIZE_CUBED]) -> Self {
        Self { entity, blocks }
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

    pub fn get_by_block_pos(&self, pos: BlockPos) -> Block {
        self.get(pos.x, pos.y, pos.z)
    }

    pub fn set_by_block_pos(&mut self, pos: BlockPos, block: Block) {
        self.set(pos.x, pos.y, pos.z, block);
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
}

#[derive(Event)]
pub struct ChunkChanged(pub Entity);

#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk_pos: ChunkPos,
    pub spatial_bundle: SpatialBundle,
    pub selectable: Selectable,
}

impl ChunkBundle {
    pub fn new(chunk_pos: ChunkPos) -> Self {
        Self {
            chunk_pos,
            spatial_bundle: SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    chunk_pos.x as f32 * CHUNK_SIZE as f32 / 4.0,
                    chunk_pos.y as f32 * CHUNK_SIZE as f32 / 4.0,
                    chunk_pos.z as f32 * CHUNK_SIZE as f32 / 4.0,
                )),
                ..Default::default()
            },
            selectable: Selectable,
        }
    }
}
