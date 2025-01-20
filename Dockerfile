FROM syui/aios

WORKDIR /root
ADD ./test/entrypoint.sh .
RUN chmod +x /root/entrypoint.sh

ENTRYPOINT ["/root/entrypoint.sh"]
