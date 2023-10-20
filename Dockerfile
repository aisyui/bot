FROM syui/aios

USER root
ADD .config /root/.config
WORKDIR /root

ADD ./scpt/entrypoint.sh /
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
