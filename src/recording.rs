use crate::board::tile::Tile;
use crate::board::WIDTH;
use crate::board::HEIGHT;
use crate::direction::Direction;

#[derive(Debug)]
pub struct Recording{
    pub history: Vec<( [[Option<Tile>; WIDTH]; HEIGHT], Direction, Option<Tile> )>
}