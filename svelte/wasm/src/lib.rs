// This example uses 3 crates serde_json, reqwest, tokio

use reqwest::Client;
use serde_json::{json};
use wasm_bindgen::prelude::*;

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

const AIRING: &str = "
query {
  Page {
    pageInfo {
      total
    }
    airingSchedules(notYetAired: true) {
      id
      airingAt
      timeUntilAiring
      episode
      mediaId
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

#[wasm_bindgen]
pub async fn trending() -> JsValue {
    return search(TRENDING, None).await;
}

#[wasm_bindgen]
pub async fn airing() -> JsValue {
    return search(AIRING, None).await;
}

async fn search(query: &str, variables: Option<serde_json::Value>) -> JsValue {
    let client = Client::new();
    let json = json!({"query": query, "variables": variables.unwrap_or_default()});
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
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();
    return JsValue::from_serde(&result).unwrap();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
  // Note that this is using the `log` function imported above during
  // `bare_bones`
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet() {
    console_log!("Hello Javascript!");
}
