## ai `bot`

<img src="./icon/avatar.png" width="100">

- name : ai bot
- base : [ai os](https://git.syui.ai/ai/os)
- host : [yui.syui.ai](https://bsky.app/profile/yui.syui.ai), [ai.syu.is](https://web.syu.is/profile/ai.syu.is)

```sh
$ ai
```

```sh
$ docker run -it syui/aios ai
```

### build

```sh
$ cargo build
$ ./target/debug/ai ai
```

```sh
$ ai ai -t avatar
```

### login

```sh
# ai login $handle -p $password
$ ai l yui.syui.ai -p password

$ cat ~/.config/ai/token.toml
```

```sh
# ai l $handle -p $password -s $server
$ ai l ai.syu.is -p password -s syu.is
```

### refresh

```
$ ai r
```

### notify

```
$ ai n
```

```
$ ai n | jq .
```

### bot

```
$ ai bot
```

|command|sub|type|link|auth|
|---|---|---|---|---|
|@yui.syui.ai did||mention, reply| [plc.directory](https://plc.directory)/$did/log |user|
|@yui.syui.ai card|r, s, b|mention, reply| [card.syui.ai](https://card.syui.ai) |user|
|@yui.syui.ai ten|start, d, p, coin|mention, reply| [card.syui.ai](https://card.syui.ai)  |user|
|@yui.syui.ai fav|{cid}|mention, reply| [card.syui.ai](https://card.syui.ai)  |user|
|@yui.syui.ai egg|{password}|mention, reply| [card.syui.ai](https://card.syui.ai)  |user|
|@yui.syui.ai 占い||mention, reply| [yui.syui.ai](https://yui.syui.ai) |user|
|@yui.syui.ai nyan|🍬|mention, reply| [yui.syui.ai](https://yui.syui.ai) |user|
|@yui.syui.ai diffusers|{keyword}|mention, reply| [huggingface.co/diffusers](https://huggingface.co/docs/diffusers/index) |user|
|@yui.syui.ai sh|{command}|mention, reply| [archlinux.org](https://wiki.archlinux.org/title/Systemd-nspawn) |admin|

### test

`zsh`

```sh
$ ./test/ai.zsh t
```

### make

```sh
$ cargo install --force cargo-make
$ cargo make build
```


### docker

> .env 

```sh
HANDLE=ai.syu.is
PASSWORD=xxx
HOST=syu.is
ADMIN=syui.syu.is
```

```sh
$ docker compose build
$ docker compose up -d
```
