
function cid(){
	dd=${d:h}
	ai=$dd/target/debug/ai
	txt=$dd/.config/ai/txt
	f=$txt/notify_cid
	if [ ! -f $ai ];then
		cd $dd
		cargo build
	fi
	if [ ! -d $txt ];then
		mkdir -p $txt
	fi
	$ai n|jq -r ".[]|.[]?.cid" >> $f.txt
	cp -rf $f.txt ${f}_run.txt
}
