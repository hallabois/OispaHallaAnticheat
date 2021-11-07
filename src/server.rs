// #![feature(proc_macro_hygiene, decl_macro)]

use rocket::http::Method;
use rocket_cors::AllowedHeaders;
use rocket_cors::AllowedOrigins;

use crate::DEBUG_INFO;
use crate::parser::parse_data;
use crate::board::print_board;

use crate::validator::validate_history;
use crate::validator::validate_first_move;

pub fn start_server(){
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8080", "https://oispahalla.com/", "http://oispahalla.com", "http://oispahalla-dev.netlify.app/", "https://oispahalla-dev.netlify.app/", "https://dev--oispahalla-dev.netlify.app"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().expect("Cors did not set up correctly!");
    rocket::ignite().mount("/HAC", routes![alive, hello]).attach(cors).launch();
}

#[get("/alive")]
fn alive() -> String{
    format!("true")
}

#[get("/validate/<run_json>")]
fn hello(run_json: String) -> String {
    let history = parse_data(run_json);
    println!("Loaded record with the length of {}.", history.history.len());
    if DEBUG_INFO{
        let mut index = 0;
        for i in &history.history{
            println!("History at index {}:", index);
            print_board(i.0, history.width, history.height);
            println!("move to direction {:?} and add {:?}", i.1, i.2);
            index += 1;
        }
        println!("#\t#\t#\t#\t");
    }
    let result0 = validate_first_move(&history);
    let (result1, score, breaks) = validate_history(history);
    let valid = result0 && result1;
    println!( "Run score: {}", score );
    println!( "Breaks used: {}", breaks );
    format!("{}\"valid\": {:#?}, \"score\": {}, \"breaks\": {}{}", "{", valid, score, breaks, "}")
}