#![feature(proc_macro_hygiene, decl_macro)]

use std::env;
use read_input::prelude::*;

use twothousand_forty_eight::{board, direction, parser, recording, validator};
use board::Board;
use direction::Direction;
use parser::parse_data;

mod bot;
use bot::hack;

#[macro_use] extern crate rocket;
mod server;

const DEBUG_INFO: bool = false;

const NAME: &str = "G2048";
const NAME_SERVER: &str = "HAC";
const VERSION: &str = "0.1.0";

fn give_help() {
    println!("{} {}", NAME, VERSION);
    println!("\t--server{}starts a webserver for {}", "\t".repeat(4), NAME_SERVER);
    println!("\t--game{}starts an interactive game of 2048", "\t".repeat(5));
    println!("\t--benchmark [rounds]{}starts a benchmark", "\t".repeat(3));
    println!("\t--analyze [game]{}prints the game step by step and validates it.", "\t".repeat(3));
    println!("\t--hack [max score] [board size]{}plays the game by itself.", "\t".repeat(2));
    println!("\t--sanity-check{}tests (lightly) if this program works or not.", "\t".repeat(4));
    println!("\t--help{}shows this info", "\t".repeat(5));
}

fn demo(){
    println!("Basic Sanity Check:");
    println!("-------------");
    let mut board: Board = Board{
        tiles: board::create_tiles(4, 4),
        width: 4,
        height: 4
    };
    board.set_tile(0, 0, 2);
    board.set_tile(1, 0, 2);
    board.set_tile(3, 1, 4);
    //println!("{:?}", board.tiles);
    board::print_board(board.tiles, 4, 4);
    for i in [ Direction::LEFT, Direction::DOWN ].iter() {
        let next = board::is_move_possible(board, *i);
        if next.1 {
            println!("Next state: ");
            board::print_board(next.0, 4, 4);
        }
        else {
            println!("Move not possible!")
        }
        board.tiles = next.0;
    }
    println!("-------------");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);
    let enable_server = args.contains(&"--server".to_owned());
    let game = args.contains(&"--game".to_owned());
    let benchmark = args.contains(&"--benchmark".to_owned());
    let mut benchmark_rounds = 1000;
    let analyze = args.contains(&"--analyze".to_owned());
    let mut analyze_data = "";
    let do_hack = args.contains(&"--hack".to_owned());
    let mut hack_stack_size: usize = usize::MAX;
    let mut hack_board_size: usize = 4;
    let mut hack_max_score: usize = 10000;
    if do_hack && args.len() > 2{
        hack_max_score = args[2].parse::<usize>().unwrap();
    }
    if do_hack && args.len() > 3{
        hack_board_size = args[3].parse::<usize>().unwrap();
    }
    if do_hack && args.len() > 4{
        hack_stack_size = args[4].parse::<usize>().unwrap();
    }
    if benchmark && args.len() == 3{
        benchmark_rounds = args[2].parse::<usize>().unwrap();
    }
    if analyze && args.len() == 3{
        analyze_data = args[2].as_str();
    }
    let sanity_check = args.contains(&"--sanity-check".to_owned());
    let help = args.contains(&"--help".to_owned()) || !(enable_server || benchmark || sanity_check || game || analyze || do_hack);

    if help{
        give_help();
        return;
    }

    if sanity_check {
        demo();
        println!("An actual scenario:");
        let data = "0.0.0.0.0.0.0.0.0.0.0.0.2.0.2.0+3,0.2;3:0.0.0.2.0.0.0.0.0.0.0.0.4.0.0.0+2,1.2;1:0.0.0.2.0.0.2.0.0.0.0.0.0.0.0.4+2,1.2;3:2.0.0.0.2.0.2.0.0.0.0.0.4.0.0.0+0,1.2;1:0.0.0.2.2.0.0.4.0.0.0.0.0.0.0.4+2,2.2;3:2.0.0.0.2.4.0.0.0.0.2.0.4.0.0.0+2,3.2;1:0.0.0.2.0.0.2.4.0.0.0.2.0.0.2.4+2,2.2;3:2.0.0.0.2.4.0.0.2.0.2.0.2.4.0.0+1,0.2;1:0.2.0.2.0.0.2.4.0.0.0.4.0.0.2.4+1,2.2;3:4.0.0.0.2.4.0.0.4.2.0.0.2.4.0.0+1,2.2;1:0.0.0.4.0.0.2.4.0.2.4.2.0.0.2.4+2,3.2;3:4.0.0.0.2.4.0.0.2.4.2.0.2.4.2.0+0,1.2;1:0.0.0.4.2.0.2.4.0.2.4.2.0.2.4.2+1,0.2;3:4.2.0.0.4.4.0.0.2.4.2.0.2.4.2.0+0,0.2;1:2.0.4.2.0.0.0.8.0.2.4.2.0.2.4.2+3,3.2;3:2.4.2.0.8.0.0.0.2.4.2.0.2.4.2.2+0,2.2;1:0.2.4.2.0.0.0.8.2.2.4.2.0.2.4.4+2,3.2;3:2.4.2.0.8.0.0.0.4.4.2.0.2.8.2.0+1,1.2;1:0.2.4.2.0.2.0.8.0.0.8.2.0.2.8.2+3,1.2;3:2.4.2.0.2.8.0.2.8.2.0.0.2.8.2.0+0,3.2;1:0.2.4.2.0.2.8.2.0.0.8.2.2.2.8.2+3,3.2;3:2.4.2.0.2.8.2.0.8.2.0.0.4.8.2.2+0,2.2;1:0.2.4.2.0.2.8.2.2.0.8.2.0.4.8.4+3,3.2;3:2.4.2.0.2.8.2.0.2.8.2.0.4.8.4.2+0,2.2;1:0.2.4.2.0.2.8.2.2.2.8.2.4.8.4.2+3,2.2;3:2.4.2.0.2.8.2.0.4.8.2.2.4.8.4.2+0,1.2;1:0.2.4.2.2.2.8.2.0.4.8.4.4.8.4.2+3,1.2;3:2.4.2.0.4.8.2.2.4.8.4.0.4.8.4.2+0,2.2;1:0.2.4.2.0.4.8.4.2.4.8.4.4.8.4.2+3,0.2;3:2.4.2.2.4.8.4.0.2.4.8.4.4.8.4.2+0,0.2;1:2.2.4.4.0.4.8.4.2.4.8.4.4.8.4.2+2,0.2;3:4.8.2.0.4.8.4.0.2.4.8.4.4.8.4.2+0,1.2;1".to_owned();
        let parsed = parse_data(data);
        println!("Loaded record wit the length of {}.", parsed.history.len());
        println!( "{:#?}", validator::validate_history( parsed ) );
        return;
    }
    
    if analyze {
        let parsed = parse_data(analyze_data.to_owned());
        println!("Loaded record wit the length of {}.", parsed.history.len());
        println!("History:");
        let mut index: usize = 0;
        for i in &parsed.history{
            println!("History at index {}:", index);
            board::print_board(i.0, parsed.width, parsed.height);
            println!("move to direction {:?} and add {:?}", i.1, i.2);
            index += 1;
        }
        println!( "{:#?}", validator::validate_history( parsed ) );
        let parsed2 = parse_data(analyze_data.to_owned());
        println!( "Run score: {}", validator::get_run_score(&parsed2) );
        return;
    }

    if do_hack {
        hack(hack_stack_size, hack_max_score, hack_board_size);
        return;
    }

    if benchmark {
        println!("Benchmarking with {} rounds:", benchmark_rounds);
        for _i in 0..benchmark_rounds{
            let data = "0.0.0.0.0.0.0.0.0.2.0.0.2.0.0.0+3,0.2;1:0.0.0.2.0.0.0.0.0.0.0.2.0.0.0.2+3,3.2;0:0.0.0.4.0.0.0.2.0.0.0.0.0.0.0.2+2,0.2;3:4.0.2.0.2.0.0.0.0.0.0.0.2.0.0.0+0,1.2;1:0.0.4.2.2.0.0.2.0.0.0.0.0.0.0.2+2,0.4;2:0.0.4.0.0.0.0.0.0.0.0.2.2.0.4.4+1,0.2;3:4.2.0.0.0.0.0.0.2.0.0.0.2.8.0.0+1,0.2;1:0.2.4.2.0.0.0.0.0.0.0.2.0.0.2.8+3,1.2;2:0.0.0.0.0.0.0.2.0.0.4.4.0.2.2.8+2,0.2;3:0.0.2.0.2.0.0.0.8.0.0.0.4.8.0.0+0,1.2;1:0.0.0.2.2.0.0.2.0.0.0.8.0.0.4.8+3,1.2;3:2.0.0.0.4.0.0.2.8.0.0.0.4.8.0.0+0,3.2;1:0.0.0.2.0.0.4.2.0.0.0.8.2.0.4.8+1,3.2;0:2.0.8.4.0.0.0.16.0.0.0.0.0.2.0.0+2,2.2;3:2.8.4.0.16.0.0.0.0.0.2.0.2.0.0.0+1,1.2;1:0.2.8.4.0.2.0.16.0.0.0.2.0.0.0.2+2,1.4;3:2.8.4.0.2.16.4.0.2.0.0.0.2.0.0.0+2,2.4;1:0.2.8.4.0.2.16.4.0.0.4.2.0.0.0.2+3,1.2;2:0.0.0.0.0.0.8.2.0.0.16.8.0.4.4.4+1,3.2;0:0.4.8.2.0.0.16.8.0.0.4.4.0.2.0.0+3,1.2;3:4.8.2.0.16.8.0.2.8.0.0.0.2.0.0.0+0,2.2;1:0.4.8.2.0.16.8.2.2.0.0.8.0.0.0.2+0,3.2;0:2.4.16.4.0.16.0.8.0.0.0.2.2.0.0.0+2,2.2;3:2.4.16.4.16.8.0.0.2.0.2.0.2.0.0.0+0,3.2;1:2.4.16.4.0.0.16.8.0.0.0.4.2.0.0.2+1,3.2;1:2.4.16.4.0.0.16.8.0.0.0.4.0.2.0.4+0,2.4;0:2.4.32.4.0.2.0.8.4.0.0.8.0.0.0.0+2,2.2;3:2.4.32.4.2.8.0.0.4.8.2.0.0.0.0.0+0,3.2;0:4.4.32.4.4.16.2.0.0.0.0.0.2.0.0.0+3,2.2;1:0.8.32.4.0.4.16.2.0.0.0.2.0.0.0.2+0,0.2;2:2.0.0.0.0.0.0.4.0.8.32.2.0.4.16.4+1,0.2;3:2.2.0.0.4.0.0.0.8.32.2.0.4.16.4.0+0,2.2;1:0.0.0.4.0.0.0.4.2.8.32.2.0.4.16.4+2,0.2;2:0.0.2.0.0.0.0.8.0.8.32.2.2.4.16.4+0,0.2;1:2.0.0.2.0.0.0.8.0.8.32.2.2.4.16.4+0,1.2;2:0.0.0.2.2.0.0.8.0.8.32.2.4.4.16.4+3,3.2;3:2.0.0.0.2.8.0.0.8.32.2.0.8.16.4.2+0,2.2;1:0.0.0.2.0.0.2.8.2.8.32.2.8.16.4.2+0,3.2;0:2.8.2.2.8.16.32.8.0.0.4.4.2.0.0.0+2,3.2;3:2.8.4.0.8.16.32.8.8.0.0.0.2.0.2.0+1,3.2;1:0.2.8.4.8.16.32.8.0.0.0.8.0.2.0.4+3,2.2;3:2.8.4.0.8.16.32.8.8.0.0.2.2.4.0.0+0,2.2;1:0.2.8.4.8.16.32.8.2.0.8.2.0.0.2.4+2,3.2;3:2.8.4.0.8.16.32.8.2.8.2.0.2.4.2.0+3,2.2;0:2.8.4.8.8.16.32.0.4.8.4.2.0.4.0.0+2,3.2;1:2.8.4.8.0.8.16.32.4.8.4.2.0.0.2.4+0,0.2;2:2.0.4.8.0.0.16.32.2.8.4.2.4.16.2.4+3,0.2;3:2.4.8.2.16.32.0.0.2.8.4.2.4.16.2.4+1,1.2;1:2.4.8.2.0.2.16.32.2.8.4.2.4.16.2.4+0,0.4;2:4.4.8.2.0.2.16.32.4.8.4.2.4.16.2.4+3,0.2;3:8.8.2.2.2.16.32.0.4.8.4.2.4.16.2.4+0,1.2;1:0.0.16.4.2.2.16.32.4.8.4.2.4.16.2.4+0,2.4;0:2.2.32.4.8.8.4.32.4.16.2.2.0.0.0.4+3,2.2;3:4.32.4.0.16.4.32.0.4.16.4.2.4.0.0.0+0,0.2;1:2.4.32.4.0.16.4.32.4.16.4.2.0.0.0.4+2,3.2;0:2.4.32.4.4.32.8.32.0.0.0.2.0.0.2.4+1,2.4;3:2.4.32.4.4.32.8.32.2.4.0.0.2.4.0.0+1,3.2;1:2.4.32.4.4.32.8.32.0.0.2.4.0.2.2.4+0,3.2;0:2.4.32.4.4.32.8.32.0.2.4.8.2.0.0.0+2,3.4;3:2.4.32.4.4.32.8.32.2.4.8.0.2.0.4.0+0,3.2;1:2.4.32.4.4.32.8.32.0.2.4.8.2.0.2.4+2,3.2;3:2.4.32.4.4.32.8.32.2.4.8.0.4.4.2.0+0,3.2;1:2.4.32.4.4.32.8.32.0.2.4.8.2.0.8.2+0,0.2;2:2.0.32.4.2.4.8.32.4.32.4.8.2.2.8.2+0,0.4;2:4.0.32.4.4.4.8.32.4.32.4.8.2.2.8.2+0,3.2;1:0.4.32.4.0.8.8.32.4.32.4.8.2.4.8.2+0,1.2;1:0.4.32.4.2.0.16.32.4.32.4.8.2.4.8.2+0,0.2;2:2.0.32.4.2.4.16.32.4.32.4.8.2.4.8.2+0,0.4;f".to_owned();
            let parsed = parse_data(data);
            validator::validate_history( parsed );
        }
        println!("Done!");
        return;
    }
    if game{
        let mut board: Board = Board{
            tiles: board::create_tiles(4, 4),
            width: 4,
            height: 4
        };
        board.set_tile(0, 0, 2);
        board.set_tile(1, 0, 2);
        board.set_tile(3, 1, 2);
        board::print_board(board.tiles, 4, 4);
        println!("input \"9\" to exit.");
        loop {
            let inp = input::<usize>().get();
            if inp == 9{
                break;
            }
            let dir = [Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT, Direction::END][inp];
            let next = board::is_move_possible(board, dir);
            if next.1 {
                println!("Next state: ");
                board::print_board(next.0, 4, 4);
            }
            else {
                println!("Move not possible!")
            }
            board.tiles = next.0;
        }
        return;
    }
    if enable_server{
        println!("Start the web server:");
        server::start_server();
    }
}
