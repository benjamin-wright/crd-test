FROM alpine:3.7

WORKDIR /var/app/src

RUN apk add git openssh && git --version

COPY . .

CMD [ "version" ]