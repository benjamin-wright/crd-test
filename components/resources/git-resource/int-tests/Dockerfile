FROM node:12.14.0-alpine

RUN apk add git openssh \
    && npm set @minion-ci:registry https://npm.ponglehub.co.uk \
    && npm config set strict-ssl false

COPY package.json package.json

RUN npm install

COPY ./ ./