mod response_types;
use response_types::*;
mod input_types;
use input_types::*;
mod keys;
use keys::{get_cert, get_key};

use poem::{
    http::Method,
    listener::{Listener, RustlsCertificate, RustlsConfig, TcpListener},
    middleware::Cors,
    EndpointExt, Route, Server,
};
use poem_openapi::{param::Path, payload::Json, OpenApi, OpenApiService, Tags};
use tokio::sync::Mutex;
use twothousand_forty_eight::unified::{hash::Hashable, parse, validate};

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Tags)]
enum ApiTags {
    /// Information about the server
    Meta,
}

#[derive(Default)]
struct Api {
    request_count: Mutex<usize>,
    invalid_count: Mutex<usize>,
    error_count: Mutex<usize>,
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
        let version =
            std::env::var("CARGO_PKG_VERSION").unwrap_or(match built_info::GIT_COMMIT_HASH {
                Some(v) => String::from(v),
                _ => String::from("unknown version"),
            });
        GetConfigResponse::Ok(Json(ServerConfig {
            platform: built_info::TARGET.to_string(),
            version: version,
            rust_version: built_info::RUSTC_VERSION.to_string(),
        }))
    }

    /// Get statistics about the server
    #[oai(path = "/stats", method = "get", tag = "ApiTags::Meta")]
    async fn stats(&self) -> StatsResponse {
        let request_count = *self.request_count.lock().await;
        let error_count = *self.error_count.lock().await;
        let invalid_count = *self.invalid_count.lock().await;
        StatsResponse::Ok(Json(Stats {
            request_count,
            invalid_count,
            error_count,
        }))
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
    #[oai(path = "/validate", method = "post")]
    async fn validate_post(&self, input: Json<ValidationInput>) -> ValidationResponse {
        self.validate(input.run.clone()).await
    }

    async fn validate(&self, run: String) -> ValidationResponse {
        let mut request_count = self.request_count.lock().await;
        *request_count += 1;

        let recording = parse(&run);
        match recording {
            Err(e) => {
                println!("Error while parsing run \"{}\"", run);
                println!("Error: {:?}", e);
                let mut error_count = self.error_count.lock().await;
                *error_count += 1;
                ValidationResponse::ParsingFailed(poem_openapi::payload::PlainText(e.to_string()))
            }
            Ok(result) => {
                let (length, hash, w, h) = match result {
                    twothousand_forty_eight::unified::ParseResult::V1(v1) => {
                        (v1.history.len(), v1.game_hash(), v1.width, v1.height)
                    }
                    twothousand_forty_eight::unified::ParseResult::V2(v2) => {
                        (v2.moves.len(), v2.game_hash(), v2.width, v2.height)
                    }
                };
                println!("Loaded record with the length of {}.", length);
                let validation_result = validate(&run);
                let valid = validation_result.is_ok();
                println!("Run <{}>", hash);
                println!("\tBoard size: {}x{}", w, h);
                println!("\tValid: {}", valid);
                match validation_result {
                    Err(e) => {
                        let mut invalid_count = self.invalid_count.lock().await;
                        *invalid_count += 1;
                        ValidationResponse::InvalidRun(poem_openapi::payload::PlainText(format!(
                            "{:?}",
                            e
                        )))
                    }
                    Ok(data) => {
                        println!("\tRun score: {}", data.score);
                        println!("\tBreaks used: {}", data.breaks);
                        ValidationResponse::Ok(Json(ValidationOK {
                            run_hash: hash,
                            board_w: w,
                            board_h: h,
                            score: data.score,
                            score_end: data.score_end,
                            score_margin: data.score_margin,
                            breaks: data.breaks,
                            break_positions: data
                                .break_positions
                                .iter()
                                .filter(|p| p.is_some())
                                .map(|p| p.unwrap())
                                .collect(),
                            length,
                        }))
                    }
                }
            }
        }
    }
}

pub async fn start_server(https: bool) -> Result<(), std::io::Error> {
    let version = built_info::PKG_VERSION;
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let cors = Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST)
        .allow_credentials(false);

    let api_service = OpenApiService::new(Api::default(), "OispaHallaAnticheat", version)
        .server("https://hac.oispahalla.com/api")
        .server("https://hac.fly.dev/api")
        .server("https://localhost/api")
        .server("http://localhost/api");
    let ui = api_service.swagger_ui();

    let key = get_key()?;
    let cert = get_cert()?;

    let router = Route::new()
        .nest("/", ui)
        .nest("/api", api_service)
        .with(cors);
    let bind = std::env::var("HTTP_BIND")
        .unwrap_or_else(|_| if https { "0.0.0.0:443" } else { "0.0.0.0:80" }.to_string());
    if https {
        let listener = TcpListener::bind(bind)
            .rustls(RustlsConfig::new().fallback(RustlsCertificate::new().key(key).cert(cert)));

        return Server::new(listener).run(router).await;
    } else {
        let listener = TcpListener::bind(bind);

        return Server::new(listener).run(router).await;
    };
}
