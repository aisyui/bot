## test-notify

```sh
./target/debug/ai n|jq -r ".notifications|.[].cid" >> ~/.config/ai/txt/notify_cid*
./target/debug/ai bot
```

## docker

```sh
$ docker run -it syui/aios ai
$ docker run -it -d syui/aios zsh -c "ai login <handle> -p <password> && ai bot"
```

```sh
$ cp -rf ~/.config/ai ./.config/

$ docker compose up
```

## cron

```sh
$ sudo pacman -S fcron
$ fcrontab -e
* * * * * $HOME/bot/test/ai.zsh c
```

## ssh

```sh
$ ssh-keygen -f /.ssh/diffusers.key -t ed25519
```

```sh
FROM syui/aios
ADD .ssh /root/.ssh
```

```sh
Host diffusers
	HostName localhost
	User root
	IdentityFile ~/.ssh/diffusers.key
	StrictHostKeyChecking no
	UserKnownHostsFile /dev/null
```

```sh
services:
  aios:
    #image: syui/aios
    build:
      context: .
    restart: always
    volumes:
      - ./.config:/root/.config
    command: ai bot -a syui.syu.is
```

## openapi

```sh
# https://github.com/rdmurphy/atproto-openapi-types
$ curl -sLO https://raw.githubusercontent.com/rdmurphy/atproto-openapi-types/main/spec/api.json
```

## plc

```sh
# 何度か実行するとplcをlatestまでexportされる
$ .config/ai/scpt/test/pds.zsh e
``` 

## cmt

blogなどにblueskyアカウントのpostを表示します。

以下でbotがblogのコメントシステムを開きます。

```sh
@yui.syui.ai /comment https://syui.ai/blog/post/2024/04/25/bluesky/
```

開いたbotのpostに返信することで、特定のblog path上でpostを表示します。

<blockquote class="bluesky-embed" data-bluesky-uri="at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.post/3kqxbtmwlje2h" data-bluesky-cid="bafyreiasxp5g3nkkd6g7lxh55qaxcc7ylefaljmbcp627nu2geks62c57m"><p lang="">please reply with your comments here ↓ 
</p>&mdash; ai (<a href="https://bsky.app/profile/did:plc:4hqjfn7m6n5hno3doamuhgef?ref_src=embed">@yui.syui.ai</a>) <a href="https://bsky.app/profile/did:plc:4hqjfn7m6n5hno3doamuhgef/post/3kqxbtmwlje2h?ref_src=embed">Apr 25, 2024 at 20:18</a></blockquote><script async src="https://embed.bsky.app/static/embed.js" charset="utf-8"></script>

```ts
<link href="https://syui.ai/js/comment/app.js" rel="preload" as="script">
<link href="https://syui.ai/js/comment/chunk-vendors.js" rel="preload" as="script">
<div id="comment"></div>
<script async src="https://embed.bsky.app/static/embed.js" charset="utf-8"></script>
<script src="https://syui.ai/js/comment/chunk-vendors.js"></script>
<script src="https://syui.ai/js/comment/app.js"></script>
```

## example json

