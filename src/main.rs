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
use crate::game::post_card;
use crate::game::post_card_verify;
use crate::game::post_game;
use crate::game::post_game_user;
use crate::game::post_game_login;

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
pub mod game;
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
            Command::new("card")
            .description("-v <at://verify> -i <int:id> -p <int:cp> -r <int:rank> -c <collection> -a <author> -img <link> -rare <normal>")
            .action(card)
            .flag(
                Flag::new("id", FlagType::Int)
                .alias("i"),
            )
            .flag(
                Flag::new("cp", FlagType::Int)
                .alias("p"),
            )
            .flag(
                Flag::new("rank", FlagType::Int)
                .alias("r"),
            )
            .flag(
                Flag::new("rare", FlagType::Int)
            )
            .flag(
                Flag::new("col", FlagType::String)
                .alias("c"),
            )
            .flag(
                Flag::new("author", FlagType::String)
                .alias("a"),
            )
            .flag(
                Flag::new("verify", FlagType::String)
                .alias("v"),
            )
            .flag(
                Flag::new("img", FlagType::String)
            )
        )
        .command(
            Command::new("card-verify")
            .description("<at://verify> -c <collection> -i <id> -p <cp> -r <rank> -rare <normal> -H <syui.ai> -d <did>")
            .action(card_verify)
            .flag(
                Flag::new("col", FlagType::String)
                .alias("c"),
            )
            .flag(
                Flag::new("id", FlagType::Int)
                .alias("i"),
            )
            .flag(
                Flag::new("cp", FlagType::Int)
                .alias("p"),
            )
            .flag(
                Flag::new("rank", FlagType::Int)
                .alias("r"),
            )
            .flag(
                Flag::new("rare", FlagType::String)
            )
            .flag(
                Flag::new("handle", FlagType::String)
                .alias("H"),
            )
            .flag(
                Flag::new("did", FlagType::String)
                .alias("did"),
            )
        )
        .command(
            Command::new("game")
            .description("a <at://yui.syui.ai/ai.syui.game.user/username>")
            .action(game)
            .flag(
                Flag::new("col", FlagType::String)
                .alias("c"),
            )
            .flag(
                Flag::new("account", FlagType::String)
                .alias("a"),
            )
        )
        .command(
            Command::new("game-login")
            .description("l <bool> -u <username> -c <collection>")
            .action(game_login)
            .flag(
                Flag::new("col", FlagType::String)
                .alias("c"),
            )
            .flag(
                Flag::new("login", FlagType::Bool)
                .alias("l"),
            )
            .flag(
                Flag::new("username", FlagType::String)
                .alias("u"),
            )
        )
        .command(
            Command::new("game-user")
            .description("-chara ai -l 20240101 -ten 0 --lv 0 --exp 0 --hp 0 --rank 0 --mode 0 --attach 0 --critical 0 --critical_d 0")
            .action(game_user)
            .flag(
                Flag::new("username", FlagType::String)
                .alias("u"),
            )
            .flag(
                Flag::new("col", FlagType::String)
                .alias("c"),
            )
            .flag(
                Flag::new("did", FlagType::String)
                .alias("d"),
            )
            .flag(
                Flag::new("handle", FlagType::String)
                .alias("H"),
            )
            .flag(
                Flag::new("character", FlagType::String)
                .alias("chara"),
            )
            .flag(
                Flag::new("aiten", FlagType::Int)
                .alias("ten"),
            )
            .flag(
                Flag::new("limit", FlagType::Int)
                .alias("l"),
            )
            .flag(
                Flag::new("lv", FlagType::Int)
            )
            .flag(
                Flag::new("hp", FlagType::Int)
            )
            .flag(
                Flag::new("attach", FlagType::Int)
            )
            .flag(
                Flag::new("exp", FlagType::Int)
            )
            .flag(
                Flag::new("critical", FlagType::Int)
            )
            .flag(
                Flag::new("critical_d", FlagType::Int)
            )
            .flag(
                Flag::new("rank", FlagType::Int)
            )
            .flag(
                Flag::new("mode", FlagType::Int)
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
            .flag(
                Flag::new("col", FlagType::String)
                .alias("c"),
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

async fn c_card(c: &Context) -> Result<(), Box<dyn std::error::Error>> {
    //let m = c.args[0].to_string();
    let author = c.string_flag("author").unwrap_or_else(|_| "syui".to_string());
    let verify = c.string_flag("verify").unwrap_or_else(|_| "at://did:plc:4hqjfn7m6n5hno3doamuhgef/ai.syui.card.verify/3lagpvhppmd2q".to_string());
    let col = c.string_flag("col").unwrap_or_else(|_| "ai.syui.card".to_string());
    //let img = c.string_flag("img").unwrap_or_else(|_| "bafkreigvcjc46qtelpc4wsg7fwf6qktbi6a23ouqiupth2r37zhrn7wbza".to_string());
    let id = c.int_flag("id")?.try_into()?;
    let cp = c.int_flag("cp")?.try_into()?;
    let rank = c.int_flag("rank")?.try_into()?;
    let rare = c.string_flag("rare").unwrap_or_else(|_| "normal".to_string());
    let str = post_card::post_request(verify, id, cp, rank, rare, col, author);
    println!("{}", str.await);
    Ok(())
}

fn card(c: &Context) {
    refresh(c);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            if let Err(e) = c_card(c).await {
                eprintln!("Error: {}", e);
            }
        });
}

async fn c_card_verify(c: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let col = c.string_flag("col").unwrap_or_else(|_| "ai.syui.card.verify".to_string());
    let img = c.string_flag("img").unwrap_or_else(|_| "bafkreigvcjc46qtelpc4wsg7fwf6qktbi6a23ouqiupth2r37zhrn7wbza".to_string());
    let id = c.int_flag("id")?.try_into()?;
    let cp = c.int_flag("cp")?.try_into()?;
    let rank = c.int_flag("rank")?.try_into()?;
    let rare = c.string_flag("rare").unwrap_or_else(|_| "normal".to_string());
    let user_handle = c.string_flag("handle").unwrap_or_else(|_| "syui.ai".to_string());
    let user_did = c.string_flag("did").unwrap_or_else(|_| "did:plc:uqzpqmrjnptsxezjx4xuh2mn".to_string());

    //match id === 1 let img = "xxx";
    let str = post_card_verify::post_request(col, img, id, cp, rank, rare, user_handle, user_did);
    println!("{}", str.await);
    Ok(())
}

fn card_verify(c: &Context) {
    refresh(c);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            if let Err(e) = c_card_verify(c).await {
                eprintln!("Error: {}", e);
            }
        });
}

