pub mod tile;
use tile::Tile;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

#[derive(Debug, Copy, Clone)]
pub struct Board{
    pub tiles: [[Option<Tile>; WIDTH]; HEIGHT]
}

impl Board{
    pub fn set_tile(&mut self, x: usize, y: usize, val: usize){
        if let Some(_i) = self.tiles[y][x] {
            self.tiles[y][x] = Some(Tile{x: x, y: y, value: val});
        } else {
            println!("Error!")
        }
    }
}


pub fn create_tiles() -> [[Option<Tile>; WIDTH]; HEIGHT] {
    let mut tiles: [[Option<Tile>; WIDTH]; HEIGHT] = [[None; WIDTH]; HEIGHT];
    for y in 0..HEIGHT{
        for x in 0..WIDTH{
            tiles[y][x] = Some(Tile{x: x, y: y, value: 0});
        }
    }
    return tiles;
}