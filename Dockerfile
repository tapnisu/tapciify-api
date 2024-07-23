FROM rust:1.79-alpine3.20 AS builder
LABEL authors="tapnisu"

WORKDIR /usr/src/tapciify-api

RUN apk update \
    && apk upgrade --available \
    && apk add --no-cache alpine-sdk

COPY . .
RUN cargo build --release

FROM alpine:3.20 AS runner

RUN apk update \
    && apk upgrade --available

COPY --from=builder /usr/src/tapciify-api/target/release/tapciify-api /usr/local/bin/tapciify-api

CMD ["tapciify-api"]
ENV PORT=3000
EXPOSE 3000
