extern crate reqwest;
use crate::data_toml;
use crate::data_refresh;
use crate::url;
use iso8601_timestamp::Timestamp;
use serde_json::json;

pub async fn post_request(cid: String, uri: String) -> String {
    let token = data_refresh(&"access");
    let did = data_toml(&"did");
    let handle = data_toml(&"handle");

    let url = url(&"record_create");
    let col = "app.bsky.feed.like".to_string();

    let d = Timestamp::now_utc();
    let d = d.to_string();

    let post = Some(json!({
        "repo": handle.to_string(),
        "did": did.to_string(),
        "collection": col.to_string(),
        "record": {
            "subject": {
                "uri": uri.to_string(),
                "cid": cid.to_string()
            },
            "createdAt": d.to_string(),
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
