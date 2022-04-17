pub mod models {
  use serde::{Deserialize, Serialize};

  #[derive(Serialize, Deserialize, Debug)]
  pub struct CreatePostRequest {
    pub author: String,
    pub content: String,
  }

  #[derive(Serialize, Deserialize, Debug)]
  pub struct Post {
    pub author: String,
    pub created_at: i64,
    pub content: String,
    pub id: String,
  }

  #[derive(Serialize, Deserialize, Debug)]
  pub struct Feed {
    pub posts: Vec<Post>,
  }
}
