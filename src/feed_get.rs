extern crate reqwest;
use crate::data_refresh;
use crate::url;

pub async fn get_request(feed: String) -> String {
    let token = data_refresh(&"access");
    let url = url(&"feed_get");
    let feed = feed.to_string();
    //let col = "app.bsky.feed.generator".to_string();

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .query(&[("feed", feed)])
        //.query(&[("feed", feed), ("collection", col)])
        .header("Authorization", "Bearer ".to_owned() + &token)
        .send()
        .await
        .unwrap();
 
    let status_ref = res.error_for_status_ref();

    match status_ref {
        Ok(_) => {
            return res.text().await.unwrap();
        }
        Err(_e) => {
            let e = "err".to_string();
            return e;
        }
    }

}
