FROM node:12.14.0-alpine

RUN apk add expect

WORKDIR /var/app/src

COPY ./ /usr/local/bin

ENTRYPOINT [ "/usr/local/bin/entrypoint.sh" ]