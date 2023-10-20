cfg=~/.config/ai/test.json

if [ -f $cfg ];then
	host=`cat $cfg|jq -r .host`
	handle=`cat $cfg|jq -r .handle`
	pass=`cat $cfg|jq -r .password`
	date=`date --iso-8601=seconds`
fi

if [ -f $cfg.t ];then
	token=`cat $cfg.t|jq -r .accessJwt`
	refresh=`cat $cfg.t|jq -r .refreshJwt`
	did=`cat $cfg.t|jq -r .did`
fi

if [ ! -d $d/json ];then
	mkdir -p $d/json
fi
