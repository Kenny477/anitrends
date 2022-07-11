// This example uses 3 crates serde_json, reqwest, tokio

use reqwest::Client;
use serde_json::json;
use wasm_bindgen::prelude::*;

// Query to use in request
const TRENDING: &str = "
query { 
  Page (page: 1, perPage: 10){
    pageInfo {
      total
    }
    mediaTrends(sort: POPULARITY_DESC) {
      media {
        id
        title {
          english
          romaji
          native
        }
        description
        status
        coverImage {
          extraLarge
          large
          medium
          color
        }
      }
    }
  }
}
";

pub fn main() {
  poll(query());
}

// #[tokio::main]
#[wasm_bindgen]
pub async fn query() {
    let client = Client::new();
    // Define query and variables
    let json = json!({"query": TRENDING});
    // Make HTTP post request
    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await
        .unwrap()
        .text()
        .await;
    // Get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();
    println!("{:#?}", result);
}
