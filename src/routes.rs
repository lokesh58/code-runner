use crate::middleware::auth::auth_middleware;
use actix_web::{get, post, web, Responder, Result};
use code_runner::{models::RunCodeParams, utils};

#[get("/list")]
async fn list_supported_languages() -> Result<impl Responder> {
    let supported_languages = utils::get_all_languages();
    Ok(web::Json(supported_languages))
}

#[post("/run", wrap = "auth_middleware()")]
async fn run_code(params: web::Json<RunCodeParams>) -> Result<impl Responder> {
    let result = utils::run_code(params.into_inner()).await?;
    Ok(web::Json(result))
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
