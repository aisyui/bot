function refresh(){
	token=`cat $cfg.t|jq -r .accessJwt`
	refresh=`cat $cfg.t|jq -r .refreshJwt`
	if [ ! -f $cfg ];then
		token
	fi
	url=https://$host/xrpc/com.atproto.server.refreshSession
	j=`curl -sL -X POST -H "Content-Type: application/json" -H "Authorization: Bearer $refresh" $url`
	echo $j
	echo $j >! $cfg.t
}
