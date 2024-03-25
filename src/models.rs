use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LanguageDetails {
    pub language: String,
    pub display: String,
}

#[derive(Deserialize)]
pub struct RunCodeParams {
    pub language: String,
    pub code: String,
    pub input: Option<String>,
}

#[derive(Serialize)]
pub struct RunCodeResult {
    pub output: String,
    pub error: String,
}
