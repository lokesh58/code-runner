use actix_web::{get, post, web, Error, HttpResponse};
use code_runner::models::RunCodeParams;

#[get("/list")]
async fn list_supported_languages() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("TODO: return list"))
}

#[post("/run")]
async fn run_code(params: web::Json<RunCodeParams>) -> Result<HttpResponse, Error> {
    println!(
        "Received params:\nlanguage: {},\ncode: {},\ninput: {}",
        params.language,
        params.code,
        params.input.clone().unwrap_or(String::from("<empty>"))
    );
    Ok(HttpResponse::Ok().body("TODO: Return code output"))
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
