mod board;
use board::Board;
use board::create_tiles;
use board::get_possible_moves;

mod direction;
use direction::Direction;

fn print_board(tiles: [[Option<board::tile::Tile>; board::WIDTH]; board::HEIGHT]){
    for y in 0..tiles.len(){
        for x in 0..tiles[0].len(){
            match tiles[y][x] {
                Some(i) => print!("{}\t", i.value),
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
    board.set_tile(0, 1, 4);
    //println!("{:?}", board.tiles);
    print_board(board.tiles);
    let possibilities = get_possible_moves(board, Direction::LEFT);
    for i in possibilities{
        println!("Possibility: ");
        print_board(i);
    }
}
