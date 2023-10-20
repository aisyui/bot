
use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;

use crate::ascii::c_ascii;
use crate::data::data_toml;
use crate::data::log_file;
use crate::data::url;
use crate::data::w_cfg;
use crate::data::w_cid;
use crate::data::c_char;
use data::Notify as Notify;

pub mod ascii;
pub mod data;
pub mod refresh;
pub mod token;
pub mod session;
pub mod notify;
pub mod notify_read;
pub mod reply;
pub mod reply_link;
pub mod describe;
pub mod timeline_author;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .command(
            Command::new("ai")
            .alias("a")
            .action(ascii_art)
            .flag(
                Flag::new("type", FlagType::String)
                .description("type flag")
                .alias("t"),
            )
        )
        .command(
            Command::new("login")
            .alias("l")
            .description("l <handle> -p <password>\n\t\t\tl <handle> -p <password> -s <server>")
            .action(token)
            .flag(
                Flag::new("password", FlagType::String)
                .description("password flag")
                .alias("p"),
                )
            .flag(
                Flag::new("server", FlagType::String)
                .description("server flag")
                .alias("s"),
            )
        )
        .command(
            Command::new("refresh")
            .alias("r")
            .action(refresh),
        )
        .command(
            Command::new("notify")
            .alias("n")
            .action(notify),
            )
        .command(
            Command::new("timeline")
            .alias("t")
            .action(timeline),
        )
        .command(
            Command::new("did")
            .description("did <handle>")
            .action(did)
            )
        //.command(
        //    Command::new("like")
        //    .description("$ ai like <cid>\n\t\t\t$ ai like <cid> -u <uri>")
        //    .action(c_like)
        //    .flag(
        //        Flag::new("uri", FlagType::String)
        //        .alias("u"),
        //        )
        //    )
        //.command(
        //    Command::new("repost")
        //    .description("$ ai repost <cid>\n\t\t\t$ ai repost <cid> -u <uri>")
        //    .action(c_repost)
        //    .flag(
        //        Flag::new("uri", FlagType::String)
        //        .alias("u"),
        //        )
        //)
        //.command(
        //    Command::new("reply-og")
        //    .description("$ ai reply-og\n\t\t\t$ ai reply-og <text> -c <cid> -u <uri> -i <img> -t <title> -d <description> -l <link>")
        //    .action(reply_og)
        //    .flag(
        //        Flag::new("uri", FlagType::String)
        //        .alias("u"),
        //    )
        //    .flag(
        //        Flag::new("cid", FlagType::String)
        //        .alias("c"),
        //    )
        //    .flag(
        //        Flag::new("link", FlagType::String)
        //        .alias("l"),
        //    )
        //    .flag(
        //        Flag::new("title", FlagType::String)
        //        .alias("t"),
        //    )
        //    .flag(
        //        Flag::new("description", FlagType::String)
        //        .alias("d"),
        //    )
        //    .flag(
        //        Flag::new("img", FlagType::String)
        //        .alias("i"),
        //    )
        //    )
        //    .command(
        //        Command::new("handle")
        //        .usage("atr h")
        //        .description("handle update\n\t\t\t$ atr -h example.com\n\t\t\t$ atr -h user.bsky.social")
        //        .alias("h")
        //        .action(c_handle)
        //    )
        //    .command(
        //        Command::new("feed")
        //        .usage("atr f")
        //        .description("feed user\n\t\t\t$ atr f\n\t\t\t$ atr f -u user.bsky.social")
        //        .alias("f")
        //        .action(c_feed)
        //        .flag(
        //            Flag::new("user", FlagType::String)
        //            .description("user flag(ex: $ atr f -u user)")
        //            .alias("u"),
        //        )
        //    )
        //    .command(
        //        Command::new("post")
        //        .description("$ ai p <text>\n\t\t\t$ ai p <text> -l https://syui.ai")
        //        .alias("p")
        //        .action(c_post)
        //        .flag(
        //            Flag::new("link", FlagType::String)
        //            .description("link flag(ex: $ atr p -l)")
        //            .alias("l"),
        //        )
        //        .flag(
        //            Flag::new("cid", FlagType::String)
        //            .description("link flag(ex: $ atr p -l)")
        //        )
        //        .flag(
        //            Flag::new("uri", FlagType::String)
        //            .description("link flag(ex: $ atr p -l)")
        //        )
        //    )
        //    .command(
        //        Command::new("reply")
        //        .usage("atr r {}")
        //        .description("reply\n\t\t\t$ atr r $text -u $uri -c $cid")
        //        .action(c_reply)
        //        .flag(
        //            Flag::new("uri", FlagType::String)
        //            .description("uri flag(ex: $ atr r -u)")
        //            .alias("u"),
        //        )
        //        .flag(
        //            Flag::new("cid", FlagType::String)
        //            .description("cid flag(ex: $ atr r -u -c)")
        //            .alias("c"),
        //        )
        //        .flag(
        //            Flag::new("link", FlagType::String)
        //            .description("link flag(ex: $ atr r $text -u $uri -c $cid -l $link)")
        //            .alias("l"),
        //        )
        //        )
        //        .command(
        //            Command::new("mention")
        //            .usage("atr mention {}")
        //            .description("mention\n\t\t\t$ atr @ syui.bsky.social -p $text")
        //            .alias("@")
        //            .action(c_mention)
        //            .flag(
        //                Flag::new("post", FlagType::String)
        //                .description("post flag\n\t\t\t$ atr @ syui.bsky.social -p text")
        //                .alias("p"),
        //            )
        //        )

                .command(
                    Command::new("bot")
                    .alias("b")
                    .action(bot),
                )
                ;
    app.run(args);
}

fn ascii_art(c: &Context) {
    if let Ok(t) = c.string_flag("type") {
        c_ascii(&t);
    } else {
        c_ascii("color");
    }
}

fn token(c: &Context) {
    let m = c.args[0].to_string();
    let h = async {
        if let Ok(p) = c.string_flag("password") {
            if let Ok(s) = c.string_flag("server") {
                let res = token::post_request(m.to_string(), p.to_string(), s.to_string()).await;
                w_cfg(&s, &res)
            } else {
                let res = token::post_request(m.to_string(), p.to_string(), "bsky.social".to_string()).await;
                w_cfg(&"bsky.social", &res)
            }
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res
}

fn refresh(_c: &Context) {
    let server = data_toml(&"host");
    let h = async {
        let session = session::get_request().await;
        if session == "err" {
            let res = refresh::post_request().await;
            println!("{}", res);
            w_cfg(&server, &res)
        } else {
            println!("no refresh");
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res
}

fn notify(c: &Context) {
    refresh(c);
    let h = async {
        let j = notify::get_request(100).await;
        println!("{}", j);
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res
}

fn did(c: &Context) {
    refresh(c);
    let h = async {
        if c.args.len() == 0 {
            let j = describe::get_request(data_toml(&"handle")).await;
            println!("{}", j);
        } else {
            let j = describe::get_request(c.args[0].to_string()).await;
            println!("{}", j);
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res
}

fn timeline(c: &Context) {
    refresh(c);
    let h = async {
        if c.args.len() == 0 {
            let str = timeline_author::get_request(data_toml(&"handle").to_string());
            println!("{}",str.await);    
        } else {
            let str = timeline_author::get_request(c.args[0].to_string());
            println!("{}",str.await);    
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res
}

fn c_bot(c: &Context) {
    
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

fn bot(c: &Context) {
    loop {
        c_bot(c);
    }
}
