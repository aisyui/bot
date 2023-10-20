extern crate reqwest;
use crate::data_refresh;
use crate::url;

pub async fn get_request(user: String) -> String {
    let token = data_refresh(&"access");
    let url = url(&"profile_get") + &"?handle=" + &user;

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("Authorization", "Bearer ".to_owned() + &token)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    return res;
}
