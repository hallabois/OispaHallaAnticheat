//! Vilpinestopalvelin peliin 2048 pohjautuvalle [oispahalla:lle](https://oispahalla.com/), pohjautuu kirjastoon [twothousand-forty-eight](https://github.com/hallabois/twothousand-forty-eight).
//! # Lataaminen
//! Tämä repo käyttää git -submoduuleja, jotka pitää ladata ennen projektin koontia.
//! 
//! Voit ladata ne automaattisesti repon kanssa samaan aikaan käyttämällä komentoa ```git clone --recurse-submodules git@github.com:hallabois/OispaHallaAnticheat.git``` tai suorittamalla komennon ```git pull --recurse-submodules``` aiemmin kloonatussa repossa. 
//! 
//! Submoduulit voi päivittää myöhemmin komennolla ```git submodule update --remote --merge```.
//! 
//! # Koonti ja suorittaminen
//! Käyttää rustia, asenna se aluksi: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
//! 
//! Huom: palvelin-ominaisuus vaatii vieläpä rustin nightly-version, josta voit asentaa tuoreen version komennoilla ```rustup update && rustup default nightly```
//! 
//! # HTTPS-tuki
//! aja palvelin komennolla ```ROCKET_TLS={certs="/etc/letsencrypt/live/hac.hallacoin.ml/fullchain.pem",key="/etc/letsencrypt/live/hac.hallacoin.ml/privkey.pem"} ./target/release/g2048engine --server```
//! 
//! # Projektin Tiedostorakenne
//! Lue [ARCHITECTURE.md](ARCHITECTURE.md).
//! 
//! # API:n Käyttö
//! ## /HAC/validate
//! ```
//! https://hac.oispahalla.com:8000/HAC/validate/[leveys]x[korkeus]S[peli]
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
//! e.g. [https://hac.oispahalla.com:8000/HAC/validate/liianpitkätähän](https://hac.oispahalla.com:8000/HAC/validate/4x4S0.0.0.4.0.0.0.0.0.0.2.0.0.0.0.0+1,1.2;1:0.0.0.4.0.2.0.0.0.0.0.2.0.0.0.0+1,1.4;2:0.0.0.0.0.4.0.0.0.0.0.4.0.2.0.2+3,3.2;0:0.4.0.4.0.2.0.2.0.0.0.0.0.0.0.2+3,0.4;3:8.0.0.4.4.0.0.0.0.0.0.0.2.0.0.0+1,3.2;2:0.0.0.0.8.0.0.0.4.0.0.0.2.2.0.4+0,3.2;0:8.2.0.4.4.0.0.0.2.0.0.0.2.0.0.0+2,3.2;2:0.0.0.0.8.0.0.0.4.0.0.0.4.2.2.4+2,1.2;1:0.0.0.0.0.0.2.8.0.0.0.4.0.4.4.4+1,3.2;0:0.4.2.8.0.0.4.8.0.0.0.0.0.2.0.0+2,1.2;2:0.0.0.0.0.0.2.0.0.4.2.0.0.2.4.16+3,1.2;3:0.0.0.0.2.0.0.2.4.2.0.0.2.4.16.0+0,0.2;1:2.0.0.0.0.0.0.4.0.0.4.2.0.2.4.16+1,2.2;0:2.2.8.4.0.0.0.2.0.2.0.16.0.0.0.0+1,0.2;2:0.2.0.0.0.0.0.4.0.0.0.2.2.4.8.16+3,0.2;3:2.0.0.2.4.0.0.0.2.0.0.0.2.4.8.16+2,1.2;1:0.0.0.4.0.0.2.4.0.0.0.2.2.4.8.16+1,1.4;0:2.4.2.8.0.4.8.2.0.0.0.16.0.0.0.0+;e:2.4.2.8.0.4.8.2.0.0.0.16.0.0.0.0+1,1.2;0:2.8.2.8.0.2.8.2.0.0.0.16.0.0.0.0+1,0.2;2:0.2.0.0.0.0.0.8.0.8.2.2.2.2.8.16+3,0.2;3:2.0.0.2.8.0.0.0.8.4.0.0.4.8.16.0+2,1.2;1:0.0.0.4.0.0.2.8.0.0.8.4.0.4.8.16+0,0.2;0:2.4.2.4.0.0.16.8.0.0.0.4.0.0.0.16+0,0.2;2:2.0.0.4.0.0.0.8.0.0.2.4.2.4.16.16+3,0.2;3:2.4.0.2.8.0.0.0.2.4.0.0.2.4.32.0+2,1.2;1:0.2.4.2.0.0.2.8.0.0.2.4.0.2.4.32+0,2.2;0:0.4.4.2.0.0.4.8.2.0.4.4.0.0.0.32+1,1.4;1:0.0.8.2.0.4.4.8.0.0.2.8.0.0.0.32+1,3.2;0:0.4.8.2.0.0.4.16.0.0.2.32.0.2.0.0+1,3.2;3:4.8.2.0.4.16.0.0.2.32.0.0.2.2.0.0+0,3.4;1:0.4.8.2.0.0.4.16.0.0.2.32.4.0.0.4+0,2.2;0:4.4.8.2.0.0.4.16.2.0.2.32.0.0.0.4+2,2.2;3:8.8.2.0.4.16.0.0.4.32.2.0.4.0.0.0+1,3.2;1:0.0.16.2.0.0.4.16.0.4.32.2.0.2.0.4+2,0.2;3:16.2.2.0.4.16.0.0.4.32.2.0.2.4.0.0+3,2.2;2:0.2.0.0.16.16.0.0.8.32.0.2.2.4.4.0+1,0.2;1:0.2.0.2.0.0.0.32.0.8.32.2.0.0.2.8+0,3.2;2:0.0.0.2.0.0.0.32.0.2.32.2.2.8.2.8+;e:0.0.0.2.0.0.0.32.0.2.32.2.2.8.2.8+1,3.2;0:2.2.32.2.0.8.2.32.0.0.0.2.0.2.0.8+3,3.2;3:4.32.2.0.8.2.32.0.2.0.0.0.2.8.0.2+0,3.2;0:4.32.2.2.8.2.32.0.4.8.0.0.2.0.0.0+3,0.2;3:4.32.4.2.8.2.32.0.4.8.0.0.2.0.0.0+1,3.2;1:4.32.4.2.0.8.2.32.0.0.4.8.0.2.0.2+0,3.2;0:4.32.4.2.0.8.2.32.0.2.4.8.2.0.0.2+2,3.2;3:4.32.4.2.8.2.32.0.2.4.8.0.4.0.2.0+0,2.2;1:4.32.4.2.0.8.2.32.2.2.4.8.0.0.4.2+1,0.2;2:0.2.0.2.0.32.4.32.4.8.2.8.2.2.8.2+2,0.4;3:4.0.4.0.32.4.32.0.4.8.2.8.4.8.2.0+0,1.4;1:0.0.0.8.4.32.4.32.4.8.2.8.0.4.8.2+1,0.2;2:0.2.0.8.0.32.4.32.0.8.2.8.8.4.8.2+3,2.2;3:2.8.0.0.32.4.32.0.8.2.8.2.8.4.8.2+1,0.2;1:0.2.2.8.0.32.4.32.8.2.8.2.8.4.8.2+3,0.2;3:4.8.0.2.32.4.32.0.8.2.8.2.8.4.8.2+3,2.2;0:4.8.32.4.32.4.16.2.16.2.0.2.0.4.0.0+0,2.2;1:4.8.32.4.32.4.16.2.2.0.16.4.0.0.0.4+3,3.2;0:4.8.32.4.32.4.32.2.2.0.0.8.0.0.0.2+3,3.2;3:4.8.32.4.32.4.32.2.2.8.0.0.2.0.0.2+1,2.2;1:4.8.32.4.32.4.32.2.0.2.2.8.0.0.0.4+0,3.2;0:4.8.64.4.32.4.2.2.0.2.0.8.2.0.0.4+0,0.2;2:2.0.0.4.4.8.0.2.32.4.64.8.2.2.2.4+3,0.4;3:2.4.0.4.4.8.2.0.32.4.64.8.4.2.4.0+0,1.2;1:0.0.2.8.2.4.8.2.32.4.64.8.0.4.2.4+;e:0.0.2.8.2.4.8.2.32.4.64.8.0.4.2.4+0,2.2;0:2.8.2.8.32.4.8.2.2.0.64.8.0.0.2.4+2,3.2;3:2.8.2.8.32.4.8.2.2.64.8.0.2.4.2.0+0,3.2;1:2.8.2.8.32.4.8.2.0.2.64.8.2.2.4.2+1,3.2;0:2.8.2.8.32.4.8.2.2.4.64.8.0.2.4.2+0,0.2;2:2.0.2.8.2.8.8.2.32.8.64.8.2.2.4.2+3,3.2;3:4.8.0.0.2.16.2.0.32.8.64.8.4.4.2.2+1,0.2;1:0.2.4.8.0.2.16.2.32.8.64.8.0.0.8.4+0,1.2;0:32.4.4.8.2.8.16.2.0.0.64.8.0.0.8.4+0,1.2;2:0.0.4.8.2.0.16.2.32.4.64.8.2.8.8.4+3,3.2;3:4.8.0.0.2.16.2.0.32.4.64.8.2.16.4.2+0,1.2;1:0.0.4.8.2.2.16.2.32.4.64.8.2.16.4.2+3,0.2;3:4.8.0.2.4.16.2.0.32.4.64.8.2.16.4.2+0,1.2;1:0.4.8.2.2.4.16.2.32.4.64.8.2.16.4.2+3,0.2;2:0.0.8.2.2.4.16.4.32.8.64.8.2.16.4.2+1,3.2;0:2.4.8.2.32.8.16.4.2.16.64.8.0.2.4.2+3,3.2;3:2.4.8.2.32.8.16.4.2.16.64.8.2.4.2.2+0,3.2;1:2.4.8.2.32.8.16.4.2.16.64.8.2.2.4.4+0,0.2;2:2.4.8.2.2.8.16.4.32.16.64.8.4.2.4.4+3,3.2;3:2.4.8.2.2.8.16.4.32.16.64.8.4.2.8.2+;e:2.4.8.2.2.8.16.4.32.16.64.8.4.2.8.2+0,0.2;f) tai
//! minimi-pituinen peli
//! [https://hac.oispahalla.com:8000/HAC/validate/4x4S0.0.0.0.0.0.0.0.0.0.0.0.0.0.2.2+2,1.2;1](https://hac.oispahalla.com:8000/HAC/validate/4x4S0.0.0.0.0.0.0.0.0.0.0.0.0.0.2.2+2,1.2;1)
//! 
//! ## /HAC/get_config
//! 
//! ```
//! https://hac.oispahalla.com:8000/HAC/get_config
//! ---------------------------------------------
//! {
//!   "allowed_origins": [                                             sallitut CORS-lähteet,
//!     "http://localhost:8080",                                       eli mistä osoitteista selain saa kutsua api:ta
//!     "https://oispahalla.com/",
//!     "http://oispahalla.com",
//!     "http://oispahalla-dev.netlify.app/",
//!     "https://oispahalla-dev.netlify.app/",
//!     "https://dev--oispahalla-dev.netlify.app",
//!     "https://dev.oispahalla.com/"
//!   ],
//!   "platform": "x86_64-unknown-linux-gnu",                          millä alustalla kyseinen instanssi pyörii
//!   "version": "febc9c91bd18d4be6b4989e3d24898c9bb12ca84",           mikä oli viimeisin git-commit ennen kasaamista
//!   "rust_version": "rustc 1.57.0-nightly (e1e9319d9 2021-10-14)",   millä rustin versiolla projekti on kasattu
//!   "request_count": 0                                               kuinka monta kertaa /validate komentoa on kutsuttu
//! }
//! ```

