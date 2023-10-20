extern crate reqwest;
use crate::data_toml;
use crate::data_refresh;
use crate::url;
use iso8601_timestamp::Timestamp;
use serde_json::json;

pub async fn post_request(text: String, at: String, udid: String, s: i32, e: i32) -> String {
    let token = data_refresh(&"access");
    let did = data_toml(&"did");
    let handle = data_toml(&"handle");

    let url = url(&"record_create");
    let col = "app.bsky.feed.post".to_string();

    let d = Timestamp::now_utc();
    let d = d.to_string();

    let post = Some(json!({
        "did": did.to_string(),
        "repo": handle.to_string(),
        "collection": col.to_string(),
        "record": {
            "text": at.to_string() + &" ".to_string() + &text.to_string(),
            "$type": "app.bsky.feed.post",
            "createdAt": d.to_string(),
            "facets": [
            {
                "$type": "app.bsky.richtext.facet",
                "index": {
                    "byteEnd": e,
                    "byteStart": s
                },"features": [
                {
                    "did": udid.to_string(),
                    "$type": "app.bsky.richtext.facet#mention"
                }
                ]
            }
            ]
        },
    }));

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&post)
        .header("Authorization", "Bearer ".to_owned() + &token)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    return res;
}
