use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse, Object,
};

#[derive(ApiResponse)]
pub enum AliveResponse {
    /// Returns when alive is requested.
    #[oai(status = 200)]
    Ok,
}

#[derive(Debug, Clone, Object)]
pub struct ServerConfig {
    pub platform: String,
    pub version: String,
    pub rust_version: String,
}

#[derive(ApiResponse)]
pub enum GetConfigResponse {
    /// Returns when get_config is requested.
    #[oai(status = 200)]
    Ok(Json<ServerConfig>),
}

#[derive(Debug, Clone, Object)]
pub struct Stats {
    pub request_count: usize,
    pub invalid_count: usize,
    pub error_count: usize,
}

#[derive(ApiResponse)]
pub enum StatsResponse {
    /// Returns when stats is requested.
    #[oai(status = 200)]
    Ok(Json<Stats>),
}

#[derive(Debug, Clone, Object)]
pub struct ValidationOK {
    pub run_hash: String,
    pub board_w: usize,
    pub board_h: usize,
    pub score: usize,
    pub score_end: usize,
    pub score_margin: usize,
    pub breaks: usize,
    pub break_positions: Vec<usize>,
    pub length: usize,
}

#[derive(ApiResponse)]
pub enum ValidationResponse {
    /// The data was able to be parsed
    #[oai(status = 200)]
    Ok(Json<ValidationOK>),

    /// Parsing the data failed
    #[oai(status = 400)]
    ParsingFailed(PlainText<String>),

    /// Invalid run, error in plaintext
    #[oai(status = 418)]
    InvalidRun(PlainText<String>),
}
