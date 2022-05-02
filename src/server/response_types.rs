use poem_openapi::{ApiResponse, Object, payload::Json};

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
    pub error_count: usize
}

#[derive(ApiResponse)]
pub enum StatsResponse {
    /// Returns when stats is requested.
    #[oai(status = 200)]
    Ok(Json<Stats>),
}

#[derive(Debug, Clone, Object)]
pub struct ValidationData {
    pub run_hash: String,
    pub board_w: usize,
    pub board_h: usize,
    pub valid: bool,
    pub score: usize,
    pub score_margin: usize,
    pub breaks: usize,
    pub length: usize
}

#[derive(ApiResponse)]
pub enum ValidationResponse {
    /// The data was able to be parsed
    #[oai(status = 200)]
    Ok(Json<ValidationData>),

    /// Parsing the data failed
    #[oai(status = 400)]
    ParsingFailed
}