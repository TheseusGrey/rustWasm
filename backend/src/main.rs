mod api;
mod repository;

use api::task::{
    get_task,
    submit_task,
    start_task,
    complete_task,
    pause_task,
    fail_task,
};
use repository::ddb::DDBRepository;
use actix_web::{HttpServer, App, web::Data, web::scope, middleware::Logger, Result, HttpRequest};
use actix_files::NamedFile;
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("./dist/index.html").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web};

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;
    HttpServer::new(move || {
        let ddb_repo: DDBRepository = DDBRepository::init(
            String::from("task"),
            config.clone()
        );
        let ddb_data = Data::new(
            ddb_repo
        );
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(
                scope("/api")
                    .service(get_task)
                    .service(submit_task)
                    .service(start_task)
                    .service(complete_task)
                    .service(pause_task)
                    .service(fail_task)
            )
            .service(
                scope("/")
                .route("/index.html", web::get().to(index))
                )

    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
