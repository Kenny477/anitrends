// This example uses 3 crates serde_json, reqwest, tokio

use reqwest::Client;
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

const SINGLE_TRENDING: &str = "
query($mediaList: [Int]) {
  MediaTrend(mediaId_not_in: $mediaList, sort: TRENDING_DESC){
    date
    averageScore
    popularity
    inProgress
    releasing
    episode
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
";

const TRENDING: &str = "
query { 
  Page (page: 1, perPage: 10){
    pageInfo {
      total
    }
    mediaTrends(releasing: true, episode_not: null, sort: POPULARITY_DESC) {
      date
			averageScore
      popularity
      inProgress
      releasing
      episode
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
pub async fn build_trending(limit: usize) -> JsValue{
  let mut trending_list: Vec<Value> = Vec::new();
  let mut media_id_list: Vec<i64> = Vec::new();
  while trending_list.len() < limit {
    let res = search(SINGLE_TRENDING, Some(json!({"mediaList": media_id_list})))
      .await;
    trending_list.push(res["data"]["MediaTrend"]["media"].clone());
    media_id_list.push(res["data"]["MediaTrend"]["media"]["id"].as_i64().unwrap());
  }
  let trending = json!({"trending": trending_list});
  return JsValue::from_serde(&trending).unwrap();
}

#[wasm_bindgen]
pub async fn trending() -> JsValue {
    return search_js(TRENDING, None).await;
}

#[wasm_bindgen]
pub async fn airing() -> JsValue {
    return search_js(AIRING, None).await;
}

async fn search(query: &str, variables: Option<serde_json::Value>) -> Value {
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
    return result;
}

async fn search_js(query: &str, variables: Option<serde_json::Value>) -> JsValue {
    let result = search(query, variables).await;
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
    console_log!("Hello JavaScript!");
}
