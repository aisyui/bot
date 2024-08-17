function feed(){
	token=`cat ~/.config/ai/token.json|jq -r .accessJwt`
	url=at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.generator/cmd
	curl -sL "https://public.api.bsky.app/xrpc/app.bsky.feed.getFeed?feed=$url" -H "Authorization: Bearer $token"
}
