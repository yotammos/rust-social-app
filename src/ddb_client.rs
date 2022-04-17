pub mod ddb_client {
  use crate::models::models::Post;

  use aws_sdk_dynamodb::error::{PutItemError, ScanError};
  use aws_sdk_dynamodb::model::AttributeValue;
  use aws_sdk_dynamodb::output::PutItemOutput;
  use aws_sdk_dynamodb::{Client, Region};
  use aws_smithy_http::result::SdkError;
  use std::collections::HashMap;
  use std::str::FromStr;

  fn unwrap_number_attribute<T: FromStr>(
    atr: Option<&AttributeValue>,
  ) -> Result<T, <T as FromStr>::Err> {
    atr.unwrap().as_n().unwrap().parse::<T>()
  }

  fn item_to_post(item: &HashMap<String, AttributeValue>) -> Post {
    let id = item.get("id").unwrap().as_s().unwrap().to_string();
    let author = item.get("author").unwrap().as_s().unwrap().to_string();
    let content = item.get("content").unwrap().as_s().unwrap().to_string();
    let created_at = unwrap_number_attribute(item.get("created_at")).unwrap();
    Post {
      author,
      id,
      content,
      created_at,
    }
  }

  pub async fn list_posts() -> Result<Vec<Post>, SdkError<ScanError>> {
    let shared_config = aws_config::from_env()
      .region(Region::new("us-east-1"))
      .load()
      .await;
    let client = Client::new(&shared_config);
    let request = client.scan().table_name("posts");
    request.send().await.map(|output| {
      output
        .items
        .unwrap()
        .iter()
        .map(|item| item_to_post(item))
        .collect::<Vec<Post>>()
    })
  }

  pub async fn create_post(post: Post) -> Result<PutItemOutput, SdkError<PutItemError>> {
    let shared_config = aws_config::from_env()
      .region(Region::new("us-east-1"))
      .load()
      .await;
    let client = Client::new(&shared_config);
    let request = client
      .put_item()
      .table_name("posts")
      .item("id", AttributeValue::S(post.id.to_string()))
      .item("author", AttributeValue::S(post.author))
      .item("content", AttributeValue::S(post.content))
      .item("created_at", AttributeValue::N(post.created_at.to_string()));
    request.send().await
  }
}
