FROM node:18-alpine3.17 AS node

FROM alpine:3.17.2 AS json-builder-setup

ARG VERSION=v0.4.1

WORKDIR /build/json-builder-setup

RUN apk add --no-cache unzip
ADD https://github.com/jsontypedef/json-typedef-codegen/releases/download/$VERSION/x86_64-unknown-linux-musl.zip x86_64-unknown-linux-musl.zip
RUN unzip x86_64-unknown-linux-musl.zip

FROM node AS json-builder

ENV OUTPUT_TS_DIR=/app/json-builder/build/ts

WORKDIR /build/json-builder

COPY --from=json-builder-setup /build/json-builder-setup/jtd-codegen /usr/bin/jtd-codegen
COPY ./json-schema ./

RUN node ./run.js

WORKDIR /build/json-builder/build/ts

FROM node AS ts-builder

ARG NODE_ENV=development
ENV NODE_ENV=$NODE_ENV

ARG API_BASE_URL
ENV API_BASE_URL=$API_BASE_URL

ARG API_AUTH_URL
ENV API_AUTH_URL=$API_AUTH_URL

ARG API_AUTH_LOGIN_URL
ENV API_AUTH_LOGIN_URL=$API_AUTH_LOGIN_URL

WORKDIR /build/client

COPY ./client ./
COPY --from=json-builder /build/json-builder/build/ts ./src/generated

RUN npm ci
RUN if [ "$NODE_ENV" = "production" ]; then npm run build; else npm run build:dev; fi

FROM nginx:1.23.3-alpine AS runtime

COPY --from=ts-builder /build/client/dist /usr/share/nginx/html
COPY --from=ts-builder /build/client/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
