use actix_web::{get, post, web, Error, HttpResponse};

#[get("/list")]
async fn list_supported_languages() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("TODO: return list"))
}

#[post("/run")]
async fn run_code() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("TODO: Return code output"))
}

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(list_supported_languages)
            .service(run_code),
    );
}
