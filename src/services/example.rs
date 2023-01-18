use actix_web::{get, post, web, HttpResponse, Responder, Scope};

pub fn example_service() -> Scope {
    web::scope("/example").service(hello).service(echo)
}

// Could build a macro to generate a simple CRUD API for me?

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
