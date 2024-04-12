extern crate reqwest;
use crate::data_toml;
use crate::data_refresh;
use crate::url;
use serde_json::json;

pub async fn post_request(rkey: String, col: String) -> String {
    let token = data_refresh(&"access");
    //let did = data_toml(&"did");
    let handle = data_toml(&"handle");

    let url = url(&"record_delete");

    let post = Some(json!({
        "repo": handle.to_string(),
        "rkey": rkey.to_string(),
        "collection": col.to_string()
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
