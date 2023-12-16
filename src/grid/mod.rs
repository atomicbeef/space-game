pub mod block;
pub mod chunk;
pub mod collider;
pub mod command;
pub mod mesh;
pub mod plugin;

use bevy::prelude::*;
use bevy::utils::HashMap;

use self::chunk::Chunk;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Component)]
pub struct ChunkPos {
    x: i16,
    y: i16,
    z: i16,
}

impl ChunkPos {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }
}

#[derive(Component)]
pub struct Grid {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }

    pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
        self.chunks.get_mut(&pos)
    }

    pub fn set_chunk(&mut self, pos: ChunkPos, chunk: Option<Chunk>) {
        match chunk {
            Some(chunk) => {
                self.chunks.insert(pos, chunk);
            }
            None => {
                self.chunks.remove(&pos);
            }
        }
    }
}

#[derive(Event)]
pub struct ChunkChanged {
    pub grid_entity: Entity,
    pub chunk_pos: ChunkPos,
}
