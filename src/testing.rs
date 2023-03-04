use actix_web::{get, web, HttpResponse, Responder};

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Start page")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("testing")
        .service(hello)
        .service(index)
    );
}