async fn c_game(c: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let account = c.string_flag("account").unwrap_or_else(|_| "at://did:plc:4hqjfn7m6n5hno3doamuhgef/ai.syui.game.user/syui".to_string());
    let col = c.string_flag("col").unwrap_or_else(|_| "ai.syui.game".to_string());
    let handle = data_toml(&"handle");
    if handle == "syui.ai" {
        let str = post_game::post_request(col, account);
        println!("{}", str.await);
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Not authorized")))
    }
}

fn game(c: &Context) {
    refresh(c);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            if let Err(e) = c_game(c).await {
                eprintln!("Error: {}", e);
            }
        });
}

async fn c_game_user(c: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let col = c.string_flag("col").unwrap_or_else(|_| "ai.syui.game.user".to_string());
    let user_name = c.string_flag("username").unwrap_or_else(|_| "syui".to_string());
    let user_handle = c.string_flag("handle").unwrap_or_else(|_| "syui.ai".to_string());
    let user_did = c.string_flag("did").unwrap_or_else(|_| "did:plc:uqzpqmrjnptsxezjx4xuh2mn".to_string());
    let chara = c.string_flag("character").unwrap_or_else(|_| "ai".to_string());
    let limit = c.int_flag("limit")?.try_into()?;
    let aiten = c.int_flag("aiten")?.try_into()?;
    let lv = c.int_flag("lv")?.try_into()?;
    let exp = c.int_flag("exp")?.try_into()?;
    let hp = c.int_flag("hp")?.try_into()?;
    let rank = c.int_flag("rank")?.try_into()?;
    let mode = c.int_flag("mode")?.try_into()?;
    let attach = c.int_flag("attach")?.try_into()?;
    let critical = c.int_flag("critical")?.try_into()?;
    let critical_d = c.int_flag("critical_d")?.try_into()?;

    if data_toml(&"handle") == "yui.syui.ai" {
        let str = post_game_user::post_request(col, user_name, user_did, user_handle, aiten, limit, chara, lv, exp, hp, rank, mode, attach, critical, critical_d);
        println!("{}", str.await);
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Not authorized")))
    }
}

fn game_user(c: &Context) {
    refresh(c);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            if let Err(e) = c_game_user(c).await {
                eprintln!("Error: {}", e);
            }
        });
}

async fn c_game_login(c: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let col = c.string_flag("col").unwrap_or_else(|_| "ai.syui.game.login".to_string());
    let user_name = c.string_flag("username").unwrap_or_else(|_| "syui".to_string());
    let account = "at://did:plc:4hqjfn7m6n5hno3doamuhgef/ai.syui.game.user/".to_string() + &user_name;
    let login = c.bool_flag("login");
    if data_toml(&"handle") == "yui.syui.ai" {
        let str = post_game_login::post_request(col, user_name, login, account);
        println!("{}", str.await);
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Not authorized")))
    }
}

fn game_login(c: &Context) {
    refresh(c);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            if let Err(e) = c_game_login(c).await {
                eprintln!("Error: {}", e);
            }
        });
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
        let col = c.string_flag("col").unwrap_or_else(|_| "app.bsky.feed.post".to_string());
        let udid = profile.did;
        let handle = m.to_string();
        let at = "@".to_owned() + &handle;
        let e = at.chars().count();
        let s = 0;
        if let Ok(post) = c.string_flag("post") {
            let str = mention::post_request(
                col,
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
