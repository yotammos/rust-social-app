use chrono::Local;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

mod ddb_client;
mod models;

use ddb_client::ddb_client::{create_post, list_posts};
use models::models::{CreatePostRequest, Feed, Post};
use uuid::Uuid;

#[macro_use]
extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
  "Hello World"
}

#[post("/post", data = "<body>")]
async fn post(body: Json<CreatePostRequest>) -> Result<String, status::Custom<String>> {
  let uuid = Uuid::new_v4();
  let post = Post {
    id: uuid.to_string(),
    created_at: Local::now().timestamp(),
    author: body.0.author,
    content: body.0.content,
  };
  match create_post(post).await {
    Ok(_result) => Ok(uuid.to_string()),
    Err(_error) => Err(status::Custom(
      Status::InternalServerError,
      String::from("failed to create post"),
    )),
  }
}

#[get("/feed")]
async fn feed() -> Result<String, status::Custom<String>> {
  match list_posts().await {
    Ok(posts) => Ok(serde_json::to_string(&Feed { posts }).unwrap()),
    Err(_error) => Err(status::Custom(
      Status::InternalServerError,
      String::from("failed to lists posts"),
    )),
  }
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/hello", routes![world])
    .mount("/", routes![feed, post])
}
