extern crate reqwest;
use crate::data::Open;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
struct OpenChar {
    choices: Vec<ChoicesChar>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChoicesChar {
    message: OpenContent,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenContent {
    content: String,
}

pub async fn post_request(prompt: String) -> String {
    let data = Open::new().unwrap();
    let data = Open { api: data.api };

    let setting = "あなたはyuiという作品のアイと呼ばれるキャラクターです。第一人称は「アイ」です。
    第二人称はできる限り省略してください。
    自由気ままな優しさを兼ね備えます。
    口調例：アイね、ここでお昼寝するの。
    幼い子供の口調で話してください。
    誕生日は1月23日。
    自分を信じて。
        ";

    let post = Some(json!({
    "model": "gpt-3.5-turbo",
    "messages": [
    {"role": "system", "content": &setting.to_string()},
    {"role": "user", "content": &prompt.to_string()},
    ]
    }));

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", "Bearer ".to_owned() + &data.api)
        .json(&post)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let p: OpenChar = serde_json::from_str(&res).unwrap();
    let o = &p.choices[0].message.content;

    return o.to_string();
}
