pub mod block;
pub mod chunk;
pub mod collider;
pub mod command;
pub mod mesh;
pub mod plugin;

use std::ops::Add;

use bevy::prelude::*;
use bevy::utils::HashMap;

use self::chunk::{BlockPos, Chunk, CHUNK_SIZE};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Component)]
pub struct ChunkPos {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl ChunkPos {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GridPos {
    pub chunk_pos: ChunkPos,
    pub block_pos: BlockPos,
}

impl Add<(i16, i16, i16)> for GridPos {
    type Output = Self;

    fn add(self, rhs: (i16, i16, i16)) -> Self::Output {
        let mut block_pos = self.block_pos;
        let mut chunk_pos = self.chunk_pos;

        let x = block_pos.x as i16 + rhs.0;
        let y = block_pos.y as i16 + rhs.1;
        let z = block_pos.z as i16 + rhs.2;

        chunk_pos.x += x.div_euclid(CHUNK_SIZE as i16);
        chunk_pos.y += y.div_euclid(CHUNK_SIZE as i16);
        chunk_pos.z += z.div_euclid(CHUNK_SIZE as i16);

        block_pos.x = (x.rem_euclid(CHUNK_SIZE as i16)) as u8;
        block_pos.y = (y.rem_euclid(CHUNK_SIZE as i16)) as u8;
        block_pos.z = (z.rem_euclid(CHUNK_SIZE as i16)) as u8;

        Self::Output {
            block_pos,
            chunk_pos,
        }
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

#[derive(Resource)]
pub struct GridMaterialHandle(pub Handle<StandardMaterial>);
