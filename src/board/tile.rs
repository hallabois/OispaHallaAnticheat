#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tile{
    pub x: usize,
    pub y: usize,
    pub value: usize,
    pub merged: bool
}
