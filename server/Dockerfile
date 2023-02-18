FROM rust:1.67.1-alpine3.17 AS builder

ARG BUILD_ARGS

WORKDIR /app/server

RUN apk add --no-cache musl-dev

COPY . .

RUN cargo install --locked --path ./workspace/app


FROM alpine:3.17.2 AS runtime

COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/app

EXPOSE 80

CMD ["app"]
