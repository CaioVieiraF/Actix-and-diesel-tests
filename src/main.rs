pub mod counter;
pub mod testing;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

pub struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/app_name")]
async fn get_app_name(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("The name is {}", app_name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_data = web::Data::new(AppState{
        app_name: String::from("Actix test"),
        counter: Mutex::new(0),
    }); 

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(counter::config)
            .configure(testing::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
