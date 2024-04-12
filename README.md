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
|/did||mention, reply| [plc.directory](https://plc.directory)/$did/log |user|
|/card|r, s, b|mention, reply| [card.syui.ai](https://card.syui.ai) |user|
|/ten|start, close, d, p|mention, reply| [card.syui.ai](https://card.syui.ai)  |user|
|/fav|{cid}|mention, reply| [card.syui.ai](https://card.syui.ai)  |user|
|/egg|{password}|mention, reply| [card.syui.ai](https://card.syui.ai)  |user|
|/nyan|🍬|mention, reply| [yui.syui.ai](https://yui.syui.ai) |user|
|/diffusers|{keyword}|mention, reply| [huggingface.co/diffusers](https://huggingface.co/docs/diffusers/index) |user|
|/sh|{command}|mention, reply| [archlinux.org](https://wiki.archlinux.org/title/Systemd-nspawn) |admin|
|/占い||mention, reply| [yui.syui.ai](https://yui.syui.ai) |user|

```sh
@yui.syui.ai /did
```

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
