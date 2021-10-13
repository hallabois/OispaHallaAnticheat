mod board;
use board::Board;
use board::create_tiles;

fn print_board(board: Board){
    for y in 0..board.tiles.len(){
        for x in 0..board.tiles[0].len(){
            match board.tiles[y][x] {
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
    board.set_tile(0, 1, 2);
    //println!("{:?}", board.tiles);
    print_board(board);
}

fn getPossibleMoves(board: Board){

}