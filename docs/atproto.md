## curl

no-authorization

https://docs.bsky.app/docs/api/com-atproto-repo-describe-repo

```sh
handle=yui.syui.ai
host=bsky.social
api=$host/xrpc
plc=plc.directory
url=$api/com.atproto.repo.describeRepo

curl -sL ${host}/xrpc/_health

d=`curl -sL "${url}?repo=$handle"`
echo $d
did=`echo $d|jq -r .did`
echo $did

collection=app.bsky.feed.post
url=$api/com.atproto.repo.listRecords
timed=`curl -sL "${url}?repo=$handle&collection=$collection&reverse=true&limit=1"|jq -r ".[]|.[0]?|.value.createdAt"`
cid=`curl -sL "${url}?repo=$handle&collection=$collection&reverse=true&limit=1"|jq -r ".[]|.[0]?|.cid"`
uri=`curl -sL "${url}?repo=$handle&collection=$collection&reverse=true&limit=1"|jq -r ".[]|.[0]?|.uri"`

rkey=`echo $uri|cut -d / -f 5`
url=$api/com.atproto.repo.getRecord
curl -sL "$url?repo=$did&collection=$collection&rkey=$rkey"|jq .

uri=at://did:plc:vjug55kidv6sye7ykr5faxxn/app.bsky.feed.post/3jzn6g7ixgq2y
cid=bafyreiey2tt4dhvuvr7tofatdverqrxmscnnus2uyfcmkacn2fov3vb4wa
did=did:plc:vjug55kidv6sye7ykr5faxxn
rkey=3jzn6g7ixgq2y
url=$api/com.atproto.repo.getRecord
curl -sL "$url?repo=$did&collection=$collection&rkey=$rkey&cid="|jq .
```

