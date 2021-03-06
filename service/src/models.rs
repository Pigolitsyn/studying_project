use diesel::{Queryable, Insertable};
use super::schema::posts;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Deserialize)]
pub struct PostDto {
    pub title: String,
    pub body: String
}