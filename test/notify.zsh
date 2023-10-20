function notify() {
	url=https://$host/xrpc/app.bsky.notification.listNotifications
	f=$d/json/notify.json
	if [ ! -f $f ];then
		curl -sL "Content-Type: application/json" -H "Authorization: Bearer $token" "$url?limit=100" >! $f
	fi

	for ((i=0;i<=99;i++))
	do
		echo "[$i]---"
		cid=`cat $f|jq -r ".|.[].[$i]?|.cid?"`
		uri=`cat $f|jq -r ".|.[].[$i]?|.uri?"`
		echo cid: $cid
		echo uri: $uri
		cid_r=`cat $f|jq -r ".[]|.[$i]?|.record.reply.root.cid?"`

		if [ "$cid_r" = "null" ];then
			continue
		fi
		uri_r=`cat $f|jq -r ".[]|.[$i]?|.record.reply.root.uri?"`
		cid_p=`cat $f|jq -r ".[]|.[$i]?|.record.reply.parent.cid?"`
		uri_p=`cat $f|jq -r ".[]|.[$i]?|.record.reply.parent.uri?"`
		did_p=`echo $uri_p|cut -d / -f 3`
		if [ "$did_p" != "did:plc:uqzpqmrjnptsxezjx4xuh2mn" ];then
			continue
		fi
		echo cid_root: $cid_r
		echo uri_root: $uri_r
		echo cid_parent: $cid_p
		echo uri_parent: $uri_p
		echo ---
		echo uri: $uri|sed "s#at://#https://bsky.app/profile/#g"|sed 's/app.bsky.feed.post/post/g'
		echo uri_root: $uri_r|sed "s#at://#https://bsky.app/profile/#g"|sed 's/app.bsky.feed.post/post/g'
		echo uri_parent: $uri_p|sed "s#at://#https://bsky.app/profile/#g"|sed 's/app.bsky.feed.post/post/g'
		echo ---
	done
}
