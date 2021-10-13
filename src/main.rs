#![feature(proc_macro_hygiene, decl_macro)]

mod board;
use std::string;

use board::Board;
use board::create_tiles;
use board::is_move_possible;

mod direction;
use direction::Direction;

mod recording;
use recording::Recording;

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

fn demo(){
    println!("Sanity Check:");
    println!("-------------");
    let mut board: Board = Board{
        tiles: create_tiles()
    };
    board.set_tile(0, 0, 2);
    board.set_tile(1, 0, 2);
    board.set_tile(5, 1, 4);
    //println!("{:?}", board.tiles);
    print_board(board.tiles);
    for i in [ Direction::LEFT, Direction::DOWN ].iter() {
        let next = is_move_possible(board, *i);
        if next.1 {
            println!("Next state: ");
            print_board(next.0);
        }
        else {
            println!("Move not possible!")
        }
        board.tiles = next.0;
    }
    println!("-------------");
}

fn main() {
    //println!( "{}", parse_data("0.0:1.2;UP;4.0;LEFT;".to_string()) );
    //demo();
    //println!("Start the web server:");
    rocket::ignite().mount("/HAC", routes![hello]).launch();
}

fn parse_data(data: String) -> Recording {
    let history: Vec < ( [[Option<board::tile::Tile>; board::WIDTH]; board::HEIGHT], Direction ) > = vec![];
    for step in data.split(":"){
        let parts = step.split(";")
        let board = parts[0];
        let dir = parts[1];
    }
    return Recording{ history };
}

#[macro_use] extern crate rocket;

#[get("/validate/<run_json>")]
fn hello(run_json: String) -> String {
    format!("Received: {}", run_json)
}
