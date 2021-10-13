mod board;
use board::Board;
use board::create_tiles;
use board::is_move_possible;

mod direction;
use direction::Direction;

fn print_board(tiles: [[Option<board::tile::Tile>; board::WIDTH]; board::HEIGHT]){
    for y in 0..tiles.len(){
        for x in 0..tiles[0].len(){
            match tiles[y][x] {
                Some(i) => {
                    let string = i.value.to_string();
                    print!("{}\t", if i.value == 0 {"."} else {string.as_str()} )
                },
                None => print!("?\t")
            }
        }
        println!("");
    }
}

fn main() {
    println!("Hello, world!");
    let mut board: Board = Board{
        tiles: create_tiles()
    };
    board.set_tile(0, 0, 2);
    board.set_tile(1, 0, 2);
    board.set_tile(5, 1, 4);
    //println!("{:?}", board.tiles);
    print_board(board.tiles);
    let next = is_move_possible(board, Direction::LEFT);
    if next.1 {
        println!("Next state: ");
        print_board(next.0);
    }
    else {
        println!("Move not possible anymore!")
    }
}
