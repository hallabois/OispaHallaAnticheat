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
use twothousand_forty_eight::{
    parser::parse_data,
    validator::{validate_first_move, validate_history},
};

#[derive(Tags)]
enum ApiTags {
    /// Information about the server
    Meta,
}

#[derive(Default)]
struct Api {
    request_count: Mutex<usize>,
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
        pub mod built_info {
            // The file has been placed there by the build script.
            include!(concat!(env!("OUT_DIR"), "/built.rs"));
        }
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
        StatsResponse::Ok(Json(Stats {
            request_count,
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

        let recording = parse_data(run.clone());
        match recording {
            Err(e) => {
                println!("Error while parsing run \"{}\"", run);
                println!("Error {:?}", e);
                let mut error_count = self.error_count.lock().await;
                *error_count += 1;
                ValidationResponse::ParsingFailed(poem_openapi::payload::PlainText(e.to_string()))
            }
            Ok(history) => {
                let length = history.history.len();
                println!("Loaded record with the length of {}.", length);
                let hash = history.hash_v1();
                let w = history.width;
                let h = history.height;
                let result0 = validate_first_move(&history);
                let result1 = validate_history(history);
                let valid = result0 && result1.is_ok();
                println!("Run <{}>", hash);
                println!("\tBoard size: {}x{}", w, h);
                println!("\tValid: {}", valid);
                if let Ok(data) = result1 {
                    println!("\tRun score: {}", data.score);
                    println!("\tBreaks used: {}", data.breaks);
                }

                match result1 {
                    Err(e) => ValidationResponse::InvalidRun(poem_openapi::payload::PlainText(
                        e.to_string(),
                    )),
                    Ok(data) => ValidationResponse::Ok(Json(ValidationOK {
                        run_hash: hash,
                        board_w: w,
                        board_h: h,
                        score: data.score,
                        score_end: data.score_end,
                        score_margin: data.score_margin,
                        breaks: data.breaks,
                        length: length,
                    })),
                }
            }
        }
    }
}

pub async fn start_server(https: bool) -> Result<(), std::io::Error> {
    pub mod built_info {
        // The file has been placed there by the build script.
        include!(concat!(env!("OUT_DIR"), "/built.rs"));
    }
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

    if https {
        let https_bind = std::env::var("HTTPS_BIND").unwrap_or_else(|_| "0.0.0.0:443".to_string());
        let listener = TcpListener::bind(https_bind)
            .rustls(RustlsConfig::new().fallback(RustlsCertificate::new().key(key).cert(cert)));

        return Server::new(listener)
            .run(
                Route::new()
                    .nest("/", ui)
                    .nest("/api", api_service)
                    .with_if(true, cors),
            )
            .await;
    } else {
        let http_bind = std::env::var("HTTP_BIND").unwrap_or_else(|_| "0.0.0.0:80".to_string());
        let listener = TcpListener::bind(http_bind);

        return Server::new(listener)
            .run(
                Route::new()
                    .nest("/", ui)
                    .nest("/api", api_service)
                    .with_if(true, cors),
            )
            .await;
    }
}
