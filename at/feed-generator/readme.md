# custom feed

- at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.generator/cmd
- [bsky.app](https://bsky.app/profile/did:plc:4hqjfn7m6n5hno3doamuhgef/feed/cmd)
- [app.bsky.feed.getFeedSkeleton](https://feed.syu.is/xrpc/app.bsky.feed.getFeedSkeleton?feed=at://did:plc:4hqjfn7m6n5hno3doamuhgef/app.bsky.feed.generator/cmd)

```sh
did=did:plc:4hqjfn7m6n5hno3doamuhgef
col=app.bsky.feed.generator
cid=cmd
uri=at://$did/$col/$cid

echo $uri
```

## bsky-feed

```sh
$ git clone https://github.com/bluesky-social/feed-generator
```

```sh
docker compose build feed-generator
docker build -t publish_feed -f Dockerfile.feed .
docker run publish_feed
```
