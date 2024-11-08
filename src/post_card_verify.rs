extern crate reqwest;
use crate::data_toml;
use crate::data_refresh;
use crate::url;
use iso8601_timestamp::Timestamp;
use serde_json::json;

pub async fn post_request(col: String, img: String, id: i32, cp: i32, rank: i32, rare: String, user_handle: String, user_did: String) -> String {
    let token = data_refresh(&"access");
    let did = data_toml(&"did");
    let handle = data_toml(&"handle");
    let url = url(&"record_create");
    let d = Timestamp::now_utc();
    let d = d.to_string();
    let link = "https://bsky.app/profile/yui.syui.ai".to_string();
    let post = Some(json!({
        "repo": handle.to_string(),
        "did": did.to_string(),
        "collection": col.to_string(),
        "record": {
            "id": id,
            "cp": cp,
            "rank": rank,
            "rare": rare.to_string(),
            "handle": user_handle.to_string(),
            "did": user_did.to_string(),
            "embed": {
                "$type": "app.bsky.embed.external",
                "external": {
                    "uri": link,
                    "thumb": {
                      "$type": "blob",
                      "ref": {
                        "$link": img.to_string()
                      },
                      "mimeType": "image/jpeg",
                      "size": 0
                    }
                }
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
