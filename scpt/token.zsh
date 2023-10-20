function token() {
	url=https://$host/xrpc/com.atproto.server.createSession
	j=`curl -sL -X POST -H "Content-Type: application/json" -d "{\"identifier\":\"$handle\",\"password\":\"$pass\"}" $url`
	echo $j
 echo $j >! $cfg.t
}
