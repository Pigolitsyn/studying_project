use diesel::{RunQueryDsl, QueryDsl};
use crate::{schema, establish_connection, models::{NewPost, Post, PostDto}};
use actix_web::{web::Json};


pub fn find_post(_id: i32) -> Json<Post> {
    
    use schema::posts::dsl::*;
    let connection = establish_connection();

    let result = posts.find(_id).first::<Post>(&connection).expect("Error to load post with id");
    println!("{}", result.title);
    Json(result)
}


pub fn show_posts() -> Json<Vec<Post>> {
    use schema::posts::dsl::*;
    let connection = establish_connection();
    let selected_posts = posts.load::<Post>(&connection)
        .expect("Error loading posts");
    Json(selected_posts)
}

pub fn create_post(post: Json<PostDto>) -> Post {
    write_post(post.title.as_str(), post.body.as_str())
}

fn write_post<'a>(title: &'a str, body: &'a str) -> Post {
    
    use schema::posts;
    
    let connection = establish_connection();
    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&connection)
        .expect("Error saving new post")
}
