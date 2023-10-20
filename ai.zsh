#!/bin/zsh
case $OSTYPE in
	darwin*)
		alias date="/opt/homebrew/bin/gdate"
		;;
esac
d=${0:a:h}/scpt
source $d/env
source $d/refresh.zsh
source $d/token.zsh
source $d/reply.zsh
source $d/notify.zsh

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
esac
