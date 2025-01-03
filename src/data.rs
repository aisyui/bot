use config::{Config, ConfigError, File};
use seahorse::Context;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn data_file(s: &str) -> String {
    let file = "/.config/ai/";
    let mut f = shellexpand::tilde("~").to_string();
    f.push_str(&file);
    let path = Path::new(&f);
    if path.is_dir() == false {
        let _ = fs::create_dir_all(f.clone());
    }
    match &*s {
        "toml" => f + &"token.toml",
        "json" => f + &"token.json",
        "refresh" => f + &"refresh.toml",
        _ => f + &"." + &s,
    }
}

pub fn log_file(s: &str) -> String {
    let file = "/.config/ai/txt/";
    let mut f = shellexpand::tilde("~").to_string();
    f.push_str(&file);
    let path = Path::new(&f);
    if path.is_dir() == false {
        let _ = fs::create_dir_all(f.clone());
    }
    match &*s {
        "n1" => f + &"notify_cid.txt",
        "n2" => f + &"notify_cid_run.txt",
        "c1" => f + &"comment_cid.txt",
        _ => f + &s,
    }
}

impl Token {
    pub fn new() -> Result<Self, ConfigError> {
        let d = data_file("json");
        let s = Config::builder()
            .add_source(File::with_name(&d))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        s.try_deserialize()
    }
}

impl Data {
    pub fn new() -> Result<Self, ConfigError> {
        let d = data_file("toml");
        let s = Config::builder()
            .add_source(File::with_name(&d))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        s.try_deserialize()
    }
}

