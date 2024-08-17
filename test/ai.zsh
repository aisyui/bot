#!/bin/zsh

# https://www.docs.bsky.app/docs/get-started

case $OSTYPE in
	darwin*)
		alias date="/opt/homebrew/bin/gdate"
		;;
esac
d=${0:a:h}

source $d/env.zsh
source $d/refresh.zsh
source $d/token.zsh
source $d/reply.zsh
source $d/notify.zsh
source $d/notify_cid.zsh
source $d/cron.zsh
source $d/feed.zsh

case $1 in
	refresh|r)
		refresh
		;;
	token|t)
		token
		;;
	reply|rep)
		reply
		;;
	notify|n)
		notify
		;;
	cron|c)
		cron
		;;
	cid)
		cid
		;;
	feed)
		feed
		;;
esac
