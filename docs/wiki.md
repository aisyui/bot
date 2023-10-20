### docker

```sh
$ docker run -it syui/aios ai
$ docker run -it -d syui/aios zsh -c "ai login <handle> -p <password> && ai bot"
```

```sh
$ cp -rf ~/.config/ai ./.config/

$ docker compose up
```

### cron

```sh
$ sudo pacman -S fcron
$ fcrontab -e
* * * * * $HOME/bot/test/ai.zsh c
```

### ssh

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