#![feature(proc_macro_hygiene, decl_macro)]

use std::env;

// We'll let clap handle argument parsing
use clap::{Parser, CommandFactory};

// read_input provides an easy way to handle typed user input
#[cfg(feature = "game")]
use read_input::prelude::*;

#[allow(unused_imports)]
use twothousand_forty_eight::{board, direction, parser, recording, validator};
use parser::parse_data;

#[cfg(feature = "bot")]
mod bot;
#[cfg(feature = "bot")]
use bot::hack;

#[cfg(feature = "game")]
use board::Board;
#[cfg(feature = "game")]
use direction::Direction;

#[cfg(feature = "server")]
#[macro_use] extern crate rocket;
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

    #[cfg(feature = "game")]
    /// starts an interactive game of 2048
    #[clap(short, long)]
    game: bool,

    /// starts a benchmark
    #[clap(short, long, name = "rounds")]
    benchmark: Option<usize>,

    /// parses and prints the game step by step and then validates it.
    #[clap(short, long, name = "game")]
    analyze: Option<String>,

    #[cfg(feature = "bot")]
    /// plays the game by itself.
    #[clap(short, long, name = "max score")]
    hack: Option<usize>
}

#[allow(unreachable_code)]
fn main() {
    let args = Args::parse();

    if let Some(benchmark_rounds) = args.benchmark {
        println!("Benchmarking with {} rounds:", benchmark_rounds);
        for _i in 0..benchmark_rounds{
            let data = "0.0.0.0.0.0.0.0.0.2.0.0.2.0.0.0+3,0.2;1:0.0.0.2.0.0.0.0.0.0.0.2.0.0.0.2+3,3.2;0:0.0.0.4.0.0.0.2.0.0.0.0.0.0.0.2+2,0.2;3:4.0.2.0.2.0.0.0.0.0.0.0.2.0.0.0+0,1.2;1:0.0.4.2.2.0.0.2.0.0.0.0.0.0.0.2+2,0.4;2:0.0.4.0.0.0.0.0.0.0.0.2.2.0.4.4+1,0.2;3:4.2.0.0.0.0.0.0.2.0.0.0.2.8.0.0+1,0.2;1:0.2.4.2.0.0.0.0.0.0.0.2.0.0.2.8+3,1.2;2:0.0.0.0.0.0.0.2.0.0.4.4.0.2.2.8+2,0.2;3:0.0.2.0.2.0.0.0.8.0.0.0.4.8.0.0+0,1.2;1:0.0.0.2.2.0.0.2.0.0.0.8.0.0.4.8+3,1.2;3:2.0.0.0.4.0.0.2.8.0.0.0.4.8.0.0+0,3.2;1:0.0.0.2.0.0.4.2.0.0.0.8.2.0.4.8+1,3.2;0:2.0.8.4.0.0.0.16.0.0.0.0.0.2.0.0+2,2.2;3:2.8.4.0.16.0.0.0.0.0.2.0.2.0.0.0+1,1.2;1:0.2.8.4.0.2.0.16.0.0.0.2.0.0.0.2+2,1.4;3:2.8.4.0.2.16.4.0.2.0.0.0.2.0.0.0+2,2.4;1:0.2.8.4.0.2.16.4.0.0.4.2.0.0.0.2+3,1.2;2:0.0.0.0.0.0.8.2.0.0.16.8.0.4.4.4+1,3.2;0:0.4.8.2.0.0.16.8.0.0.4.4.0.2.0.0+3,1.2;3:4.8.2.0.16.8.0.2.8.0.0.0.2.0.0.0+0,2.2;1:0.4.8.2.0.16.8.2.2.0.0.8.0.0.0.2+0,3.2;0:2.4.16.4.0.16.0.8.0.0.0.2.2.0.0.0+2,2.2;3:2.4.16.4.16.8.0.0.2.0.2.0.2.0.0.0+0,3.2;1:2.4.16.4.0.0.16.8.0.0.0.4.2.0.0.2+1,3.2;1:2.4.16.4.0.0.16.8.0.0.0.4.0.2.0.4+0,2.4;0:2.4.32.4.0.2.0.8.4.0.0.8.0.0.0.0+2,2.2;3:2.4.32.4.2.8.0.0.4.8.2.0.0.0.0.0+0,3.2;0:4.4.32.4.4.16.2.0.0.0.0.0.2.0.0.0+3,2.2;1:0.8.32.4.0.4.16.2.0.0.0.2.0.0.0.2+0,0.2;2:2.0.0.0.0.0.0.4.0.8.32.2.0.4.16.4+1,0.2;3:2.2.0.0.4.0.0.0.8.32.2.0.4.16.4.0+0,2.2;1:0.0.0.4.0.0.0.4.2.8.32.2.0.4.16.4+2,0.2;2:0.0.2.0.0.0.0.8.0.8.32.2.2.4.16.4+0,0.2;1:2.0.0.2.0.0.0.8.0.8.32.2.2.4.16.4+0,1.2;2:0.0.0.2.2.0.0.8.0.8.32.2.4.4.16.4+3,3.2;3:2.0.0.0.2.8.0.0.8.32.2.0.8.16.4.2+0,2.2;1:0.0.0.2.0.0.2.8.2.8.32.2.8.16.4.2+0,3.2;0:2.8.2.2.8.16.32.8.0.0.4.4.2.0.0.0+2,3.2;3:2.8.4.0.8.16.32.8.8.0.0.0.2.0.2.0+1,3.2;1:0.2.8.4.8.16.32.8.0.0.0.8.0.2.0.4+3,2.2;3:2.8.4.0.8.16.32.8.8.0.0.2.2.4.0.0+0,2.2;1:0.2.8.4.8.16.32.8.2.0.8.2.0.0.2.4+2,3.2;3:2.8.4.0.8.16.32.8.2.8.2.0.2.4.2.0+3,2.2;0:2.8.4.8.8.16.32.0.4.8.4.2.0.4.0.0+2,3.2;1:2.8.4.8.0.8.16.32.4.8.4.2.0.0.2.4+0,0.2;2:2.0.4.8.0.0.16.32.2.8.4.2.4.16.2.4+3,0.2;3:2.4.8.2.16.32.0.0.2.8.4.2.4.16.2.4+1,1.2;1:2.4.8.2.0.2.16.32.2.8.4.2.4.16.2.4+0,0.4;2:4.4.8.2.0.2.16.32.4.8.4.2.4.16.2.4+3,0.2;3:8.8.2.2.2.16.32.0.4.8.4.2.4.16.2.4+0,1.2;1:0.0.16.4.2.2.16.32.4.8.4.2.4.16.2.4+0,2.4;0:2.2.32.4.8.8.4.32.4.16.2.2.0.0.0.4+3,2.2;3:4.32.4.0.16.4.32.0.4.16.4.2.4.0.0.0+0,0.2;1:2.4.32.4.0.16.4.32.4.16.4.2.0.0.0.4+2,3.2;0:2.4.32.4.4.32.8.32.0.0.0.2.0.0.2.4+1,2.4;3:2.4.32.4.4.32.8.32.2.4.0.0.2.4.0.0+1,3.2;1:2.4.32.4.4.32.8.32.0.0.2.4.0.2.2.4+0,3.2;0:2.4.32.4.4.32.8.32.0.2.4.8.2.0.0.0+2,3.4;3:2.4.32.4.4.32.8.32.2.4.8.0.2.0.4.0+0,3.2;1:2.4.32.4.4.32.8.32.0.2.4.8.2.0.2.4+2,3.2;3:2.4.32.4.4.32.8.32.2.4.8.0.4.4.2.0+0,3.2;1:2.4.32.4.4.32.8.32.0.2.4.8.2.0.8.2+0,0.2;2:2.0.32.4.2.4.8.32.4.32.4.8.2.2.8.2+0,0.4;2:4.0.32.4.4.4.8.32.4.32.4.8.2.2.8.2+0,3.2;1:0.4.32.4.0.8.8.32.4.32.4.8.2.4.8.2+0,1.2;1:0.4.32.4.2.0.16.32.4.32.4.8.2.4.8.2+0,0.2;2:2.0.32.4.2.4.16.32.4.32.4.8.2.4.8.2+0,0.4;f".to_owned();
            let parsed = parse_data(data);
            validator::validate_history( parsed );
        }
        println!("Done!");
        return;
    }
    #[cfg(feature = "server")]
    if args.server {
        println!("Starting the api server...");
        server::start_server();
        return;
    }
    #[cfg(feature = "game")]
    if args.game {
        game();
        return;
    }
    #[cfg(feature = "bot")]
    if let Some(hack_max_score) = args.hack {
        let hack_stack_size = 400;
        let hack_board_size = 4;
        let hack_mode = 0;
        hack(hack_stack_size, hack_max_score, hack_board_size, hack_mode);
        return;
    }

    // Print help if no arguments are supplied
    let _ = Args::command().print_long_help();
}

#[cfg(feature = "game")]
fn game() {
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