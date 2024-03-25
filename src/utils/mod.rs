use crate::models::{LanguageDetails, RunCodeParams, RunCodeResult};

pub fn get_all_languages() -> Vec<LanguageDetails> {
    // TODO: Implement this
    vec![LanguageDetails {
        language: String::from("dummy"),
        display: String::from("Dummy Language"),
    }]
}

pub async fn run_code(params: RunCodeParams) -> RunCodeResult {
    println!(
        "Received params:\nlanguage: {},\ncode: {},\ninput: {}",
        params.language,
        params.code,
        params.input.clone().unwrap_or(String::from("<empty>"))
    );
    RunCodeResult {
        output: String::from("TODO: Run code to get actual output"),
        error: String::from("TODO: Run code to get actual error"),
    }
}
