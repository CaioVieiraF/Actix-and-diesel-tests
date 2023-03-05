use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: i32,
}
