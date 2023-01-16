use actix_web::{App, HttpServer};
mod services;

use services::example::{
    example_service,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(example_service())
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
