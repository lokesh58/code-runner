use actix_web::{get, post, web, Responder, Result};
use code_runner::models::{LanguageDetails, RunCodeParams, RunCodeResult};

#[get("/list")]
async fn list_supported_languages() -> Result<impl Responder> {
    Ok(web::Json(vec![LanguageDetails {
        language: String::from("dummy"),
        display: String::from("Dummy Lang for test"),
    }]))
}

#[post("/run")]
async fn run_code(params: web::Json<RunCodeParams>) -> Result<impl Responder> {
    println!(
        "Received params:\nlanguage: {},\ncode: {},\ninput: {}",
        params.language,
        params.code,
        params.input.clone().unwrap_or(String::from("<empty>"))
    );
    Ok(web::Json(RunCodeResult {
        output: String::from("TODO: Run code to get actual output"),
        error: String::from("TODO: Run code to get actual error"),
    }))
}

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    let json_config = web::JsonConfig::default().limit(4096);
    cfg.service(
        web::scope("/api")
            .app_data(json_config)
            .service(list_supported_languages)
            .service(run_code),
    );
}
