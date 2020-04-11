FROM node:12.14.0-alpine

COPY package*.json ./

RUN npm ci

COPY ./ ./

ENTRYPOINT [ "npm" ]
CMD [ "test" ]