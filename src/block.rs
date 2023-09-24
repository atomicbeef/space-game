pub const BLOCK_SIZE: f32 = 0.25;

#[derive(Clone, Copy, Debug)]
pub enum BlockMaterial {
    Empty,
    Aluminum,
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub material: BlockMaterial,
}