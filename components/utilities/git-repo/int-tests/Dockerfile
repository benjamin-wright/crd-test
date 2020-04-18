FROM node:12.14.0-alpine

RUN apk add git openssh

COPY package*.json ./

RUN npm ci

COPY ./ ./

ENTRYPOINT [ "npm" ]
CMD [ "test" ]