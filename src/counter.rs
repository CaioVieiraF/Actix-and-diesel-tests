use actix_web::{get, web, HttpResponse, Responder};
use crate::AppState;

#[get("/up")]
pub async fn increase_counter(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    HttpResponse::Ok().body(format!("Counter value: {counter}"))
}

#[get("/down")]
pub async fn decrease_counter(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter -= 1;

    HttpResponse::Ok().body(format!("Counter value: {counter}"))
}

#[get("/")]
pub async fn get_counter(data: web::Data<AppState>) -> impl Responder {
    let counter_value = data.counter.lock().unwrap();
    HttpResponse::Ok().body(format!("The current value is: {counter_value:?}"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("counter")
        .service(increase_counter)
        .service(decrease_counter)
        .service(get_counter)
    );
}
