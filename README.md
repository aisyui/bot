## ai `bot`

<img src="./icon/avatar.png" width="100">

- name : ai bot
- base : [rust](https://www.rust-lang.org)
- host : [yui.syui.ai](https://bsky.app/profile/yui.syui.ai), [ai.syu.is](https://web.syu.is/profile/ai.syu.is)

```sh
$ ai
```

### logo

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

|command|type|body|
|---|---|---|
|@yui.syui.ai did|mention, reply| plc.directory/$did/log |

### test

`zsh`

```sh
$ ./ai.zsh t
```

### docker

```sh
$ cp -rf ~/.config/ai ./config/
$ docker build -t syui/aios .
$ docker run -it syui/aios ai bot
---
$ docker stop `docker ps -a -q`
```
