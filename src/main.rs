//! Vilpinestopalvelin peliin 2048 pohjautuvalle [oispahalla:lle](https://oispahalla.com/), pohjautuu kirjastoon [twothousand-forty-eight](https://github.com/hallabois/twothousand-forty-eight).
//! # Lataaminen
//! Tämä repo käyttää git -submoduuleja, jotka pitää ladata ennen projektin koontia.
//!
//! Voit ladata ne automaattisesti repon kanssa samaan aikaan käyttämällä komentoa ```git clone --recurse-submodules git@github.com:hallabois/OispaHallaAnticheat.git``` tai suorittamalla komennon ```git submodule update --init --recursive``` aiemmin kloonatussa repossa.
//!
//! Submoduulit voi päivittää myöhemmin komennolla ```git submodule update --remote --merge```.
//!
//! # Koonti ja suorittaminen
//! Käyttää rustia, asenna se aluksi: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
//!
//! Huom: palvelin-ominaisuus vaatii vieläpä rustin nightly-version, josta voit asentaa tuoreen version komennoilla ```rustup update && rustup default nightly```
//!
//! # HTTPS-tuki
//! aja palvelin komennolla ```TLS_CERT="[sertifikaatti]" TLS_KEY="[avain]" ./target/release/OispaHallaAnticheat --server```
//!
//! # Projektin Tiedostorakenne
//! Lue [ARCHITECTURE.md](ARCHITECTURE.md).
//!
//! # API:n Käyttö
//! ## /api/validate
//! ```
//! https://hac.oispahalla.com/api/validate/[leveys]x[korkeus]S[peli]
//! ---------------------------------------------
//! {
//!     "run_hash":"724...",   Pelin uniikki tunniste. Älä käytä, jos peli ei ole validi
//!     "board_w":4,           Tulkittu pelilaudan leveys
//!     "board_h":4,           Tulkittu pelilaudan korkeus
//!     "valid":true,          Onko syöte validi peli
//!     "score":584,           Mitä pelin pisteiden tulisi olla*
//!     "score_margin":4,      Viimeisen vuoron aiheuttama mahdollinen pisteiden liikkumisvara*
//!     "breaks":0,            Kuinka monta kurinpalautusta pelin aikana käytettiin*
//!     "length":85            Pelin pituus siirroissa
//! }
//! *palauttaa nollan, jos peli ei ole validi
//! ```
//! e.g. [https://hac.oispahalla.com/api/validate/liianpitkätähän](https://hac.oispahalla.com/api/validate/4x4S0.0.0.4.0.0.0.0.0.0.2.0.0.0.0.0+1,1.2;1:0.0.0.4.0.2.0.0.0.0.0.2.0.0.0.0+1,1.4;2:0.0.0.0.0.4.0.0.0.0.0.4.0.2.0.2+3,3.2;0:0.4.0.4.0.2.0.2.0.0.0.0.0.0.0.2+3,0.4;3:8.0.0.4.4.0.0.0.0.0.0.0.2.0.0.0+1,3.2;2:0.0.0.0.8.0.0.0.4.0.0.0.2.2.0.4+0,3.2;0:8.2.0.4.4.0.0.0.2.0.0.0.2.0.0.0+2,3.2;2:0.0.0.0.8.0.0.0.4.0.0.0.4.2.2.4+2,1.2;1:0.0.0.0.0.0.2.8.0.0.0.4.0.4.4.4+1,3.2;0:0.4.2.8.0.0.4.8.0.0.0.0.0.2.0.0+2,1.2;2:0.0.0.0.0.0.2.0.0.4.2.0.0.2.4.16+3,1.2;3:0.0.0.0.2.0.0.2.4.2.0.0.2.4.16.0+0,0.2;1:2.0.0.0.0.0.0.4.0.0.4.2.0.2.4.16+1,2.2;0:2.2.8.4.0.0.0.2.0.2.0.16.0.0.0.0+1,0.2;2:0.2.0.0.0.0.0.4.0.0.0.2.2.4.8.16+3,0.2;3:2.0.0.2.4.0.0.0.2.0.0.0.2.4.8.16+2,1.2;1:0.0.0.4.0.0.2.4.0.0.0.2.2.4.8.16+1,1.4;0:2.4.2.8.0.4.8.2.0.0.0.16.0.0.0.0+;e:2.4.2.8.0.4.8.2.0.0.0.16.0.0.0.0+1,1.2;0:2.8.2.8.0.2.8.2.0.0.0.16.0.0.0.0+1,0.2;2:0.2.0.0.0.0.0.8.0.8.2.2.2.2.8.16+3,0.2;3:2.0.0.2.8.0.0.0.8.4.0.0.4.8.16.0+2,1.2;1:0.0.0.4.0.0.2.8.0.0.8.4.0.4.8.16+0,0.2;0:2.4.2.4.0.0.16.8.0.0.0.4.0.0.0.16+0,0.2;2:2.0.0.4.0.0.0.8.0.0.2.4.2.4.16.16+3,0.2;3:2.4.0.2.8.0.0.0.2.4.0.0.2.4.32.0+2,1.2;1:0.2.4.2.0.0.2.8.0.0.2.4.0.2.4.32+0,2.2;0:0.4.4.2.0.0.4.8.2.0.4.4.0.0.0.32+1,1.4;1:0.0.8.2.0.4.4.8.0.0.2.8.0.0.0.32+1,3.2;0:0.4.8.2.0.0.4.16.0.0.2.32.0.2.0.0+1,3.2;3:4.8.2.0.4.16.0.0.2.32.0.0.2.2.0.0+0,3.4;1:0.4.8.2.0.0.4.16.0.0.2.32.4.0.0.4+0,2.2;0:4.4.8.2.0.0.4.16.2.0.2.32.0.0.0.4+2,2.2;3:8.8.2.0.4.16.0.0.4.32.2.0.4.0.0.0+1,3.2;1:0.0.16.2.0.0.4.16.0.4.32.2.0.2.0.4+2,0.2;3:16.2.2.0.4.16.0.0.4.32.2.0.2.4.0.0+3,2.2;2:0.2.0.0.16.16.0.0.8.32.0.2.2.4.4.0+1,0.2;1:0.2.0.2.0.0.0.32.0.8.32.2.0.0.2.8+0,3.2;2:0.0.0.2.0.0.0.32.0.2.32.2.2.8.2.8+;e:0.0.0.2.0.0.0.32.0.2.32.2.2.8.2.8+1,3.2;0:2.2.32.2.0.8.2.32.0.0.0.2.0.2.0.8+3,3.2;3:4.32.2.0.8.2.32.0.2.0.0.0.2.8.0.2+0,3.2;0:4.32.2.2.8.2.32.0.4.8.0.0.2.0.0.0+3,0.2;3:4.32.4.2.8.2.32.0.4.8.0.0.2.0.0.0+1,3.2;1:4.32.4.2.0.8.2.32.0.0.4.8.0.2.0.2+0,3.2;0:4.32.4.2.0.8.2.32.0.2.4.8.2.0.0.2+2,3.2;3:4.32.4.2.8.2.32.0.2.4.8.0.4.0.2.0+0,2.2;1:4.32.4.2.0.8.2.32.2.2.4.8.0.0.4.2+1,0.2;2:0.2.0.2.0.32.4.32.4.8.2.8.2.2.8.2+2,0.4;3:4.0.4.0.32.4.32.0.4.8.2.8.4.8.2.0+0,1.4;1:0.0.0.8.4.32.4.32.4.8.2.8.0.4.8.2+1,0.2;2:0.2.0.8.0.32.4.32.0.8.2.8.8.4.8.2+3,2.2;3:2.8.0.0.32.4.32.0.8.2.8.2.8.4.8.2+1,0.2;1:0.2.2.8.0.32.4.32.8.2.8.2.8.4.8.2+3,0.2;3:4.8.0.2.32.4.32.0.8.2.8.2.8.4.8.2+3,2.2;0:4.8.32.4.32.4.16.2.16.2.0.2.0.4.0.0+0,2.2;1:4.8.32.4.32.4.16.2.2.0.16.4.0.0.0.4+3,3.2;0:4.8.32.4.32.4.32.2.2.0.0.8.0.0.0.2+3,3.2;3:4.8.32.4.32.4.32.2.2.8.0.0.2.0.0.2+1,2.2;1:4.8.32.4.32.4.32.2.0.2.2.8.0.0.0.4+0,3.2;0:4.8.64.4.32.4.2.2.0.2.0.8.2.0.0.4+0,0.2;2:2.0.0.4.4.8.0.2.32.4.64.8.2.2.2.4+3,0.4;3:2.4.0.4.4.8.2.0.32.4.64.8.4.2.4.0+0,1.2;1:0.0.2.8.2.4.8.2.32.4.64.8.0.4.2.4+;e:0.0.2.8.2.4.8.2.32.4.64.8.0.4.2.4+0,2.2;0:2.8.2.8.32.4.8.2.2.0.64.8.0.0.2.4+2,3.2;3:2.8.2.8.32.4.8.2.2.64.8.0.2.4.2.0+0,3.2;1:2.8.2.8.32.4.8.2.0.2.64.8.2.2.4.2+1,3.2;0:2.8.2.8.32.4.8.2.2.4.64.8.0.2.4.2+0,0.2;2:2.0.2.8.2.8.8.2.32.8.64.8.2.2.4.2+3,3.2;3:4.8.0.0.2.16.2.0.32.8.64.8.4.4.2.2+1,0.2;1:0.2.4.8.0.2.16.2.32.8.64.8.0.0.8.4+0,1.2;0:32.4.4.8.2.8.16.2.0.0.64.8.0.0.8.4+0,1.2;2:0.0.4.8.2.0.16.2.32.4.64.8.2.8.8.4+3,3.2;3:4.8.0.0.2.16.2.0.32.4.64.8.2.16.4.2+0,1.2;1:0.0.4.8.2.2.16.2.32.4.64.8.2.16.4.2+3,0.2;3:4.8.0.2.4.16.2.0.32.4.64.8.2.16.4.2+0,1.2;1:0.4.8.2.2.4.16.2.32.4.64.8.2.16.4.2+3,0.2;2:0.0.8.2.2.4.16.4.32.8.64.8.2.16.4.2+1,3.2;0:2.4.8.2.32.8.16.4.2.16.64.8.0.2.4.2+3,3.2;3:2.4.8.2.32.8.16.4.2.16.64.8.2.4.2.2+0,3.2;1:2.4.8.2.32.8.16.4.2.16.64.8.2.2.4.4+0,0.2;2:2.4.8.2.2.8.16.4.32.16.64.8.4.2.4.4+3,3.2;3:2.4.8.2.2.8.16.4.32.16.64.8.4.2.8.2+;e:2.4.8.2.2.8.16.4.32.16.64.8.4.2.8.2+0,0.2;f) tai
//! minimi-pituinen peli
//! [https://hac.oispahalla.com/api/validate/4x4S0.0.0.0.0.0.0.0.0.0.0.0.0.0.2.2+2,1.2;1](https://hac.oispahalla.com/api/validate/4x4S0.0.0.0.0.0.0.0.0.0.0.0.0.0.2.2+2,1.2;1)
//!
//! ## /api/get_config
//!
//! ```
//! https://hac.oispahalla.com/api/get_config
//! ---------------------------------------------
//! {
//!   "platform": "x86_64-unknown-linux-gnu",                          millä alustalla kyseinen instanssi pyörii
//!   "version": "febc9c91bd18d4be6b4989e3d24898c9bb12ca84",           mikä oli viimeisin julkaistu versio tai git-commit ennen kasaamista
//!   "rust_version": "rustc 1.57.0-nightly (e1e9319d9 2021-10-14)",   millä rustin versiolla projekti on kasattu
//!   "request_count": 0                                               kuinka monta kertaa /validate komentoa on kutsuttu
//! }
//! ```

