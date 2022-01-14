// #![feature(proc_macro_hygiene, decl_macro)]

use rocket::http::Method;
use rocket_cors::AllowedHeaders;
use rocket_cors::AllowedOrigins;

use serde::Serialize;
use rocket_contrib::json::Json;

use crate::DEBUG_INFO;
use crate::parser::parse_data;
use crate::board::print_board;

use crate::validator::validate_history;
use crate::validator::validate_first_move;

static ALLOWED_ORIGINS_STR: [&'static str; 7] = [
    "http://localhost:8080",
    "https://oispahalla.com/",
    "http://oispahalla.com",
    "http://oispahalla-dev.netlify.app/",
    "https://oispahalla-dev.netlify.app/",
    "https://dev--oispahalla-dev.netlify.app",
    "https://dev.oispahalla.com/"
];

pub fn start_server(){
    let allowed_origins = AllowedOrigins::some_exact(&ALLOWED_ORIGINS_STR);
    let routes = routes![alive, hello, config];
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().expect("Cors did not set up correctly!");
    rocket::ignite().mount("/HAC", routes).attach(cors).launch();
}

#[derive(Serialize)]
struct ConfigResponse {
    allowed_origins: [&'static str; 7],
    platform: &'static str,
    version: String,
    rust_version: &'static str
}

#[get("/get_config")]
fn config() -> Json<ConfigResponse>{
    pub mod built_info {
        // The file has been placed there by the build script.
        include!(concat!(env!("OUT_DIR"), "/built.rs"));
    }
    let version = std::env::var("HAC_VERSION").unwrap_or(match built_info::GIT_COMMIT_HASH {
        Some(v)=>String::from(v),
        _ => String::from("unknown version")
    });
    Json(
        ConfigResponse{
            allowed_origins: ALLOWED_ORIGINS_STR,
            platform: built_info::TARGET,
            version: version,
            rust_version: built_info::RUSTC_VERSION,
        }
    )
}

#[get("/alive")]
fn alive() -> String{
    format!("true")
}

#[derive(Serialize)]
struct ValidationResponse{
    run_hash: String,
    board_w: usize,
    board_h: usize,
    valid: bool,
    score: usize,
    breaks: usize
}

#[get("/validate/<run_json>")]
fn hello(run_json: String) -> Json<ValidationResponse> {
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
    let hash = history.hash_v1();
    let w = history.width;
    let h = history.height;
    let result0 = validate_first_move(&history);
    let (result1, score, breaks) = validate_history(history);
    let valid = result0 && result1;
    println!( "Run <{}>", hash );
    println!( "\tBoard size: {}x{}", w, h );
    println!( "\tRun score: {}", score );
    println!( "\tBreaks used: {}", breaks );
    println!( "\tValid: {}", valid );
    //let out = format!("{}\"hash\": {}, \"board_w\": {}, \"board_h\": {}, \"valid\": {:#?}, \"score\": {}, \"breaks\": {}{}", "{", hash, w, h, valid, score, breaks, "}");
    let out = ValidationResponse{
        run_hash: hash,
        board_w: w,
        board_h: h,
        valid: valid,
        score: score,
        breaks: breaks
    };
    Json(out)
}
