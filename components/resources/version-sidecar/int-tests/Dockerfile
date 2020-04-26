FROM node:12.14.0-alpine

RUN npm set @minion-ci:registry https://npm.ponglehub.co.uk && npm config set strict-ssl false

COPY package*.json ./

RUN npm ci

COPY ./ ./