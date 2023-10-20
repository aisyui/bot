function reply() {

	#uri: https://bsky.app/profile/did:plc:4hqjfn7m6n5hno3doamuhgef/post/3kkumyv72w22o
	#uri_root: https://bsky.app/profile/did:plc:uqzpqmrjnptsxezjx4xuh2mn/post/3kkumysfipk2p
	#uri_parent: https://bsky.app/profile/did:plc:uqzpqmrjnptsxezjx4xuh2mn/post/3kkumysfipk2p

	cid=bafyreiaxz6hbqgylsxglqita73c5gzxzoatupgitd35rwjpd6dzpa4ctwi
	uri=at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.post/3kkumyv72w22o
	cid_root=bafyreiacxuk4ypaxbg7qxnmrvpnaej5o7azewqioelfgbuikp77jevy6hq
	uri_root=at://did:plc:uqzpqmrjnptsxezjx4xuh2mn/app.bsky.feed.post/3kkumysfipk2p
	cid_parent=bafyreiacxuk4ypaxbg7qxnmrvpnaej5o7azewqioelfgbuikp77jevy6hq
	uri_parent=at://did:plc:uqzpqmrjnptsxezjx4xuh2mn/app.bsky.feed.post/3kkumysfipk2p

	url="https://$host/xrpc/com.atproto.repo.createRecord"
	col="app.bsky.feed.post"

json="{
    \"repo\": \"$handle\",
    \"did\": \"$did\",
    \"collection\": \"$col\",
    \"record\": {
        \"text\": \"$text\",
        \"createdAt\": \"$date\",
        \"reply\": {
            \"root\": {
                \"cid\": \"$cid_root\",
                \"uri\": \"$uri_root\"
            },
            \"parent\": {
                \"cid\": \"$cid\",
                \"uri\": \"$uri\"
            }
        }
    }
}"

	echo $json|jq .
	url=https://$host/xrpc/com.atproto.repo.createRecord
	curl -sL -X POST -H "Content-Type: application/json" -H "Authorization: Bearer $token" -d "$json" $url
}
