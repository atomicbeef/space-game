use bevy::prelude::*;

use crate::grid::{block::Block, GridPos};

#[derive(Event)]
pub struct PlaceBlockRequest {
    pub grid: Entity,
    pub pos: GridPos,
    pub block: Block,
}
