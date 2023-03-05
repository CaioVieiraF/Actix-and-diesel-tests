pub mod counter;
pub mod testing;
pub mod queries;
pub mod models;
pub mod schema;

use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use std::env;

pub struct AppState {
    app_name: String,
    counter: Mutex<i32>,
    database: r2d2::Pool<ConnectionManager<SqliteConnection>>,
}

#[get("/app_name")]
async fn get_app_name(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("The name is {}", app_name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let manager = ConnectionManager::<SqliteConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set."));
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let app_data = web::Data::new(AppState{
        app_name: String::from("Actix test"),
        counter: Mutex::new(0),
        database: pool,
    }); 

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(counter::config)
            .configure(testing::config)
            .configure(queries::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
