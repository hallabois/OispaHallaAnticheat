
use std::fs::File;
use std::io::Write;

use crate::board;
use crate::board::Board;
use crate::direction::Direction;
use crate::recording::Recording;
use crate::board::create_tiles;
use crate::board::is_move_possible;
use crate::board::print_board;

use read_input::prelude::*;

pub fn hack(max_stack_size: usize, max_score: usize, board_size: usize, scoring_function: usize){
    println!("Starting bot with the parameters {:#?}...", (max_stack_size, max_score, board_size, scoring_function));
    let mut stack: Vec<( [[Option<board::tile::Tile>; board::MAX_WIDTH]; board::MAX_HEIGHT], Direction, Recording, usize )> = vec!(); // Where usize is the score
    let mut visited: Vec<( [[Option<board::tile::Tile>; board::MAX_WIDTH]; board::MAX_HEIGHT], Direction )> = vec!();
    let mut b = create_tiles(board_size, board_size);
    b[0][0] = Some( board::tile::Tile::new(0, 0, 2, false) );
    b[1][1] = Some( board::tile::Tile::new(1, 1, 2, false) );
    stack.push( (b, Direction::UP, Recording{history: vec![],width: board_size,height: board_size}, 0) );
    stack.push( (b, Direction::RIGHT, Recording{history: vec![],width: board_size,height: board_size}, 0) );
    stack.push( (b, Direction::DOWN, Recording{history: vec![],width: board_size,height: board_size}, 0) );
    stack.push( (b, Direction::LEFT, Recording{history: vec![],width: board_size,height: board_size}, 0) );

    //let mut actual_stack_size_addition: usize = 0;
    let mut best_score = usize::MIN;
    let mut best_history = Recording{history: vec![],width: board_size,height: board_size};

    let mut last_result: ([[Option<board::tile::Tile>; board::MAX_WIDTH]; board::MAX_HEIGHT], bool, usize ) = ([[None; board::MAX_WIDTH]; board::MAX_HEIGHT], false, 42069);

    loop{
        //let max_stack_size: usize = 1000;
        if stack.len() > max_stack_size{
            stack.remove(0);
            //actual_stack_size_addition += 1;
        }
        if best_score >= max_score{
            break;
        }
        if stack.len() > 0 {
            let data = stack.pop();
            let d = data.unwrap();
            let mut history = d.2.history;
            if !visited.contains( &(d.0, d.1) ) || false {
                let boarddata = d.0;
                let dir = d.1;
                let current_score = d.3;

                let board = Board{tiles: boarddata, width: d.2.width, height: d.2.height};
                let next = is_move_possible(board, dir);
                let score_addition = next.2;

                //visited.push( (d.0, d.1) );

                if next.1 {
                    visited.push( (boarddata, dir) );

                    let non_occupied = Board{tiles: next.0, width: d.2.width, height: d.2.height}.get_non_occupied_tiles();
                    let mut addition: Option<board::tile::Tile> = None;
                    if non_occupied.len() > 0{
                        let mut t = non_occupied[0];
                        t.value = 4;
                        addition = Some(t);
                    }

                    history.push( (boarddata, dir, addition) );
                    let r = Recording{history, width: d.2.width, height: d.2.height};
                    let mut score = current_score + score_addition;//board.get_total_value(); //stack.len() + actual_stack_size_addition;
                    if scoring_function == 1 {
                        score = board.get_total_value();
                    }
                    if score > best_score {
                        best_score = score;
                        best_history = r.clone();
                    }
                    let mut next_board = next.0;
                    match addition{
                        None => {},
                        Some(t) => {next_board[t.y][t.x] = Some(t)}
                    }
                    stack.push( (next_board, Direction::UP, r.clone(), current_score + score_addition) );
                    stack.push( (next_board, Direction::RIGHT, r.clone(), current_score + score_addition) );
                    stack.push( (next_board, Direction::DOWN, r.clone(), current_score + score_addition) );
                    stack.push( (next_board, Direction::LEFT, r.clone(), current_score + score_addition) );
                }
                else {
                    if stack.len() > 20 {
                        last_result = next;
                    }
                }
            }
            else{
                //println!("Visited already contains {:?}", (d.0, d.1));
            }
        }
        else{
            break;
        }
        //print!("Best score:         Stack size:             \r");
        print!("Best score: {}        Stack size: {}          \r", best_score, stack.len());
    }
    println!("");
    println!("Done!");
    println!("Best score: {}", best_score);
    let index = best_history.history.len() - 1;
    let i = best_history.history[index];
    println!("History at index {}:", index);
    print_board(i.0, board_size, board_size);
    println!("move to direction {:?} and add {:?}", i.1, i.2);
    println!("---------------------------------------------");
    println!("Last failed result:");
    println!("{:?}", last_result);
    println!("---------------------------------------------");
    println!("Gimme the code? (true/false)");
    let analyze = true; //input::<bool>().get();
    if analyze{
        println!("Choose: ");
        println!("\t0: Exit");
        println!("\t1: HAC validation url");
        println!("\t2: HAC history");
        println!("\t3: both");
        let thisorthat = input::<usize>().get();
        if thisorthat == 0{
            println!("Bye!");
        }
        if thisorthat == 1{
            //println!("https://hac.oispahalla.com:8000/HAC/validate/{}", best_history.to_string());
            let mut file = File::create("hac_url.txt").unwrap();
            let result = file.write_all( format!("https://hac.oispahalla.com:8000/HAC/validate/{}x{}S{}", best_history.width, best_history.height, best_history.to_string()).as_bytes() );
            println!("Result of writing to hac_url.txt: {:?}", result);
        }
        if thisorthat == 2{
            println!();
            let b = Board{tiles: best_history.history[best_history.history.len() - 1].0, width: best_history.width, height: best_history.height};

            let mut file1 = File::create("gameState.txt").unwrap();
            let mut file2 = File::create("HAC_history.txt").unwrap();

            let result1 = file1.write_all( b.oispahalla_serialize().as_str().replace("\\", "").replace("SCOREHERE", &best_score.to_string()).as_bytes() );
            println!("Result of writing to gameState.txt: {:?}", result1);

            let result2 = file2.write_all( format!("{:?}", best_history.to_string().split(":").collect::<Vec<&str>>()).as_bytes() );
            println!("Result of writing to HAC_history.txt: {:?}", result2);
        }
        if thisorthat == 3{
            // HAC validation url
            let mut file = File::create("hac_url.txt").unwrap();
            let result = file.write_all( format!("https://hac.oispahalla.com:8000/HAC/validate/{}x{}S{}", best_history.width, best_history.height, best_history.to_string()).as_bytes() );
            println!("Result of writing to hac_url.txt: {:?}", result);
            // Game data
            println!();
            let b = Board{tiles: best_history.history[best_history.history.len() - 1].0, width: best_history.width, height: best_history.height};

            let mut file1 = File::create("gameState.txt").unwrap();
            let mut file2 = File::create("HAC_history.txt").unwrap();

            let result1 = file1.write_all( b.oispahalla_serialize().as_str().replace("\\", "").replace("SCOREHERE", &best_score.to_string()).as_bytes() );
            println!("Result of writing to gameState.txt: {:?}", result1);

            let result2 = file2.write_all( format!("{:?}", best_history.to_string().split(":").collect::<Vec<&str>>()).as_bytes() );
            println!("Result of writing to HAC_history.txt: {:?}", result2);
        }
    }
}
