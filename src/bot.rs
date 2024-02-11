use seahorse::Context;

use crate::refresh;
use crate::reply_link;
use crate::notify;
use crate::notify_read;

use crate::data::log_file;
use crate::data::w_cid;
use crate::data::c_char;
use crate::data::Notify as Notify;

pub fn c_bot(c: &Context) {
    let h = async {
        let mut notify = notify::get_request(100).await;
        if notify == "err" {
            refresh(c);
            notify = notify::get_request(100).await;
        }
        let notify: Notify = serde_json::from_str(&notify).unwrap();

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
            if ! n[i].record.reply.is_none() {
                cid_root = &n[i].record.reply.as_ref().unwrap().root.cid;
                uri_root = &n[i].record.reply.as_ref().unwrap().root.uri;
            }
            println!("{}", read);
            println!("{}", handle);
            println!("{} {}", cid, uri);
            let mut text = "";
            if ! n[i].record.text.is_none() { 
                text = &n[i].record.text.as_ref().unwrap();
            }
            let vec: Vec<&str> = text.split_whitespace().collect();
            let rep_com = &vec[0..].join(" ");

            if check_cid == false && { reason == "mention" || reason == "reply" } || check_cid_run == false && { reason == "mention" || reason == "reply" } {
                w_cid(cid.to_string(), log_file(&"n2"), true);
                if rep_com.contains("did") == true || rep_com.contains("/did") == true {
                    let link = "https://plc.directory/".to_owned() + &did + &"/log";
                    let s = 0;
                    let e = link.chars().count();

                    let d = "\n".to_owned() + &did.to_string();
                    let text_limit = c_char(d);

                    if text_limit.len() > 3 {
                        let str_rep = reply_link::post_request(text_limit.to_string(), link.to_string(), s, e.try_into().unwrap(), cid.to_string(), uri.to_string(), cid_root.to_string(), uri_root.to_string()).await;
                        let str_notify = notify_read::post_request(time.to_string()).await;

                        w_cid(cid.to_string(), log_file(&"n1"), true);
                        println!("{}", str_rep);
                        println!("{}", str_notify);
                    }
                }
            }
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res

}
