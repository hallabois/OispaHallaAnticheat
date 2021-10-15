pub mod tile;
use tile::Tile;

pub const WIDTH: usize = 4;
pub const HEIGHT: usize = 4;

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
    pub fn get_non_occupied_tiles(&self) -> Vec<Tile> {
        let mut out: Vec<Tile> = vec![];
        for y in 0..HEIGHT{
            for x in 0..WIDTH{
                let t = self.tiles[y][x];
                match t{
                    Some(tile) => (
                        if tile.value == 0 {
                            out.push(tile)
                        }
                    ),
                    None => println!("Error! (pt. 2)")
                }
            }
        }
        out
    }
    pub fn get_score(&self) -> usize {
        let mut sum: usize = 0;
        for row in self.tiles{
            for i in row{
                match i{
                    Some(t) => sum += t.value,
                    None => ()
                }
            }
        }
        return sum;
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

pub fn get_closest_tile(t: Tile, viable_tiles: &Vec<Tile>, dir: Direction, mask: usize) -> Tile{ //if t is returned, an error occured along the way
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
                    closest = *i;
                    closest_dist = distance;
                }
                else if distance != 0 && i.value != mask{
                    return t;
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
                    closest = *i;
                    closest_dist = distance;
                }
                else if distance != 0 && i.value != mask{
                    return t;
                }
            }
        }
    }
    return closest;
}

pub fn get_farthest_tile(t: Tile, viable_tiles: &Vec<Tile>, dir: Direction, mask: usize) -> Tile{ //if t is returned, an error occured along the way
    let dir_x = dir.get_x();
    let dir_y = dir.get_y();

    let mut farthest = t;
    let mut farthest_dist: usize = usize::MIN;

    if dir_y == 0{ // A vertical move
        for i in viable_tiles{
            let condition = if dir_x > 0 { t.x < i.x } else { t.x > i.x };
            if (t.y == i.y) && condition {
                let distance = if dir_x > 0 { i.x - t.x } else { t.x - i.x };
                if distance != 0 && distance > farthest_dist {
                    farthest = *i;
                    farthest_dist = distance;
                }
                else if distance != 0 && i.value != mask{
                    return t;
                }
            }
        }
    }
    else { // A horizontal move
        for i in viable_tiles{
            let condition = if dir_y > 0 { t.y < i.y } else { t.y > i.y };
            if (t.x == i.x) && condition {
                let distance = if dir_y > 0 { i.y - t.y } else { t.y - i.y };
                if distance != 0 && distance > farthest_dist {
                    farthest = *i;
                    farthest_dist = distance;
                }
                else if distance != 0 && i.value != mask{
                    return t;
                }
            }
        }
    }
    return farthest;
}

pub fn is_move_possible(board: Board, dir: Direction) -> ( [[Option<Tile>; WIDTH]; HEIGHT], bool ) {

    if dir == Direction::END {
        return (board.tiles, true);
    }

    let tiles = board.get_occupied_tiles();

    let mut was_changed = false;

    // clone the current board
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

    // Merge
    let mut merged_tiles: Vec<Tile> = vec![]; // we don't want to merge a tile more than once per turn
    for _r in 0..32{
        let b = Board{tiles: universe};
        let occupied_tiles= b.get_occupied_tiles();
        //println!("Occupied tiles: {}", occupied_tiles.len());
        for t in &occupied_tiles{
            if merged_tiles.contains(t) {
                // Do nothing
            }
            else{
                let closest = get_closest_tile(*t, &occupied_tiles, dir, t.value);
                if t != &closest && t.value == closest.value && !merged_tiles.contains(&closest){
                    
                    universe[t.y][t.x] = Some( Tile{x: t.x, y: t.y, value: 0} );
                    let merged = Tile{x: closest.x, y: closest.y, value: closest.value*2};
                    universe[closest.y][closest.x] = Some( merged );
                    merged_tiles.push(merged);
                    was_changed = true;
                    println!("Merge {:?} + {:?} -> {:?}", t, closest, merged);
                    break; // HOTFIX, we only want the first one before updating occupied_tiles again
                }
            }
        }
    }

    // Slide
    let mut moved_tiles: Vec<Tile> = vec![];
    for _r in 0..32{
        let b = Board{tiles: universe};
        let tiles_post = b.get_occupied_tiles();
        let free_tiles = b.get_non_occupied_tiles();
        //println!("Free tiles: {}", free_tiles.len());
        for t in &tiles_post{
            if moved_tiles.contains(t){
                // Do nothing
            }
            else{
                let farthest_free = get_farthest_tile(*t, &free_tiles, dir, 0);

                if farthest_free != *t {
                    universe[t.y][t.x] = Some( Tile{x: t.x, y: t.y, value: 0} );
                    let new_tile = Tile{x: farthest_free.x, y: farthest_free.y, value: t.value};
                    universe[farthest_free.y][farthest_free.x] = Some( new_tile );
                    println!("Move {:?} -> {:?}", t, farthest_free);
                    moved_tiles.push(new_tile);
                    was_changed = true;
                    break; // HOTFIX, we only want the first one before updating tiles_post and free_tiles again
                }
            }
        }
    }

    return (universe, was_changed);
}