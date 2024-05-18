use seahorse::Context;
use std::process::Command;

use crate::notify;
use crate::notify_read;
use crate::openai;
use crate::refresh;
use crate::reply;
use crate::reply_link;
use crate::feed_get;

use crate::data::c_char;
use crate::data::data_scpt;
use crate::data::data_toml;
use crate::data::log_file;
use crate::data::w_cid;
use crate::data::Notify;
use crate::data::Timeline;

pub fn c_bot(c: &Context) {
    let h = async {
        let mut notify = notify::get_request(50).await;
        if notify == "err" {
            refresh(c);
            notify = notify::get_request(50).await;
        }
        let notify: Notify = serde_json::from_str(&notify).unwrap();
        let host = data_toml(&"host");
        let n = notify.notifications;
        let length = &n.len();
        let su = 0..*length;
        for i in su {
            let reason = &n[i].reason;
            let handle = &n[i].author.handle;
            let did = &n[i].author.did;
            let read = n[i].isRead;
            let cid = &n[i].cid;
            let uri = &n[i].uri;
            let time = &n[i].indexedAt;
            let mut cid_root = cid;
            let mut uri_root = uri;
            let check_cid = w_cid(cid.to_string(), log_file(&"n1"), false);
            let check_cid_run = w_cid(cid.to_string(), log_file(&"n2"), false);
            // thread
            if !n[i].record.reply.is_none() {
                cid_root = &n[i].record.reply.as_ref().unwrap().root.cid;
                uri_root = &n[i].record.reply.as_ref().unwrap().root.uri;
            }
            let check_uri_comment = w_cid(uri_root.to_string(), log_file(&"c1"), false);

            let mut text = "";
            if !n[i].record.text.is_none() {
                text = &n[i].record.text.as_ref().unwrap();
            }
            let vec: Vec<&str> = text.split_whitespace().collect();
            let handlev: Vec<&str> = handle.split('.').collect();
            let mut handlev = handlev[0].trim().to_string();
            let mut ten_p = "false";

            let mut link = "https://card.syui.ai/".to_owned() + &handlev;
            let s = 0;
            let mut e = link.chars().count();

            let mut com = "".to_string();
            let mut prompt = "".to_string();
            let mut prompt_sub = "".to_string();
            let mut prompt_chat = "".to_string();
            if reason == "mention" {
                com = vec[1].trim().to_string();
                prompt = vec[2..].join(" ");
                prompt_chat = vec[1..].join(" ");
                if vec.len() > 2 {
                    prompt_sub = vec[3..].join(" ");
                }
            } else if reason == "reply" {
                com = vec[0].trim().to_string();
                prompt = vec[1..].join(" ");
                prompt_chat = vec[0..].join(" ");
                if vec.len() > 1 {
                    prompt_sub = vec[2..].join(" ");
                }
            }

            if prompt.is_empty() == false || com.is_empty() == false {
                println!("{}", read);
                println!("{}", handle);
                println!(
                    "cid:{}\nuri:{}\ncid_root:{}\nuri_root:{}\nhost:{}",
                    cid, uri, cid_root, uri_root, host
                );
                println!("reason:{}\ncom:{}\nprompt:{}", reason, com, prompt);
                println!("prompt_sub:{}", prompt_sub);
            }

            let mut admin = "".to_string();
            if c.string_flag("admin").is_ok() {
                admin = c.string_flag("admin").unwrap();
            }

            let mut avatar = "".to_string();
            if let Some(v) = &n[i].author.avatar {
                avatar = v.to_string();
            }

            if check_cid == false && { reason == "mention" || reason == "reply" }
                || check_cid_run == false && { reason == "mention" || reason == "reply" }
            {
                w_cid(cid.to_string(), log_file(&"n2"), true);
                if com == "did" || com == "/did" {
                    let link = "https://plc.directory/".to_owned() + &did + &"/log";
                    let s = 0;
                    let e = link.chars().count();
                    let d = "\n".to_owned() + &did.to_string();
                    let text_limit = c_char(d);
                    if text_limit.len() > 3 {
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                        .await;
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                        println!("{}", str_rep);
                    }
                } else if com == "help" || com == "/help" {
                    let link = "https://git.syui.ai/ai/bot/wiki/help".to_string();
                    let s = 0;
                    let e = link.chars().count();
                    let str_rep = reply_link::post_request(
                        "\n".to_string(),
                        link.to_string(),
                        s,
                        e.try_into().unwrap(),
                        cid.to_string(),
                        uri.to_string(),
                        cid_root.to_string(),
                        uri_root.to_string(),
                    )
                        .await;
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                    println!("{}", str_rep);
                } else if { com == "diffusers"  || com == "/diffusers" } && handle == &admin{
                    let _output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"diffusers")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                } else if com.contains("占") == true
                    || com.contains("うらない") == true
                    || com.contains("うらなって") == true
                {
                    let _output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"fortune")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                } else if com == "card" || com == "/card" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"card")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    if text_limit.len() > 3 {
                        //handlev = handle.replace(".", "-").to_string();
                        handlev = d.lines().collect::<Vec<_>>()[0].to_string();
                        link = "https://card.syui.ai/".to_owned() + &handlev;
                        e = link.chars().count();
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                            .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if com == "fav" || com == "/fav" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"fav")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    if text_limit.len() > 3 {
                        handlev = d.lines().collect::<Vec<_>>()[0].to_string();
                        link = "https://card.syui.ai/".to_owned() + &handlev;
                        e = link.chars().count();
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                        .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if com == "egg"  || com == "/egg" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"egg")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    if text_limit.len() > 3 {
                        handlev = d.lines().collect::<Vec<_>>()[0].to_string();
                        link = "https://card.syui.ai/".to_owned() + &handlev;
                        e = link.chars().count();
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                        .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if com == "nyan" || com == "/nyan" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"nyan")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    println!("{}", text_limit);
                    if text_limit.len() > 3 {
                        let str_rep = reply::post_request(
                            text_limit.to_string(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                        .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if com == "ten" || com == "/ten" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"ten")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    handlev = d.lines().collect::<Vec<_>>()[0].to_string();
                    let ten_l = d.lines().collect::<Vec<_>>().len();
                    println!("handlev {}", handlev);
                    println!("ten_l {}", ten_l);
                    if ten_l == 3 {
                        ten_p = d.lines().collect::<Vec<_>>()[1];
                        println!("ten_p {}", ten_p);
                    }
                    if ten_p != "true" {
                        link = "https://card.syui.ai/".to_owned() + &handlev;
                        e = link.chars().count();
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                            .await;
                        println!("{}", str_rep);
                    }
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                } else if com == "coin" || com == "/coin" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"coin")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    handlev = d.lines().collect::<Vec<_>>()[0].to_string();
                    link = "https://card.syui.ai/".to_owned() + &handlev;
                    println!("{}", e);
                    e = link.chars().count();
                    if text_limit.len() > 3 {
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                        .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if com == "quiz" || com == "/quiz" {
                    println!("admin:{}", admin);
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"quiz")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let d = d.to_string();
                    let text_limit = c_char(d);
                    let str_rep = reply::post_request(
                        text_limit.to_string(),
                        cid.to_string(),
                        uri.to_string(),
                        cid_root.to_string(),
                        uri_root.to_string(),
                    )
                    .await;
                    println!("{}", str_rep);
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                } else if check_uri_comment == true {
                    println!("admin:{}", admin);
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"comment")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&avatar)
                        .arg(&prompt_chat)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let d = d.to_string();
                    let text_limit = c_char(d);
                    let str_rep = reply::post_request(
                        text_limit.to_string(),
                        cid.to_string(),
                        uri.to_string(),
                        cid_root.to_string(),
                        uri_root.to_string(),
                    )
                    .await;
                    println!("{}", str_rep);
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                } else if { com == "sh" || com == "/sh" } && handle == &admin {
                    println!("admin:{}", admin);
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"sh")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let d = d.to_string();
                    let text_limit = c_char(d);
                    let str_rep = reply::post_request(
                        text_limit.to_string(),
                        cid.to_string(),
                        uri.to_string(),
                        cid_root.to_string(),
                        uri_root.to_string(),
                    )
                    .await;
                    println!("{}", str_rep);
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                } else if { com == "mitractl" || com == "/mitractl" } && handle == &admin {
                    println!("admin:{}", admin);
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"mitra")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let d = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(d);
                    link = "https://m.syu.is".to_string();
                    e = link.chars().count();
                    if text_limit.len() > 3 {
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                        .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if { com == "comment" || com == "/comment" } && { handle == &admin || handle == "yui.bsky.social" } && check_uri_comment == false {
                    println!("admin:{}", admin);
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"comment")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&avatar)
                        .arg(&prompt_chat)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let d = d.to_string();
                    let text_limit = c_char(d);
                    let str_rep = reply::post_request(
                        text_limit.to_string(),
                        cid.to_string(),
                        uri.to_string(),
                        cid_root.to_string(),
                        uri_root.to_string(),
                    )
                        .await;
                    println!("{}", str_rep);
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                    w_cid(uri_root.to_string(), log_file(&"c1"), true);
                } else {
                    // openai
                    let str_openai = openai::post_request(prompt_chat.to_string()).await;
                    let text_limit = c_char(str_openai);
                    let str_rep = reply::post_request(
                        text_limit.to_string(),
                        cid.to_string(),
                        uri.to_string(),
                        cid_root.to_string(),
                        uri_root.to_string(),
                    )
                        .await;
                    println!("{}", str_rep);
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                }
                let str_notify = notify_read::post_request(time.to_string()).await;
                println!("{}", str_notify);
                println!("---");
            }
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