use std::fs;

// We'll let clap handle argument parsing
use clap::{CommandFactory, Parser};

// read_input provides an easy way to handle typed user input
#[cfg(feature = "game")]
use read_input::prelude::*;

#[cfg(feature = "game")]
use board::Board;
#[cfg(feature = "game")]
use direction::Direction;
use twothousand_forty_eight::unified::validate;

#[cfg(feature = "server")]
mod server;

/// Näyttää lisää virheenkorjaustietoja tarvittaessa
#[allow(dead_code)]
const DEBUG_INFO: bool = false;

/// An anticheat solution for the 2048 fork OispaHalla
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[cfg(feature = "server")]
    /// starts the api server
    #[clap(short, long)]
    server: bool,
    #[cfg(feature = "server")]
    /// starts the api server without tls
    #[clap(long)]
    server_http: bool,

    #[cfg(feature = "game")]
    /// starts an interactive game of 2048
    #[clap(short, long)]
    game: bool,

    /// parses and prints the game step by step and then validates it.
    #[clap(short, long, name = "game")]
    analyze: Option<String>,

    /// parses and prints the game step by step and then validates it.
    #[clap(long, name = "file")]
    analyze_file: Option<String>,
}

#[cfg_attr(feature = "server", tokio::main)]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    if let Some(uri) = args.analyze_file {
        println!("Reading file...");
        let contents = fs::read_to_string(uri)?;
        let s = validate(&contents).unwrap();
        println!("Contents of the file are valid! Data:\n{:#?}", s);
    }

    if let Some(contents) = args.analyze {
        println!("Parsing string...");
        let s = validate(&contents).unwrap();
        println!("Contents of the file are valid! Data:\n{:#?}", s);
    }

    #[cfg(feature = "server")]
    if args.server {
        println!("Starting the api server...");
        return server::start_server(true).await;
    }
    #[cfg(feature = "server")]
    if args.server_http {
        println!("Starting the api server without tls...");
        return server::start_server(false).await;
    }
    #[cfg(feature = "game")]
    if args.game {
        game();
        return Ok(());
    }

    // Print help if no arguments are supplied
    let _ = Args::command().print_long_help();
    Ok(())
}

#[cfg(feature = "game")]
fn game() {
    let mut board: Board = Board {
        tiles: board::create_tiles(4, 4),
        width: 4,
        height: 4,
    };
    board.set_tile(0, 0, 2);
    board.set_tile(1, 0, 2);
    board::print_board(board.tiles, 4, 4);
    println!("input \"9\" to exit.");
    loop {
        let inp = input::<usize>().get();
        if inp == 9 {
            break;
        }
        let dir = [
            Direction::UP,
            Direction::RIGHT,
            Direction::DOWN,
            Direction::LEFT,
            Direction::END,
        ][inp];
        let next = board::check_move(board, dir);
        if next.possible {
            board.tiles = next.tiles;

            twothousand_forty_eight::add_random_to_board(&mut board);

            println!("Next state: ");
            board::print_board(board.tiles, 4, 4);
        } else {
            board.tiles = next.tiles;
            println!("Move not possible!")
        }
    }
    return;
}
