mod routes;

use actix_web::{App, HttpServer};
use routes::setup_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(setup_routes))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
