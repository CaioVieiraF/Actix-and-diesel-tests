use actix_web::{get, Responder, web, HttpResponse};
use crate::AppState;
use crate::models::*;
use diesel::prelude::*;

#[get("/")]
pub async fn get_posts(data: web::Data<AppState>) -> web::Json<Vec<Post>> {
    use crate::schema::posts::dsl::*;

    let mut connection = data.database.get().unwrap();
    let post_list = posts.filter(published.eq(0)).limit(5).load::<Post>(&mut connection).expect("Error loading posts");

    web::Json(post_list)
}

#[get("/{id}")]
pub async fn get_post_by_id(data: web::Data<AppState>, info: web::Path<i32>) -> web::Json<Post> {
    use crate::schema::posts::dsl::*;

    let mut connection = data.database.get().unwrap();
    let post_id = info.into_inner();
    let post_vec = posts.filter(id.eq(post_id)).load::<Post>(&mut connection).expect("Error loading posts");
    let post = post_vec[0].clone();

    web::Json(post)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/posts")
        .service(get_posts)
        .service(get_post_by_id)
    );
}
