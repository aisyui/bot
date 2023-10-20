extern crate reqwest;
use crate::data_toml;
use crate::data_refresh;
use crate::url;
use iso8601_timestamp::Timestamp;
use serde_json::json;

pub async fn post_request(
    text: String,
    link: String,
    s: i32,
    e: i32,
    cid: String,
    uri: String,
    cid_root: String,
    uri_root: String,
) -> String {
    let token = data_refresh(&"access");
    let did = data_toml(&"did");
    let handle = data_toml(&"handle");

    let url = url(&"record_create");
    let col = "app.bsky.feed.post".to_string();

    let d = Timestamp::now_utc();
    let d = d.to_string();

    let post = Some(json!({
        "repo": handle.to_string(),
        "did": did.to_string(),
        "collection": col.to_string(),
        "record": {
            "text": link.to_string() + &" ".to_string() + &text.to_string(),
            "createdAt": d.to_string(),
            "reply": {
                "root": {
                    "cid": cid_root.to_string(),
                    "uri": uri_root.to_string()
                },
                "parent": {
                    "cid": cid.to_string(),
                    "uri": uri.to_string()
                }
            },
            "facets": [
            {
                "index": {
                    "byteStart": s,
                    "byteEnd": e
                },
                "features": [
                {
                    "$type": "app.bsky.richtext.facet#link",
                    "uri": link.to_string()
                }
                ]
            }
            ],
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
