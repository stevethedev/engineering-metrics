FROM node:18-alpine3.17 AS node

FROM alpine:3.17.2 AS json-builder-setup

ARG VERSION=v0.4.1

WORKDIR /build/json-builder-setup

RUN apk add --no-cache unzip
ADD https://github.com/jsontypedef/json-typedef-codegen/releases/download/$VERSION/x86_64-unknown-linux-musl.zip x86_64-unknown-linux-musl.zip
RUN unzip x86_64-unknown-linux-musl.zip

FROM node AS json-builder

ENV OUTPUT_RS_DIR=/build/json-builder/build/rs
ENV JTD_EXECUTOR_PATH=/usr/bin/jtd-codegen

WORKDIR /build/json-builder

COPY --from=json-builder-setup /build/json-builder-setup/jtd-codegen /usr/bin/jtd-codegen
COPY ./json-schema ./

RUN npm ci --omit=dev
RUN npm run start

FROM rust:1.67.1-alpine3.17 AS rs-builder

ARG BUILD_ARGS

WORKDIR /build/server

RUN apk add --no-cache musl-dev

COPY ./server .
COPY --from=json-builder /build/json-builder/build/rs ./workspace/lib-json-schema/src/generated

RUN \
    --mount=type=cache,target=/build/server/target,rw \
    cargo install --locked --path ./workspace/app


FROM alpine:3.17.2 AS runtime

COPY --from=rs-builder /usr/local/cargo/bin/app /usr/local/bin/app

EXPOSE 80

CMD ["app"]
