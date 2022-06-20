use actix_web::{get, HttpResponse, post, web::{Json, self}};
use serde_json::json;
use service::{posts::{show_posts, create_post, find_post}, models::{PostDto}};

#[get("/post/{post_id}")]
async fn get_one_post(path: web::Path<i32>) -> HttpResponse {
    let post_id = path.into_inner();
    HttpResponse::Ok().json(find_post(post_id))
}

#[get("/posts")]
async fn get_all_posts() -> HttpResponse {
    HttpResponse::Ok().json(show_posts())
}

#[post("/post/create")]
async fn create_new_post(post: Json<PostDto>) -> HttpResponse {
    HttpResponse::Ok().json(json!(create_post(post)))
}