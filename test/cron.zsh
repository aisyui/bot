function cron() {
	t=`docker ps |grep aios|grep R`
	if [ -z "$t" ];then
		exit
	fi
	docker compose up -d
}
