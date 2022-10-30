//! Rocket-kirjastoon pohjautuva palvelin, joka tarjoaa verkkorajapinnan vilpinestolle

use std::sync::Arc;
use std::sync::Mutex;

use rocket::http::Method;
use rocket::State;
use rocket_cors::AllowedHeaders;
use rocket_cors::AllowedOrigins;

use serde::Serialize;
use rocket_contrib::json::Json;

use crate::DEBUG_INFO;
use crate::parser::parse_data;
use crate::board::print_board;

use crate::validator::validate_history;
use crate::validator::validate_first_move;

/// sallitut CORS-lähteet, eli mistä osoitteista käyttäjän selain saa kutsua api:ta.
/// HUOM: Kukaan ei valvo ominaisuuden toteutusta käyttäjän puolella, tämä ei suojaa palvelinta millään tasolla.
static ALLOWED_ORIGINS_STR: [&'static str; 7] = [
    "http://localhost:8080",
    "https://oispahalla.com/",
    "http://oispahalla.com",
    "http://oispahalla-dev.netlify.app/",
    "https://oispahalla-dev.netlify.app/",
    "https://dev--oispahalla-dev.netlify.app",
    "https://dev.oispahalla.com/"
];

/// Kuinka monta kertaa /validate funktiota on kutsuttu
struct RequestCount(Arc<Mutex<usize>>);

/// Käynnistää Rocket-palvelimen ja tekee tarvitut määritykset
pub fn start_server(){
    let rc = RequestCount(Arc::new(Mutex::new(0)));
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
    rocket::ignite().manage(rc).mount("/HAC", routes).attach(cors).launch();
}

/// Malli jossa /get_config palauttaa dataa
#[derive(Serialize)]
struct ConfigResponse {
    allowed_origins: [&'static str; 7],
    platform: &'static str,
    version: String,
    rust_version: &'static str,
    request_count: usize
}

/// /get_config
#[get("/get_config")]
fn config(rc: State<RequestCount>) -> Json<ConfigResponse>{
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
            request_count: *rc.0.lock().unwrap()
        }
    )
}

/// /alive
/// Kuuluisi aina palauttaa plaintext-merkkijonon "true"
#[get("/alive")]
fn alive() -> String{
    format!("true")
}

/// Malli jossa /validate palauttaa dataa
#[derive(Serialize)]
struct ValidationResponse{
    run_hash: String,
    board_w: usize,
    board_h: usize,
    valid: bool,
    score: usize,
    score_margin: usize,
    breaks: usize,
    length: usize
}

/// /validate/<run_json>
/// Missä <run_json> on pelattua peliä kuvaava merkkijono
#[get("/validate/<run_json>")]
fn hello(run_json: String, rc: State<RequestCount>) -> Json<ValidationResponse> {
    let history = parse_data(run_json);
    let length = history.history.len();
    println!("Loaded record with the length of {}.", length);
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
    let (result1, score, score_margin, breaks) = validate_history(history);
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
        score_margin: score_margin,
        breaks: breaks,
        length: length
    };
    // Add one to the 
    let mut request_count = rc.0.lock().unwrap();
    *request_count += 1;
    // Return the json
    Json(out)
}
