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

