FROM alpine:3.7

WORKDIR /var/app/src

RUN apk add git && git --version

COPY ./bin .

CMD [ "/bin/bash", "-c", "echo 'running'; while true; do sleep 3000; done" ]