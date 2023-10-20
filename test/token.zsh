function token() {
		mkdir -p ~/.config/ai
		echo server:
		read host

		echo password:
		read pass

		echo handle:
		read handle

		echo "{ \"host\":\"$host\", \"password\":\"$pass\", \"handle\":\"$handle\" }" >> $cfg

	url=https://$host/xrpc/com.atproto.server.createSession
	j=`curl -sL -X POST -H "Content-Type: application/json" -d "{\"identifier\":\"$handle\",\"password\":\"$pass\"}" $url`
	echo $j
 echo $j >! $cfg.t
}
