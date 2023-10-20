FROM syui/aios
ADD .ssh /root/.ssh

WORKDIR /root
ADD ./test/entrypoint.sh .
RUN chmod +x /root/entrypoint.sh

ENTRYPOINT ["/root/entrypoint.sh"]
