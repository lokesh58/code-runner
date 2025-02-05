mod middleware;
mod routes;

use actix_web::{middleware::Logger, App, HttpServer};
use routes::setup_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| App::new().wrap(Logger::default()).configure(setup_routes))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