```json
[
{
      "uri": "at://did:plc:wkzuqomvkxx5eiv5nl2lvm23/app.bsky.feed.post/3kp4ze5dcek2j",
      "cid": "bafyreic4g7mthhw654zgv4skt5tqbs2xqg6n7bli4gayl2nquljngnotiy",
      "author": {
        "did": "did:plc:wkzuqomvkxx5eiv5nl2lvm23",
        "handle": "syui.syu.is",
        "displayName": "syui",
        "avatar": "https://api.syu.is/img/avatar/plain/did:plc:wkzuqomvkxx5eiv5nl2lvm23/bafkreifvabvstfgawt6csagh44xdevb6c2uiwpgfho3xnpdrr6o7nbkxry@jpeg",
        "indexedAt": "2024-01-14T10:20:13.367Z",
        "viewer": {
          "muted": false,
          "blockedBy": false,
          "following": "at://did:plc:dconvttcori3mrh2wrmehvwt/app.bsky.graph.follow/3kiztjatnms25",
          "followedBy": "at://did:plc:wkzuqomvkxx5eiv5nl2lvm23/app.bsky.graph.follow/3kirwsboeos26"
        },
        "labels": []
      },
      "reason": "reply",
      "reasonSubject": "at://did:plc:dconvttcori3mrh2wrmehvwt/app.bsky.feed.post/3kp4zdnlo5s2j",
      "record": {
        "text": "1",
        "$type": "app.bsky.feed.post",
        "langs": [
          "ja"
        ],
        "reply": {
          "root": {
            "cid": "bafyreiceckunxajycacn7dbuojrwb2wmurhfkleermvewwik44cn6vqo3a",
            "uri": "at://did:plc:dconvttcori3mrh2wrmehvwt/app.bsky.feed.post/3kp4zdnlo5s2j"
          },
          "parent": {
            "cid": "bafyreiceckunxajycacn7dbuojrwb2wmurhfkleermvewwik44cn6vqo3a",
            "uri": "at://did:plc:dconvttcori3mrh2wrmehvwt/app.bsky.feed.post/3kp4zdnlo5s2j"
          }
        },
        "createdAt": "2024-04-02T07:12:28.799Z"
      },
      "isRead": true,
      "indexedAt": "2024-04-02T07:12:28.799Z",
      "labels": []
},
{
      "uri": "at://did:plc:wkzuqomvkxx5eiv5nl2lvm23/app.bsky.feed.post/3kp54af2zes2j",
      "cid": "bafyreig4kvfpu557qehttt2y5eh7rcyodbxqwtnl73f3fhjsstiap3abzu",
      "author": {
        "did": "did:plc:wkzuqomvkxx5eiv5nl2lvm23",
        "handle": "syui.syu.is",
        "displayName": "syui",
        "avatar": "https://api.syu.is/img/avatar/plain/did:plc:wkzuqomvkxx5eiv5nl2lvm23/bafkreifvabvstfgawt6csagh44xdevb6c2uiwpgfho3xnpdrr6o7nbkxry@jpeg",
        "indexedAt": "2024-01-14T10:20:13.367Z",
        "viewer": {
          "muted": false,
          "blockedBy": false,
          "following": "at://did:plc:dconvttcori3mrh2wrmehvwt/app.bsky.graph.follow/3kiztjatnms25",
          "followedBy": "at://did:plc:wkzuqomvkxx5eiv5nl2lvm23/app.bsky.graph.follow/3kirwsboeos26"
        },
        "labels": []
      },
      "reason": "reply",
      "reasonSubject": "at://did:plc:dconvttcori3mrh2wrmehvwt/app.bsky.feed.post/3kp4zdnlo5s2j",
      "record": {
        "text": "2",
        "$type": "app.bsky.feed.post",
        "langs": [
          "ja"
        ],
        "reply": {
          "root": {
            "cid": "bafyreiceckunxajycacn7dbuojrwb2wmurhfkleermvewwik44cn6vqo3a",
            "uri": "at://did:plc:dconvttcori3mrh2wrmehvwt/app.bsky.feed.post/3kp4zdnlo5s2j"
          },
          "parent": {
            "cid": "bafyreiceckunxajycacn7dbuojrwb2wmurhfkleermvewwik44cn6vqo3a",
            "uri": "at://did:plc:dconvttcori3mrh2wrmehvwt/app.bsky.feed.post/3kp4zdnlo5s2j"
          }
        },
        "createdAt": "2024-04-02T08:04:03.938Z"
      },
      "isRead": true,
      "indexedAt": "2024-04-02T08:04:03.938Z",
      "labels": []
}
]
```

```json
{
      "uri": "at://did:plc:uqzpqmrjnptsxezjx4xuh2mn/app.bsky.feed.post/3kp5qniyzm42h",
      "cid": "bafyreihmutmtf2clpgmx5l3qpu6xea6z25xrop74mltsycs5lfacm27u6e",
      "author": {
        "did": "did:plc:uqzpqmrjnptsxezjx4xuh2mn",
        "handle": "syui.ai",
        "displayName": "syui",
        "avatar": "https://cdn.bsky.app/img/avatar/plain/did:plc:uqzpqmrjnptsxezjx4xuh2mn/bafkreid6kcc5pnn4b3ar7mj6vi3eiawhxgkcrw3edgbqeacyrlnlcoetea@jpeg",
        "viewer": {
          "muted": false,
          "blockedBy": false,
          "followedBy": "at://did:plc:uqzpqmrjnptsxezjx4xuh2mn/app.bsky.graph.follow/3kkvst5iq6r2a"
        },
        "labels": [],
        "description": "https://syui.ai",
        "indexedAt": "2024-01-25T23:54:12.979Z"
      },
      "reason": "reply",
      "reasonSubject": "at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.post/3kp5qn72s232q",
      "record": {
        "$type": "app.bsky.feed.post",
        "createdAt": "2024-04-02T14:09:18.926Z",
        "langs": [
          "ja"
        ],
        "reply": {
          "parent": {
            "cid": "bafyreiewdfyh6rywpkdzpmf5markqa6tavc5smc32q7cw2wpwbqik5hnfm",
            "uri": "at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.post/3kp5qn72s232q"
          },
          "root": {
            "cid": "bafyreiewdfyh6rywpkdzpmf5markqa6tavc5smc32q7cw2wpwbqik5hnfm",
            "uri": "at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.post/3kp5qn72s232q"
          }
        },
        "text": "first"
      },
      "isRead": true,
      "indexedAt": "2024-04-02T14:09:18.926Z",
      "labels": []
}
```
