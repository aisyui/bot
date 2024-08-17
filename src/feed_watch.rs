use seahorse::Context;

//use crate::openai;
use crate::feed_get;

use crate::data::data_toml;
use crate::data::Timeline;

pub fn c_feed_watch(c: &Context) {
    let mut feed = "at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.generator/cmd".to_string();
    if c.string_flag("url").is_ok() {
        feed = c.string_flag("url").unwrap();
    }
    let mut tag = "syai".to_string();
    if c.string_flag("tag").is_ok() {
        tag = c.string_flag("tag").unwrap();
    }

    let h = async {
        let notify = feed_get::get_request(feed).await;
        if notify == "err" {
            return;
            //refresh(c);
            //notify = feed_get::get_request("at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.generator/cmd".to_string()).await;
        }
        let timeline: Timeline = serde_json::from_str(&notify).unwrap();
        let n = timeline.feed;
        let host = data_toml(&"host");
        let length = &n.len();
        let su = 0..*length;
        for i in su {
            let handle = &n[i].post.author.handle;
            let did = &n[i].post.author.did;
            let cid = &n[i].post.cid;
            let uri = &n[i].post.uri;
            let _time = &n[i].post.indexedAt;
            let cid_root = cid;
            let uri_root = uri;

            let mut text = "";
            if !n[i].post.record.text.is_none() {
                text = &n[i].post.record.text.as_ref().unwrap();
            }

            let vec: Vec<&str> = text.split_whitespace().collect();
            let com = vec[0].trim().to_string();
            let mut prompt = "".to_string();
            let mut prompt_sub = "".to_string();

            if com == "@ai" || com == "/ai" || com == tag {
                prompt_sub = vec[1..].join(" ");
            } else {
                prompt = vec[1..].join(" ");
                if vec.len() > 1 {
                    prompt_sub = vec[2..].join(" ");
                }
            }

            if prompt.is_empty() == false || com.is_empty() == false {
                println!("{}", handle);

                if c.bool_flag("debug") == true {
                    println!(
                        "cid:{}\nuri:{}\ncid_root:{}\nuri_root:{}\nhost:{}\ndid:{}",
                        cid, uri, cid_root, uri_root, host, did
                    );
                }
                println!("{}", prompt_sub);
                println!("---");
            }
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}
