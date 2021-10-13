pub mod tile;
use tile::Tile;

pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;

use crate::Direction;

#[derive(Debug, Copy, Clone)]
pub struct Board{
    pub tiles: [[Option<Tile>; WIDTH]; HEIGHT]
}

impl Board{
    pub fn set_tile(&mut self, x: usize, y: usize, val: usize){
        if let Some(_i) = self.tiles[y][x] {
            self.tiles[y][x] = Some(Tile{x, y, value: val});
        } else {
            println!("Error!")
        }
    }
    pub fn get_occupied_tiles(&self) -> Vec<Tile> {
        let mut out: Vec<Tile> = vec![];
        for y in 0..HEIGHT{
            for x in 0..WIDTH{
                let t = self.tiles[y][x];
                match t{
                    Some(tile) => (
                        if tile.value != 0 {
                            out.push(tile)
                        }
                    ),
                    None => println!("Error! (pt. 2)")
                }
            }
        }
        out
    }
}

pub fn set_tile(mut tiles: [[Option<Tile>; WIDTH]; HEIGHT], x: usize, y: usize, val: usize){
    let mut row = tiles[y];
    match tiles[y][x] {
        Some(_i) => {
            row[x] = Some(Tile{x, y, value: val});
        }
        None => println!("Error!"),
    }
    tiles[y] = row;
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

pub fn get_next_tile(t: Tile, viable_tiles: Vec<Tile>, dir: Direction) -> Tile{ //if t is returned, an error occured along the way
    let dir_x = dir.get_x();
    let dir_y = dir.get_y();

    let mut closest = t;
    let mut closest_dist: usize = usize::MAX;

    if dir_y == 0{ // A vertical move
        for i in viable_tiles{
            let condition = if dir_x > 0 { t.x < i.x } else { t.x > i.x };
            if (t.y == i.y) && condition {
                let distance = if dir_x > 0 { i.x - t.x } else { t.x - i.x };
                if distance != 0 && distance < closest_dist {
                    closest = i;
                    closest_dist = distance;
                }
            }
        }
    }
    else { // A horizontal move
        for i in viable_tiles{
            let condition = if dir_y > 0 { t.y < i.y } else { t.y > i.y };
            if (t.x == i.x) && condition {
                let distance = if dir_y > 0 { i.y - t.y } else { t.y - i.y };
                if distance != 0 && distance < closest_dist {
                    closest = i;
                    closest_dist = distance;
                }
            }
        }
    }
    return closest;
}

pub fn get_possible_moves(board: Board, dir: Direction) -> Vec<[[Option<Tile>; WIDTH]; HEIGHT]> {
    let mut moves: Vec<[[Option<Tile>; WIDTH]; HEIGHT]> = vec![];
    let tiles = board.get_occupied_tiles();
    for t in tiles{
        let tiles_copy= board.get_occupied_tiles();
        let closest = get_next_tile(t, tiles_copy, dir);
        if t != closest && t.value == closest.value{
            let mut universe = create_tiles();
            for y in 0..HEIGHT{
                for x in 0..WIDTH{
                    match board.tiles[y][x] {
                        None => println!("Error (pt. 6)"),
                        Some(t2) => {
                            universe[t2.y][t2.x] = Some( Tile{x: t2.x, y: t2.y, value: t2.value} );
                        }
                    }
                }
            }
            universe[t.y][t.x] = Some( Tile{x: t.x, y: t.y, value: 0} );
            universe[closest.y][closest.x] = Some( Tile{x: closest.x, y: closest.y, value: closest.value*2} );
            moves.push(universe);
            println!("{:?} -> {:?}", t, closest);
        }
    }
    return moves;
}