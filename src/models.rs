use serde::Deserialize;

#[derive(Deserialize)]
pub struct RunCodeParams {
    pub language: String,
    pub code: String,
    pub input: Option<String>,
}
