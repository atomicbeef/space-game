pub mod command;
pub mod mesh;
pub mod plugin;

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::block::{Block, BlockMaterial};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GridPos {
    x: i16,
    y: i16,
    z: i16,
}

impl GridPos {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }
}

#[derive(Component)]
pub struct Grid {
    width: i16,
    height: i16,
    depth: i16,
    blocks: HashMap<GridPos, Block>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            depth: 0,
            blocks: HashMap::new(),
        }
    }

    pub fn width(&self) -> i16 {
        self.width
    }

    pub fn height(&self) -> i16 {
        self.height
    }

    pub fn depth(&self) -> i16 {
        self.depth
    }

    pub fn get(&self, pos: GridPos) -> Option<Block> {
        self.blocks.get(&pos).copied()
    }

    fn recalculate_dimensions(&mut self) {
        if self.blocks.len() == 0 {
            self.width = 0;
            self.height = 0;
            self.depth = 0;
            return;
        }

        // Safe to unwrap since we've checked if the grid is empty
        let min_x = self.blocks.keys().min_by(|&a, &b| a.x.cmp(&b.x)).unwrap();
        let max_x = self.blocks.keys().max_by(|&a, &b| a.x.cmp(&b.x)).unwrap();
        let min_y = self.blocks.keys().min_by(|&a, &b| a.y.cmp(&b.y)).unwrap();
        let max_y = self.blocks.keys().max_by(|&a, &b| a.y.cmp(&b.y)).unwrap();
        let min_z = self.blocks.keys().min_by(|&a, &b| a.z.cmp(&b.z)).unwrap();
        let max_z = self.blocks.keys().max_by(|&a, &b| a.z.cmp(&b.z)).unwrap();

        self.width = max_x.x - min_x.x + 1;
        self.height = max_y.y - min_y.y + 1;
        self.depth = max_z.z - min_z.z + 1;
    }

    pub fn set(&mut self, pos: GridPos, block: Block) {
        // Since a HashMap is used, we can just not store the block if it's empty
        match block.material {
            BlockMaterial::Empty => {
                self.blocks.remove(&pos);
            },
            _ => {
                self.blocks.insert(pos, block);
            },
        }

        self.recalculate_dimensions();
    }
}

#[derive(Event)]
pub struct GridChanged(pub Entity);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_dimensions_update_when_adding_blocks() {
        let mut grid = Grid::new();

        grid.set(GridPos::new(0, 0, 0), Block { material: BlockMaterial::Aluminum });

        assert_eq!(grid.width(), 1);
        assert_eq!(grid.height(), 1);
        assert_eq!(grid.depth(), 1);
    }

    #[test]
    fn grid_dimensions_update_when_removing_blocks() {
        let mut grid = Grid::new();

        grid.set(GridPos::new(0, 0, 0), Block { material: BlockMaterial::Aluminum });
        grid.set(GridPos::new(0, 0, 0), Block { material: BlockMaterial::Empty });

        assert_eq!(grid.width(), 0);
        assert_eq!(grid.height(), 0);
        assert_eq!(grid.depth(), 0);
    }

    #[test]
    fn grid_dimensions_correct_with_non_contiguous_blocks() {
        let mut grid = Grid::new();

        grid.set(GridPos::new(0, 0, 0), Block { material: BlockMaterial::Aluminum });
        grid.set(GridPos::new(5, 5, 5), Block { material: BlockMaterial::Aluminum });

        assert_eq!(grid.width(), 6);
        assert_eq!(grid.height(), 6);
        assert_eq!(grid.depth(), 6);
    }

    #[test]
    fn grid_dimensions_correct_when_removing_non_contiguous_blocks() {
        let mut grid = Grid::new();

        grid.set(GridPos::new(0, 0, 0), Block { material: BlockMaterial::Aluminum });
        grid.set(GridPos::new(5, 5, 5), Block { material: BlockMaterial::Aluminum });
        grid.set(GridPos::new(5, 5, 5), Block { material: BlockMaterial::Empty });

        assert_eq!(grid.width(), 1);
        assert_eq!(grid.height(), 1);
        assert_eq!(grid.depth(), 1);
    }

    #[test]
    fn grid_dimensions_correct_when_removing_origin() {
        let mut grid = Grid::new();

        grid.set(GridPos::new(0, 0, 0), Block { material: BlockMaterial::Aluminum });
        grid.set(GridPos::new(0, 1, 0), Block { material: BlockMaterial::Aluminum });
        grid.set(GridPos::new(0, 0, 0), Block { material: BlockMaterial::Empty });

        assert_eq!(grid.width(), 1);
        assert_eq!(grid.height(), 1);
        assert_eq!(grid.depth(), 1);
    }
}