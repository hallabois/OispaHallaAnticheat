#![feature(proc_macro_hygiene, decl_macro)]

mod board;

use std::env;

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

const DEBUG_INFO: bool = false;
const HISTORY_CUTOFF: usize = usize::MAX;

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

fn give_help() {
    println!("G2048Engine");
    println!("\t--server\t\tstarts a webserver for HAC");
    println!("\t--benchmark\t\tstarts a benchmark");
    println!("\t--sanity-check\t\ttests (lightly) if this program works or not.");
    println!("\t--help\t\t\tshows this info");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);
    let enable_server = args.contains(&"--server".to_owned());
    let benchmark = args.contains(&"--benchmark".to_owned());
    let mut benchmark_rounds = 1000;
    if args.len() == 3{
        benchmark_rounds = args[2].parse::<usize>().unwrap();
    }
    let sanity_check = args.contains(&"--sanity-check".to_owned());
    let help = args.contains(&"--help".to_owned()) || !(enable_server || benchmark || sanity_check);

    if help{
        give_help();
    }

    if sanity_check {
        let data = "0.0.0.2.0.0.0.0.2.0.0.0.0.0.0.0+0,3.2;1:0.0.0.2.0.0.0.0.0.0.0.2.2.0.0.0+2,1.2;1:0.0.0.2.0.0.2.0.0.0.0.2.0.0.0.2+2,0.2;1:0.0.2.2.0.0.0.2.0.0.0.2.0.0.0.2+1,3.2;1:0.0.0.4.0.0.0.2.0.0.0.2.0.2.0.2+0,0.2;1:2.0.0.4.0.0.0.2.0.0.0.2.0.0.0.4+2,1.2;1:0.0.2.4.0.0.2.2.0.0.0.2.0.0.0.4+1,1.2;1".to_owned();
        let parsed = parse_data(data);
        println!("Loaded record wit the length of {}.", parsed.history.len());
        println!( "{:#?}", validate_history( parsed ) );
        demo();
    }

    if benchmark {
        println!("Benchmarking with {} rounds:", benchmark_rounds);
        for _i in 0..benchmark_rounds{
            let data = "0.0.0.2.0.0.0.0.2.0.0.0.0.0.0.0+0,3.2;1:0.0.0.2.0.0.0.0.0.0.0.2.2.0.0.0+2,1.2;1:0.0.0.2.0.0.2.0.0.0.0.2.0.0.0.2+2,0.2;1:0.0.2.2.0.0.0.2.0.0.0.2.0.0.0.2+1,3.2;1:0.0.0.4.0.0.0.2.0.0.0.2.0.2.0.2+0,0.2;1:2.0.0.4.0.0.0.2.0.0.0.2.0.0.0.4+2,1.2;1:0.0.2.4.0.0.2.2.0.0.0.2.0.0.0.4+1,1.2;1".to_owned();
            let parsed = parse_data(data);
            validate_history( parsed );
        }
        println!("Done!");
    }

    if enable_server{
        println!("Start the web server:");
        
        let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8080", "https://oisphalla.com", "http://oispahalla.com", "http://oispahalla-dev.netlify.app/", "https://oispahalla-dev.netlify.app/"]);

        let cors = rocket_cors::CorsOptions {
            allowed_origins,
            allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
            allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors().expect("Cors did not set up correctly!");
        rocket::ignite().mount("/HAC", routes![hello, alive]).attach(cors).launch();
    }
}

fn parse_data(data: String) -> Recording {
    let mut history: Vec < ( [[Option<board::tile::Tile>; board::WIDTH]; board::HEIGHT], Direction, Option<board::tile::Tile> ) > = vec![];
    for step in data.split(":"){
        let parts = step.split(";").collect::<Vec<&str>>();
        let bdata = parts[0].split("+").collect::<Vec<&str>>();
        let mut added = "";
        if bdata.len() > 1 {
            added = bdata[1];
        }
        let b = bdata[0];
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
            let y = index / board::WIDTH;
            board[ y ][ x ] = Some ( board::tile::Tile{x: x, y: y, value: val} );
            index += 1;
        }

        let mut added_tile = None;
        if added != ""{
            let added_vals = added.split(".").collect::<Vec<&str>>();
            let added_index = added_vals[0];
            let added_pos = added_index.split(",").collect::<Vec<&str>>();
            let added_x = added_pos[0].parse::<usize>().unwrap();
            let added_y = added_pos[1].parse::<usize>().unwrap();
            let added_value = added_vals[1].parse::<usize>().unwrap();
            added_tile = Some( board::tile::Tile{ y: added_y, x: added_x , value: added_value } );
        }
        
        history.push( (board, direction, added_tile) );
    }
    return Recording{ history };
}

fn validate_history(history: Recording) -> bool{
    let history_len = history.history.len();
    for ind in 0..history_len{
        let i = history.history[ind];
        if ind < (history_len - 1) && ind < (HISTORY_CUTOFF) {
            let board_next = history.history[ind + 1].0;
            let board = i.0;
            let dir = i.1;
            let mut addition = None;
            if ind > 0 || true{
                addition = history.history[ind].2
            }
    
            let predicted = is_move_possible(Board { tiles: board }, dir);
            let mut predicted_board = predicted.0;
            
            match addition{
                Some(add) => {
                    if crate::DEBUG_INFO {println!("[Add] Change {:?} => {:?}", predicted_board[add.y][add.x], add)};
                    predicted_board[add.y][add.x] = Some( add );
                },
                None => {
                    if crate::DEBUG_INFO {println!("No addition at index {}!", ind)};
                }
            }

            if dir == Direction::END {

            }
            else if predicted_board == board_next { // (predicted.1) && 
                
            }
            else{
                println!("Went wrong at index {}: \n{:?}\n{:?}", ind, predicted_board, board_next);
                //println!("{:#?}", i);
                println!("Expected: ");
                print_board(predicted_board);
                println!("Got instead: ");
                print_board(board_next);
                return false;
            }
        }
    }
    return true;
}

fn validate_first_move(history: &Recording) -> bool {
    let history_len = history.history.len();
    if history_len > 0{
        let first_frame = history.history[0].0;
        let first_board = Board{tiles: first_frame};
        if first_board.get_score() < 17 {
            return true;
        }
    }
    println!("First move was not OK!");
    return false;
}

#[macro_use] extern crate rocket;

#[get("/alive")]
fn alive() -> String{
    format!("true")
}

#[get("/validate/<run_json>")]
fn hello(run_json: String) -> String {
    let history = parse_data(run_json);
    println!("Loaded record wit the length of {}.", history.history.len());
    for i in &history.history{
        println!("#\t#\t#\t#\t\n");
        println!("#\t#\t#\t#\t");
        print_board(i.0);
        println!("Direction: {:?}, add tile {:?}", i.1, i.2);
    }
    println!("#\t#\t#\t#\t");
    let result0 = validate_first_move(&history);
    let result1 = validate_history(history);
    let valid = result0 && result1;
    format!("{}\"valid\": {:#?}{}", "{", valid, "}")
}
