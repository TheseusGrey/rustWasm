use crate::{lib::crud::CRUDRepository, repository::exampleRepository::ExampleData};
use actix_web::{get, post, web, HttpResponse, Responder, Scope};

pub fn example_service() -> Scope {
    web::scope("/example").service(hello).service(echo)
}

async fn all() -> impl Responder {
    HttpResponse::Ok().body(ExampleData::load())
}

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