pub fn c_bot_feed(c: &Context) {
    let mut feed = "at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.generator/cmd".to_string();
    if c.string_flag("feed").is_ok() {
        feed = c.string_flag("feed").unwrap();
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
            let check_cid = w_cid(cid.to_string(), log_file(&"n1"), false);
            let check_cid_run = w_cid(cid.to_string(), log_file(&"n2"), false);
            //let mut avatar = "".to_string();
            //if let Some(v) = &n[i].post.author.avatar {
            //    avatar = v.to_string();
            //}
            let mut text = "";
            if !n[i].post.record.text.is_none() {
                text = &n[i].post.record.text.as_ref().unwrap();
            }
            //let mut reason = false;
            //if !n[i].post.record.reply.is_none() {
            //    reason = true;
            //}
            let vec: Vec<&str> = text.split_whitespace().collect();
            let handlev: Vec<&str> = handle.split('.').collect();
            let mut handlev = handlev[0].trim().to_string();
            let mut ten_p = "false";

            let mut link = "https://card.syui.ai/".to_owned() + &handlev;
            let s = 0;
            let mut e = link.chars().count();

            let com = vec[0].trim().to_string();
            let mut prompt = "".to_string();
            let mut prompt_sub = "".to_string();
            let mut prompt_chat = "".to_string();
            let mut prompt_all = "".to_string();

            if com == "@ai" || com == "/ai" {
                prompt_chat = vec[1..].join(" ");
            } else {
                prompt = vec[1..].join(" ");
                prompt_all = vec[0..].join(" ");
                if vec.len() > 1 {
                    prompt_sub = vec[2..].join(" ");
                }
            }

            if prompt.is_empty() == false || com.is_empty() == false {
                println!("{}", handle);
                println!(
                    "cid:{}\nuri:{}\ncid_root:{}\nuri_root:{}\nhost:{}",
                    cid, uri, cid_root, uri_root, host
                );
                println!("prompt_sub:{}", prompt_sub);
            }

            let mut admin = "".to_string();
            if c.string_flag("admin").is_ok() {
                admin = c.string_flag("admin").unwrap();
            }

            if check_cid == false
            || check_cid_run == false
            {
                w_cid(cid.to_string(), log_file(&"n2"), true);
                if com == "did" || com == "/did" {
                    let link = "https://plc.directory/".to_owned() + &did + &"/log";
                    let s = 0;
                    let e = link.chars().count();
                    let d = "\n".to_owned() + &did.to_string();
                    let text_limit = c_char(d);
                    if text_limit.len() > 3 {
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                            .await;
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                        println!("{}", str_rep);
                    }
                } else if com == "help" || com == "/help" {
                    let link = "https://git.syui.ai/ai/bot/wiki/help".to_string();
                    let s = 0;
                    let e = link.chars().count();
                    let str_rep = reply_link::post_request(
                        "\n".to_string(),
                        link.to_string(),
                        s,
                        e.try_into().unwrap(),
                        cid.to_string(),
                        uri.to_string(),
                        cid_root.to_string(),
                        uri_root.to_string(),
                    )
                        .await;
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                    println!("{}", str_rep);
                } else if { com == "diffusers"  || com == "/diffusers" } && handle == &admin{
                    let _output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"diffusers")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                } else if com.contains("/占") == true
                    || com.contains("/うらない") == true
                        || com.contains("/うらなって") == true
                {
                    let _output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"fortune")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                } else if prompt_all.contains("アイ") == true
                    || prompt_all.contains("うらな") == true
                        || prompt_all.contains("占") == true
                {
                    if prompt_all.contains("うらな") == true || prompt_all.contains("占") == true {
                        let _output = Command::new(data_scpt(&"ai"))
                            .arg(&"atproto").arg(&"fortune")
                            .arg(&handle)
                            .arg(&did)
                            .arg(&cid)
                            .arg(&uri)
                            .arg(&cid_root)
                            .arg(&uri_root)
                            .arg(&host)
                            .arg(&prompt)
                            .arg(&prompt_sub)
                            .output()
                            .expect("zsh");
                    }
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                } else if com == "card" || com == "/card" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"card")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    if text_limit.len() > 3 {
                        //handlev = handle.replace(".", "-").to_string();
                        handlev = d.lines().collect::<Vec<_>>()[0].to_string();
                        link = "https://card.syui.ai/".to_owned() + &handlev;
                        e = link.chars().count();
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                            .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if com == "fav" || com == "/fav" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"fav")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    if text_limit.len() > 3 {
                        handlev = d.lines().collect::<Vec<_>>()[0].to_string();
                        link = "https://card.syui.ai/".to_owned() + &handlev;
                        e = link.chars().count();
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                            .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if com == "egg"  || com == "/egg" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"egg")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    if text_limit.len() > 3 {
                        handlev = d.lines().collect::<Vec<_>>()[0].to_string();
                        link = "https://card.syui.ai/".to_owned() + &handlev;
                        e = link.chars().count();
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                            .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if com == "nyan" || com == "/nyan" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"nyan")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    println!("{}", text_limit);
                    if text_limit.len() > 3 {
                        let str_rep = reply::post_request(
                            text_limit.to_string(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                            .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if com == "ten" || com == "/ten" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"ten")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    handlev = d.lines().collect::<Vec<_>>()[0].to_string();
                    let ten_l = d.lines().collect::<Vec<_>>().len();
                    println!("handlev {}", handlev);
                    println!("ten_l {}", ten_l);
                    if ten_l == 3 {
                        ten_p = d.lines().collect::<Vec<_>>()[1];
                        println!("ten_p {}", ten_p);
                    }
                    if ten_p != "true" {
                        link = "https://card.syui.ai/".to_owned() + &handlev;
                        e = link.chars().count();
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                            .await;
                        println!("{}", str_rep);
                    }
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                } else if com == "coin" || com == "/coin" {
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"coin")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let dd = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(dd);
                    handlev = d.lines().collect::<Vec<_>>()[0].to_string();
                    link = "https://card.syui.ai/".to_owned() + &handlev;
                    println!("{}", e);
                    e = link.chars().count();
                    if text_limit.len() > 3 {
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                            .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if { com == "sh" || com == "/sh" } && handle == &admin {
                    println!("admin:{}", admin);
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"sh")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let d = d.to_string();
                    let text_limit = c_char(d);
                    let str_rep = reply::post_request(
                        text_limit.to_string(),
                        cid.to_string(),
                        uri.to_string(),
                        cid_root.to_string(),
                        uri_root.to_string(),
                    )
                        .await;
                    println!("{}", str_rep);
                    w_cid(cid.to_string(), log_file(&"n1"), true);

                } else if com == "quiz" || com == "/quiz" {
                    println!("admin:{}", admin);
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"quiz")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let d = d.to_string();
                    let text_limit = c_char(d);
                    let str_rep = reply::post_request(
                        text_limit.to_string(),
                        cid.to_string(),
                        uri.to_string(),
                        cid_root.to_string(),
                        uri_root.to_string(),
                    )
                        .await;
                    println!("{}", str_rep);
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                } else if { com == "mitractl" || com == "/mitractl" } && handle == &admin {
                    println!("admin:{}", admin);
                    let output = Command::new(data_scpt(&"ai"))
                        .arg(&"atproto").arg(&"mitra")
                        .arg(&handle)
                        .arg(&did)
                        .arg(&cid)
                        .arg(&uri)
                        .arg(&cid_root)
                        .arg(&uri_root)
                        .arg(&host)
                        .arg(&prompt)
                        .arg(&prompt_sub)
                        .output()
                        .expect("zsh");
                    let d = String::from_utf8_lossy(&output.stdout);
                    let d = "\n".to_owned() + &d.to_string();
                    let text_limit = c_char(d);
                    link = "https://m.syu.is".to_string();
                    e = link.chars().count();
                    if text_limit.len() > 3 {
                        let str_rep = reply_link::post_request(
                            text_limit.to_string(),
                            link.to_string(),
                            s,
                            e.try_into().unwrap(),
                            cid.to_string(),
                            uri.to_string(),
                            cid_root.to_string(),
                            uri_root.to_string(),
                        )
                            .await;
                        println!("{}", str_rep);
                        w_cid(cid.to_string(), log_file(&"n1"), true);
                    }
                } else if com == "@ai" || com == "/ai" {
                    let str_openai = openai::post_request(prompt_chat.to_string()).await;
                    let text_limit = c_char(str_openai);
                    let str_rep = reply::post_request(
                        text_limit.to_string(),
                        cid.to_string(),
                        uri.to_string(),
                        cid_root.to_string(),
                        uri_root.to_string(),
                    )
                        .await;
                    println!("{}", str_rep);
                    w_cid(cid.to_string(), log_file(&"n1"), true);
                }
                println!("---");
            }
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}
