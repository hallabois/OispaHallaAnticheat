mod response_types;
use response_types::{*};
mod input_types;
use input_types::{*};
mod keys;
use keys::{KEY, CERT};

use poem::{http::Method, middleware::Cors, listener::{TcpListener, RustlsConfig, RustlsCertificate, Listener}, Route, Server, EndpointExt};
use poem_openapi::{OpenApi, OpenApiService, Tags, payload::Json, param::Path};
use tokio::sync::Mutex;
use twothousand_forty_eight::{validator::{validate_first_move, validate_history}, parser::parse_data};

#[derive(Tags)]
enum ApiTags {
    /// Information about the server
    Meta
}

#[derive(Default)]
struct Api {
    request_count: Mutex<usize>,
    error_count: Mutex<usize>
}

#[OpenApi]
impl Api {

    /// Check if the server is online
    #[oai(path = "/alive", method = "get", tag = "ApiTags::Meta")]
    async fn alive(&self) -> AliveResponse {
        AliveResponse::Ok
    }

    /// Get information about the server
    #[oai(path = "/get_config", method = "get", tag = "ApiTags::Meta")]
    async fn get_config(&self) -> GetConfigResponse {
        pub mod built_info {
            // The file has been placed there by the build script.
            include!(concat!(env!("OUT_DIR"), "/built.rs"));
        }
        let version = std::env::var("CARGO_PKG_VERSION").unwrap_or(match built_info::GIT_COMMIT_HASH {
            Some(v)=>String::from(v),
            _ => String::from("unknown version")
        });
        GetConfigResponse::Ok(
            Json(ServerConfig{
                platform: built_info::TARGET.to_string(),
                version: version,
                rust_version: built_info::RUSTC_VERSION.to_string(),
            })
        )
    }

    async fn validate(&self, run: String) -> ValidationResponse {
        let mut request_count = self.request_count.lock().await;
        *request_count += 1;

        let recording = parse_data(run.clone());
        match recording {
            None => {
                println!("Error while parsing run \"{}\"", run);
                let mut error_count = self.error_count.lock().await;
                *error_count += 1;
                ValidationResponse::ParsingFailed
            },
            Some( history ) => {
                let length = history.history.len();
                println!("Loaded record with the length of {}.", length);
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

                ValidationResponse::Ok(Json(ValidationData{
                    run_hash: hash,
                    board_w: w,
                    board_h: h,
                    valid: valid,
                    score: score,
                    score_margin: score_margin,
                    breaks: breaks,
                    length: length
                }))
            },
        }
    }

    /// Validate a played game.
    /// A get request is easy to test, but not that practical with longer runs.
    /// Please also note that Swagger UI apparently breaks the formatting of the input.
    #[oai(path = "/validate/:run", method = "get")]
    async fn validate_get(&self, run: Path<String>) -> ValidationResponse {
        self.validate(run.to_string()).await
    }

    /// Validate a played game.
    /// You may prefer a post request for various reasons.
    #[oai(path = "/validate/:run", method = "post")]
    async fn validate_post(&self, input: Json<ValidateInput>) -> ValidationResponse {
        self.validate(input.run.clone()).await
    }
}

pub async fn start_server() -> Result<(), std::io::Error> {
    pub mod built_info {
        // The file has been placed there by the build script.
        include!(concat!(env!("OUT_DIR"), "/built.rs"));
    }
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let cors = Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST)
        .allow_credentials(false);

    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or(match built_info::GIT_COMMIT_HASH {
        Some(v)=>String::from(v),
        _ => String::from("unknown version")
    });

    let api_service =
        OpenApiService::new(Api::default(), "OispaHallaAnticheat", version).server("https://localhost:8000/HAC");
    let ui = api_service.swagger_ui();

    let listener = TcpListener::bind("0.0.0.0:8000")
        .rustls(RustlsConfig::new().fallback(RustlsCertificate::new().key(KEY).cert(CERT)));

    Server::new(listener)
        .run(Route::new()
        .nest("/", ui)
        .nest("/HAC", api_service)
            .with_if(true, cors)
        )
        .await
}