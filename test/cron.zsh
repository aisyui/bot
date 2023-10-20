function cron() {
	t=`docker ps |grep aios|grep Up`
	if [ -z "$t" ];then
		docker compose up -d
	fi
	exit
}