impl Refresh {
    pub fn new() -> Result<Self, ConfigError> {
        let d = data_file("refresh");
        let s = Config::builder()
            .add_source(File::with_name(&d))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        s.try_deserialize()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Token {
    pub did: String,
    pub handle: String,
    pub accessJwt: String,
    pub refreshJwt: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Data {
    pub host: String,
    pub password: String,
    pub did: String,
    pub handle: String,
    pub access: String,
    pub refresh: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Refresh {
    pub access: String,
    pub refresh: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseUrl {
    pub profile_get: String,
    pub thread_get: String,
    pub describe: String,
    pub record_list: String,
    pub record_create: String,
    pub record_delete: String,
    pub record_put: String,
    pub session_create: String,
    pub session_refresh: String,
    pub session_get: String,
    pub timeline_get: String,
    pub timeline_author: String,
    pub upload_blob: String,
    pub update_handle: String,
    pub account_create: String,
    pub notify_count: String,
    pub notify_list: String,
    pub notify_update: String,
    pub repo_update: String,
    pub like: String,
    pub repost: String,
    pub follow: String,
    pub follows: String,
    pub followers: String,
    pub feed_get: String,
}

pub fn url(s: &str) -> String {
    let s = String::from(s);
    let data = Data::new().unwrap();
    let data = Data {
        host: data.host,
        password: data.password,
        handle: data.handle,
        did: data.did,
        access: data.access,
        refresh: data.refresh,
    };
    let t = "https://".to_string() + &data.host.to_string() + &"/xrpc/".to_string();
    let baseurl = BaseUrl {
        profile_get: "com.atproto.identity.resolveHandle".to_string(),
        thread_get: "app.bsky.feed.getPostThread".to_string(),
        record_put: "com.atproto.repo.putRecord".to_string(),
        record_create: "com.atproto.repo.createRecord".to_string(),
        record_delete: "com.atproto.repo.deleteRecord".to_string(),
        describe: "com.atproto.repo.describeRepo".to_string(),
        record_list: "com.atproto.repo.listRecords".to_string(),
        session_create: "com.atproto.server.createSession".to_string(),
        session_refresh: "com.atproto.server.refreshSession".to_string(),
        session_get: "com.atproto.server.getSession".to_string(),
        timeline_get: "app.bsky.feed.getTimeline".to_string(),
        feed_get: "app.bsky.feed.getFeed".to_string(),
        timeline_author: "app.bsky.feed.getAuthorFeed".to_string(),
        like: "app.bsky.feed.like".to_string(),
        repost: "app.bsky.feed.repost".to_string(),
        follow: "app.bsky.graph.follow".to_string(),
        follows: "app.bsky.graph.getFollows".to_string(),
        followers: "app.bsky.graph.getFollowers".to_string(),
        upload_blob: "com.atproto.repo.uploadBlob".to_string(),
        account_create: "com.atproto.server.createAccount".to_string(),
        update_handle: "com.atproto.identity.updateHandle".to_string(),
        notify_count: "app.bsky.notification.getUnreadCount".to_string(),
        notify_list: "app.bsky.notification.listNotifications".to_string(),
        notify_update: "app.bsky.notification.updateSeen".to_string(),
        repo_update: "com.atproto.sync.updateRepo".to_string(),
    };

    match &*s {
        "profile_get" => t.to_string() + &baseurl.profile_get,
        "thread_get" => t.to_string() + &baseurl.thread_get,
        "describe" => t.to_string() + &baseurl.describe,
        "record_list" => t.to_string() + &baseurl.record_list,
        "record_create" => t.to_string() + &baseurl.record_create,
        "record_delete" => t.to_string() + &baseurl.record_delete,
        "record_put" => t.to_string() + &baseurl.record_put,
        "session_create" => t.to_string() + &baseurl.session_create,
        "session_refresh" => t.to_string() + &baseurl.session_refresh,
        "session_get" => t.to_string() + &baseurl.session_get,
        "timeline_get" => t.to_string() + &baseurl.timeline_get,
        "timeline_author" => t.to_string() + &baseurl.timeline_get,
        "upload_blob" => t.to_string() + &baseurl.upload_blob,
        "account_create" => t.to_string() + &baseurl.account_create,
        "update_handle" => t.to_string() + &baseurl.update_handle,
        "notify_list" => t.to_string() + &baseurl.notify_list,
        "notify_count" => t.to_string() + &baseurl.notify_count,
        "notify_update" => t.to_string() + &baseurl.notify_update,
        "repo_update" => t.to_string() + &baseurl.repo_update,
        "like" => t.to_string() + &baseurl.like,
        "repost" => t.to_string() + &baseurl.repost,
        "follow" => t.to_string() + &baseurl.follow,
        "follows" => t.to_string() + &baseurl.follows,
        "followers" => t.to_string() + &baseurl.followers,
        "feed_get" => t.to_string() + &baseurl.feed_get,
        _ => s,
    }
}

pub fn data_toml(s: &str) -> String {
    let s = String::from(s);
    let data = Data::new().unwrap();
    let data = Data {
        host: data.host,
        password: data.password,
        handle: data.handle,
        did: data.did,
        access: data.access,
        refresh: data.refresh,
    };
    match &*s {
        "host" => data.host,
        "password" => data.password,
        "handle" => data.handle,
        "did" => data.did,
        "access" => data.access,
        "refresh" => data.refresh,
        _ => s,
    }
}

pub fn c_refresh(access: &str, refresh: &str) {
    let ff = data_file(&"refresh");
    let mut ff = fs::File::create(ff.clone()).unwrap();
    let refreshs = Refresh {
        access: access.to_string(),
        refresh: refresh.to_string(),
    };
    let toml = toml::to_string(&refreshs).unwrap();
    ff.write_all(&toml.as_bytes()).unwrap();
}

pub fn data_refresh(s: &str) -> String {
    let s = String::from(s);

    let data = Data::new().unwrap();
    let data = Data {
        host: data.host,
        password: data.password,
        handle: data.handle,
        did: data.did,
        access: data.access,
        refresh: data.refresh,
    };

    let mut _file = match Refresh::new()
    {
        Err(_why) => c_refresh(&data.access, &data.refresh),
        Ok(_) => println!(""),
    };
    let refresh = Refresh::new().unwrap();
    let refresh = Refresh {
        access: refresh.access,
        refresh: refresh.refresh,
    };
    match &*s {
        "access" => refresh.access,
        "refresh" => refresh.refresh,
        _ => s,
    }
}

pub fn data_scpt(s: &str) -> String {
    let s = String::from(s);
    let file = "/.config/ai/scpt/".to_owned() + &s + &".zsh";
    let mut f = shellexpand::tilde("~").to_string();
    f.push_str(&file);
    return f;
}

#[derive(Serialize, Deserialize)]
pub struct Notify {
    pub notifications: Vec<Notifications>,
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub handle: String,
    pub did: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct DidDocs {
    pub verificationMethod: Vec<VerificationMethod>,
    pub service: Vec<Service>,
    pub id: String,
    pub alsoKnownAs: Vec<AlsoKnownAs>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct VerificationMethod {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    pub publicKeyMultibase: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Service {
    pub id: String,
    pub r#type: String,
    pub serviceEndpoint: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AlsoKnownAs {}

#[derive(Serialize, Deserialize)]
pub struct Timeline {
    pub feed: Vec<Feed>,
}
#[derive(Serialize, Deserialize)]
pub struct Session {
    pub did: String,
    pub email: String,
    pub handle: String,
}
#[derive(Serialize, Deserialize)]
pub struct Follow {
    pub follows: Vec<Author>,
    pub cursor: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Notifications {
    pub uri: String,
    pub cid: String,
    pub author: Author,
    pub reason: String,
    //pub reasonSubject: String,
    pub record: Record,
    pub isRead: bool,
    pub indexedAt: String,
    //pub labels: Labels,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Thread {
    pub r#type: String,
    pub post: String,
    pub root: String,
    pub author: Author,
    pub reason: String,
    //pub reasonSubject: String,
    pub record: Record,
    pub isRead: bool,
    pub indexedAt: String,
    //pub labels: Labels,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Author {
    pub did: String,
    //pub declaration: Declaration,
    pub description: Option<String>,
    pub displayName: Option<String>,
    pub handle: String,
    pub avatar: Option<String>,
    pub viewer: Viewer,
    //pub labels: Labels,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Labels {
    pub src: Option<String>,
    pub uri: Option<String>,
    pub cid: Option<String>,
    pub val: Option<String>,
    pub cts: Option<String>,
    pub neg: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Declaration {
    pub actorType: String,
    pub cid: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Viewer {
    pub muted: bool,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Record {
    pub text: Option<String>,
    pub createdAt: String,
    pub reply: Option<Reply>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Reply {
    pub parent: ReplyParent,
    pub root: ReplyRoot,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct ReplyRoot {
    pub cid: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct ReplyParent {
    pub cid: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Langs {}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Feed {
    pub post: Post,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Post {
    pub did: Option<String>,
    pub uri: String,
    pub cid: String,
    pub collection: Option<String>,
    pub record: Record,
    pub author: Author,
    pub reason: Option<String>,
    pub indexedAt: String,
    pub replyCount: i32,
    pub postCount: Option<i32>,
    pub repostCount: i32,
    pub likeCount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Cid {
    pub cid: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Img {
    pub blob: Blob,
}

#[derive(Serialize, Deserialize)]
pub struct Blob {
    pub r#ref: Ref,
}

#[derive(Serialize, Deserialize)]
pub struct Ref {
    pub link: String,
}

#[derive(Serialize, Deserialize)]
pub struct Handle {
    pub handle: String,
}

//#[derive(Serialize, Deserialize)]
//pub struct Did {
//    pub did: String
//}

//#[derive(Serialize, Deserialize)]
//pub struct Labels {
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct Viewer {
//    pub muted: bool,
//    pub blockedBy: bool,
//}
//
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ProfileIdentityResolve {
    pub did: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Profile {
    pub did: String,
    pub handle: String,
    pub followsCount: Option<i32>,
    pub followersCount: Option<i32>,
    pub postsCount: i32,
    pub indexedAt: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub displayName: Option<String>,
    pub description: Option<String>,
    pub viewer: Viewer,
    pub labels: Labels,
}

pub fn c_char(i: String) -> String {
    let l = 250;
    let mut s = String::new();
    for ii in i.chars().enumerate() {
        match ii.0 {
            n if n > l.try_into().unwrap() => break,
            _ => s.push(ii.1),
        }
    }
    return s;
}

pub fn w_cfg(h: &str, res: &str, password: &str) {
    let f = data_file(&"json");
    let ff = data_file(&"toml");
    let mut f = fs::File::create(f.clone()).unwrap();
    let mut ff = fs::File::create(ff.clone()).unwrap();
    f.write_all(&res.as_bytes()).unwrap();
    let json: Token = serde_json::from_str(&res).unwrap();
    let datas = Data {
        host: h.to_string(),
        password: password.to_string(),
        did: json.did.to_string(),
        handle: json.handle.to_string(),
        access: json.accessJwt.to_string(),
        refresh: json.refreshJwt.to_string(),
    };
    let toml = toml::to_string(&datas).unwrap();
    ff.write_all(&toml.as_bytes()).unwrap();

    let ff = data_file(&"refresh");
    let mut ff = fs::File::create(ff.clone()).unwrap();
    let refreshs = Refresh {
        access: json.accessJwt.to_string(),
        refresh: json.refreshJwt.to_string(),
    };
    let toml = toml::to_string(&refreshs).unwrap();
    ff.write_all(&toml.as_bytes()).unwrap();
}

pub fn w_refresh(res: &str) {
    let ff = data_file(&"refresh");
    let mut ff = fs::File::create(ff.clone()).unwrap();
    let json: Token = serde_json::from_str(&res).unwrap();
    let refreshs = Refresh {
        access: json.accessJwt.to_string(),
        refresh: json.refreshJwt.to_string(),
    };
    let toml = toml::to_string(&refreshs).unwrap();
    ff.write_all(&toml.as_bytes()).unwrap();
}

pub fn w_cid(cid: String, file: String, t: bool) -> bool {
    let f = file;
    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .append(true)
        .open(f.clone())
    {
        Err(why) => panic!("Couldn't open {}: {}", f, why),
        Ok(file) => file,
    };
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Couldn't read {}: {}", f, why),
        Ok(_) => (),
    }
    if contents.contains(&cid) == false {
        if t {
            let cid = cid + "\n";
            match file.write_all(cid.as_bytes()) {
                Err(why) => panic!("Couldn't write \"{}\" to {}: {}", contents, f, why),
                Ok(_) => println!("finished"),
            }
        }
        let check = false;
        return check;
    } else {
        let check = true;
        return check;
    }
}

pub fn c_follow_all() {
    let file = "/.config/ai/scpt/follow_all.zsh";
    let mut f = shellexpand::tilde("~").to_string();
    f.push_str(&file);
    use std::process::Command;
    let output = Command::new(&f).output().expect("zsh");
    let d = String::from_utf8_lossy(&output.stdout);
    let d = "\n".to_owned() + &d.to_string();
    println!("{}", d);
}

pub fn c_openai_key(c: &Context) {
    let api = c.args[0].to_string();
    let o = "api='".to_owned() + &api.to_string() + &"'".to_owned();
    let o = o.to_string();
    let l = shellexpand::tilde("~") + "/.config/ai/openai.toml";
    let l = l.to_string();
    let mut l = fs::File::create(l).unwrap();
    if o != "" {
        l.write_all(&o.as_bytes()).unwrap();
    }
    println!("{:#?}", l);
}

impl Open {
    pub fn new() -> Result<Self, ConfigError> {
        let d = shellexpand::tilde("~") + "/.config/ai/openai.toml";
        let s = Config::builder()
            .add_source(File::with_name(&d))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        s.try_deserialize()
    }
}

#[derive(Debug, Deserialize)]
pub struct Open {
    pub api: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenData {
    choices: Vec<Choices>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choices {
    text: String,
}

