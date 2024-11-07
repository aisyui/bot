use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;

use crate::ascii::c_ascii;
use crate::bot::c_bot;
use crate::bot::c_bot_feed;
use crate::data::c_follow_all;
use crate::data::c_openai_key;
use crate::data::data_toml;
use crate::data::data_refresh;
use crate::data::url;
use crate::data::w_cfg;
use crate::data::w_refresh;
use crate::feed_watch::c_feed_watch;

use data::ProfileIdentityResolve;

pub mod ascii;
pub mod bot;
pub mod data;
pub mod describe;
pub mod follow;
pub mod followers;
pub mod follows;
pub mod img_reply;
pub mod like;
pub mod mention;
pub mod notify;
pub mod notify_read;
pub mod openai;
pub mod post;
pub mod post_link;
pub mod profile;
pub mod refresh;
pub mod reply;
pub mod reply_link;
pub mod reply_og;
pub mod repost;
pub mod session;
pub mod timeline_author;
pub mod token;
pub mod feed_get;
pub mod feed_watch;
pub mod delete_record;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
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
            Command::new("bot")
            .alias("b")
            .action(bot)
            .flag(
                Flag::new("admin", FlagType::String)
                .alias("a"),
            )
            .flag(
                Flag::new("feed", FlagType::String)
                .alias("f"),
            )
        )
        .command(
            Command::new("feed_watch")
            .action(feed_watch)
            .flag(
                Flag::new("url", FlagType::String)
                .alias("u"),
            )
            .flag(
                Flag::new("tag", FlagType::String)
                .alias("t"),
            )
            .flag(
                Flag::new("debug", FlagType::Bool)
                .alias("d"),
            )
        )
        .command(
            Command::new("follow_all")
            .action(follow_all),
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
            Command::new("feed")
            .description("feed <feed-uri>")
            .alias("f")
            .action(feed)
            )
        .command(
            Command::new("did")
            .description("did <handle>")
            .action(did)
            )
        .command(
            Command::new("post")
            .description("p <text>")
            .alias("p")
            .action(post)
            .flag(
                Flag::new("link", FlagType::String)
                .alias("l"),
            )
        )
        .command(
            Command::new("delete")
            .description("d <rkey> -c <collection>")
            .alias("d")
            .action(delete)
            .flag(
                Flag::new("col", FlagType::String)
                .alias("c"),
            )
        )
        .command(
            Command::new("like")
            .description("like <cid> -u <uri>")
            .action(like)
            .flag(
                Flag::new("uri", FlagType::String)
                .alias("u"),
                )
            )
        .command(
            Command::new("repost")
            .description("repost <cid> -u <uri>")
            .action(repost)
            .flag(
                Flag::new("uri", FlagType::String)
                .alias("u"),
            )
        )
        .command(
            Command::new("reply-og")
            .description("reply-og <text> -c <cid> -u <uri> -i <img> -t <title> -d <description> -l <link>")
            .action(reply_og)
            .flag(
                Flag::new("uri", FlagType::String)
                .alias("u"),
            )
            .flag(
                Flag::new("cid", FlagType::String)
                .alias("c"),
            )
            .flag(
                Flag::new("uri-root", FlagType::String)
            )
            .flag(
                Flag::new("cid-root", FlagType::String)
            )
            .flag(
                Flag::new("link", FlagType::String)
                .alias("l"),
            )
            .flag(
                Flag::new("title", FlagType::String)
                .alias("t"),
            )
            .flag(
                Flag::new("description", FlagType::String)
                .alias("d"),
            )
            .flag(
                Flag::new("img", FlagType::String)
                .alias("i"),
            )
        )
        .command(
            Command::new("reply")
            .description("r <text> -u <uri> -c <cid>")
            .action(reply)
            .flag(
                Flag::new("uri", FlagType::String)
                .alias("u"),
            )
            .flag(
                Flag::new("cid", FlagType::String)
                .alias("c"),
            )
            .flag(
                Flag::new("link", FlagType::String)
                .description("-l <link>")
                .alias("l"),
            )
        )
        .command(
            Command::new("mention")
            .description("@ <handle> -p <text>")
            .alias("@")
            .action(mention)
            .flag(
                Flag::new("post", FlagType::String)
                .alias("p"),
            )
        )
        .command(
            Command::new("follow")
            .description("follow <did>")
            .action(follow)
            .flag(
                Flag::new("follows", FlagType::Bool)
                .alias("s"),
            )
            .flag(
                Flag::new("delete", FlagType::String)
                .alias("d"),
            )
            .flag(
                Flag::new("followers", FlagType::Bool)
                .alias("w"),
            )
            .flag(
                Flag::new("all", FlagType::Bool)
                .alias("a"),
            )
            .flag(
                Flag::new("cursor", FlagType::String)
                .alias("c"),
            )
        )
        .command(
            Command::new("profile")
            .description("pro <handle>")
            .alias("pro")
            .action(profile)
        )
        .command(
            Command::new("img-upload")
            .description("img-upload <img>")
            .action(img_upload)
        )
        .command(
            Command::new("img-post")
            .description("img-post <text> -l <link> -u <uri> -c <cid>")
            .action(img_post)
            .flag(
                Flag::new("link", FlagType::String)
                .alias("l"),
            )
            .flag(
                Flag::new("uri", FlagType::String)
                .alias("u"),
            )
            .flag(
                Flag::new("cid", FlagType::String)
                .alias("c"),
            )
        )
        .command(
            Command::new("openai")
            .description("openai <text>")
            .alias("chat")
            .action(openai)
        )
        .command(
            Command::new("openai-key")
            .description("openai-key <API_KEY>")
            .action(openai_key),
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

fn bot(c: &Context) {
    refresh(c);
    loop {
        c_bot(c);
        c_bot_feed(c);
    }
}

fn feed_watch(c: &Context) {
    refresh(c);
    loop {
        c_feed_watch(c);
    }
}

fn follow_all(_c: &Context) {
    c_follow_all();
}

fn openai_key(c: &Context) {
    c_openai_key(c);
}

fn token(c: &Context) {
    let m = c.args[0].to_string();
    let h = async {
        if let Ok(p) = c.string_flag("password") {
            if let Ok(s) = c.string_flag("server") {
                let res = token::post_request(m.to_string(), p.to_string(), s.to_string()).await;
                w_cfg(&s, &res, &p);
            } else {
                let res =
                    token::post_request(m.to_string(), p.to_string(), "bsky.social".to_string())
                        .await;
                w_cfg(&"bsky.social", &res, &p);
            }
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn refresh(_c: &Context) {
    let h = async {
        let session = session::get_request().await;
        if session == "err" {
            let res = refresh::post_request().await;
            if res == "err" {
                let m = data_toml(&"handle");
                let p = data_toml(&"password");
                let s = data_toml(&"host");
                let res = token::post_request(m.to_string(), p.to_string(), s.to_string()).await;
                w_cfg(&s, &res, &p);
            } else {
                w_refresh(&res);
            }
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn notify(c: &Context) {
    refresh(c);
    let h = async {
        let j = notify::get_request(100).await;
        println!("{}", j);
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn feed(c: &Context) {
    refresh(c);
    let feed_d = "at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.generator/cmd".to_string();
    let h = async {
        if c.args.len() == 0 {
            let j = feed_get::get_request(feed_d).await;
            println!("{}", j);
        } else {
            let j = feed_get::get_request(c.args[0].to_string()).await;
            println!("{}", j);
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
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
    return res;
}

fn timeline(c: &Context) {
    refresh(c);
    let h = async {
        if c.args.len() == 0 {
            let str = timeline_author::get_request(data_toml(&"handle").to_string());
            println!("{}", str.await);
        } else {
            let str = timeline_author::get_request(c.args[0].to_string());
            println!("{}", str.await);
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn post(c: &Context) {
    refresh(c);
    let m = c.args[0].to_string();
    let h = async {
        if let Ok(link) = c.string_flag("link") {
            let e = link.chars().count();
            let s = 0;
            let str =
                post_link::post_request(m.to_string(), link.to_string(), s, e.try_into().unwrap());
            println!("{}", str.await);
        } else {
            let str = post::post_request(m.to_string());
            println!("{}", str.await);
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn delete(c: &Context) {
    refresh(c);
    let m = c.args[0].to_string();
    let h = async {
        if let Ok(col) = c.string_flag("col") {
            let str = delete_record::post_request(m.to_string(), col);
            println!("{}", str.await);
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn like(c: &Context) {
    refresh(c);
    let m = c.args[0].to_string();
    let h = async {
        if let Ok(uri) = c.string_flag("uri") {
            let str = like::post_request(m.to_string(), uri);
            println!("{}", str.await);
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn repost(c: &Context) {
    refresh(c);
    let m = c.args[0].to_string();
    let h = async {
        if let Ok(uri) = c.string_flag("uri") {
            let str = repost::post_request(m.to_string(), uri);
            println!("{}", str.await);
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn follow(c: &Context) {
    refresh(c);
    let m = c.args[0].to_string();
    let h = async {
        let handle = data_toml(&"handle");
        if let Ok(cursor) = c.string_flag("cursor") {
            let str = follow::post_request(m.to_string());
            println!("{}", str.await);
            //let str = follow::delete_request(m.to_string(), delete.to_string());
            //let str = followers::get_request(handle,Some("".to_string()));
            //let str = followers::get_request(handle,Some(cursor.to_string()));
            //let str = follows::get_request(handle,Some("".to_string()));
            let str = follows::get_request(handle, Some(cursor.to_string()));
            println!("{}", str.await);
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn profile(c: &Context) {
    refresh(c);
    let h = async {
        if c.args.len() == 0 {
            let j = profile::get_request(data_toml(&"handle")).await;
            println!("{}", j);
        } else {
            let j = profile::get_request(c.args[0].to_string()).await;
            println!("{}", j);
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn mention(c: &Context) {
    refresh(c);
    let m = c.args[0].to_string();
    let h = async {
        let str = profile::get_request(m.to_string()).await;
        let profile: ProfileIdentityResolve = serde_json::from_str(&str).unwrap();
        let udid = profile.did;
        let handle = m.to_string();
        let at = "@".to_owned() + &handle;
        let e = at.chars().count();
        let s = 0;
        if let Ok(post) = c.string_flag("post") {
            let str = mention::post_request(
                post.to_string(),
                at.to_string(),
                udid.to_string(),
                s,
                e.try_into().unwrap(),
            )
            .await;
            println!("{}", str);
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn reply(c: &Context) {
    refresh(c);
    let m = c.args[0].to_string();
    let h = async {
        if let Ok(cid) = c.string_flag("cid") {
            if let Ok(uri) = c.string_flag("uri") {
                if let Ok(link) = c.string_flag("link") {
                    let s = 0;
                    let e = link.chars().count();
                    let str = reply_link::post_request(
                        m.to_string(),
                        link.to_string(),
                        s,
                        e.try_into().unwrap(),
                        cid.to_string(),
                        uri.to_string(),
                        cid.to_string(),
                        uri.to_string(),
                    )
                    .await;
                    println!("{}", str);
                } else {
                    let str = reply::post_request(
                        m.to_string(),
                        cid.to_string(),
                        uri.to_string(),
                        cid.to_string(),
                        uri.to_string(),
                    )
                    .await;
                    println!("{}", str);
                }
            }
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

#[tokio::main]
async fn c_img_upload(c: &Context) -> reqwest::Result<()> {
    let token = data_refresh(&"access");
    let atoken = "Authorization: Bearer ".to_owned() + &token;
    let con = "Content-Type: image/png";
    let url = url(&"upload_blob");
    let f = "@".to_owned() + &c.args[0].to_string();
    use std::process::Command;
    let output = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg("-sL")
        .arg("-H")
        .arg(&con)
        .arg("-H")
        .arg(&atoken)
        .arg("--data-binary")
        .arg(&f)
        .arg(&url)
        .output()
        .expect("curl");
    let d = String::from_utf8_lossy(&output.stdout);
    let d = d.to_string();
    println!("{}", d);
    Ok(())
}

fn img_upload(c: &Context) {
    refresh(c);
    c_img_upload(c).unwrap();
}

fn img_post(c: &Context) {
    let m = c.args[0].to_string();
    let link = c.string_flag("link").unwrap();
    let cid = c.string_flag("cid").unwrap();
    let uri = c.string_flag("uri").unwrap();
    let h = async {
        let itype = "image/png";
        let str = img_reply::post_request(
            m.to_string(),
            link.to_string(),
            cid.to_string(),
            uri.to_string(),
            itype.to_string(),
        )
        .await;
        println!("{}", str);
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn reply_og(c: &Context) {
    refresh(c);

    //let mut admin = "".to_string();
    //if c.string_flag("admin").is_ok() {
    //    admin = c.string_flag("admin").unwrap();
    //}

    let m = c.args[0].to_string();
    let link = c.string_flag("link").unwrap();
    let cid = c.string_flag("cid").unwrap();
    let uri = c.string_flag("uri").unwrap();
    let cid_root = c.string_flag("cid-root").unwrap();
    let uri_root = c.string_flag("uri-root").unwrap();
    let title = c.string_flag("title").unwrap();
    let desc = c.string_flag("description").unwrap();
    let img = c.string_flag("img").unwrap();
    let h = async {
        let str = reply_og::post_request(m, link, cid, uri, cid_root, uri_root, img, title, desc);
        println!("{}", str.await);
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}

fn openai(c: &Context) {
    let m = c.args[0].to_string();
    let h = async {
        let str = openai::post_request(m.to_string()).await;
        println!("{}", str);
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res;
}
