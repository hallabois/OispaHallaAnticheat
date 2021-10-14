#![feature(proc_macro_hygiene, decl_macro)]

mod board;

use board::Board;
use board::create_tiles;
use board::is_move_possible;

mod direction;
use direction::Direction;

mod recording;
use recording::Recording;
use rocket::http::Method;
use rocket_cors::AllowedHeaders;
use rocket_cors::AllowedOrigins;

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
    board.set_tile(3, 1, 4);
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
    //println!( "{:#?}", parse_data("0.0.0.0.0.4.0.0.2.0.0.0.0.0.0.0;e:2.4.0.0.0.0.0.0.0.0.2.0.0.0.0.0;e:0.0.2.4.0.0.0.0.0.0.2.2.0.0.0.0;e:2.4.0.0.0.0.0.0.4.0.0.2.0.0.0.0;e".to_owned()) );
    println!( "{:#?}", validate_history( parse_data("0.0.0.0.0.0.0.0.2.0.2.0.0.0.0.0;3:0.0.0.0.0.2.0.0.4.0.0.0.0.0.0.0;0:4.2.2.0.0.0.0.0.0.0.0.0.0.0.0.0;1:0.0.4.4.0.0.0.0.0.2.0.0.0.0.0.0;2:0.0.0.0.0.0.0.0.0.0.0.0.2.2.4.4;3:0.0.0.0.0.0.0.0.0.2.0.0.4.8.0.0;0:4.2.2.0.0.8.0.0.0.0.0.0.0.0.0.0;e".to_owned()) ));
    demo();
    println!("Start the web server:");
    
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8080", "https://oisphalla.com", "http://oispahalla.com"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().expect("Cors did not set up correctly!");
    rocket::ignite().mount("/HAC", routes![hello]).attach(cors).launch();
}

fn parse_data(data: String) -> Recording {
    let mut history: Vec < ( [[Option<board::tile::Tile>; board::WIDTH]; board::HEIGHT], Direction ) > = vec![];
    for step in data.split(":"){
        let parts = step.split(";").collect::<Vec<&str>>();
        let b = parts[0];
        let mut board = create_tiles();
        let dir = parts[1];
        let direction = match dir{
            "0" => {
                Direction::UP
            },
            "1" => {
                Direction::RIGHT
            },
            "2" => {
                Direction::DOWN
            },
            "3" => {
                Direction::LEFT
            },
            _ => {
                Direction::END
            }
        };
        let mut index: usize = 0;
        for i in b.split("."){
            let val = i.parse::<usize>().unwrap();
            let x = index % board::WIDTH;
            let y = index / 10;
            board[ y ][ x ] = Some ( board::tile::Tile{x: x, y: y, value: val} );
            index += 1;
        }
        history.push( (board, direction) );
    }
    return Recording{ history };
}

fn validate_history(history: Recording) -> bool{
    let history_len = history.history.len();
    for ind in 0..history_len{
        let i = history.history[ind];
        if ind < history_len - 1 {
            let board_next = history.history[ind + 1].0;
            let board = i.0;
            let dir = i.1;
    
            let predicted = is_move_possible(Board { tiles: board }, dir);
            if(dir == Direction::END){

            }
            else if (predicted.1) && (predicted.0 == board_next) {

            }
            else{
                println!("Went wrong at index {}: ", ind);
                return false;
            }
        }
    }
    return true;
}

#[macro_use] extern crate rocket;

#[get("/validate/<run_json>")]
fn hello(run_json: String) -> String {
    let history = parse_data(run_json);
    let valid = validate_history(history);
    format!("Valid: {:#?}", valid)
}
