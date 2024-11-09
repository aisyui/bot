extern crate reqwest;
use crate::data_toml;
use crate::data_refresh;
use crate::url;
use iso8601_timestamp::Timestamp;
use serde_json::json;

pub async fn post_request(col: String, user_name: String, user_did: String, user_handle: String, aiten: i32, limit: i32, chara: String, lv: i32, exp: i32, hp: i32, rank: i32, mode: i32, attach: i32, critical: i32, critical_d: i32) -> String {
    let token = data_refresh(&"access");
    let did = data_toml(&"did");
    let handle = data_toml(&"handle");
    let url = url(&"record_put");
    let d = Timestamp::now_utc();
    let d = d.to_string();
    let post = Some(json!({
        "repo": handle.to_string(),
        "did": did.to_string(),
        "collection": col.to_string(),
        "rkey": user_name.to_string(),
        "record": {
            "did": user_did.to_string(),
            "handle": user_handle.to_string(),
            "aiten": aiten,
            "limit": limit,
            "character": {
                chara.to_string(): {
                    "lv": lv,
                    "exp": exp,
                    "hp": hp,
                    "rank": rank,
                    "mode": mode,
                    "attach": attach,
                    "critical": critical,
                    "critical_d": critical_d,
                }
            },
            "createdAt": d.to_string(),
            "updatedAt": d.to_string(),
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
