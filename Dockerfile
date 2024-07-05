FROM rust:1.78-alpine3.20 as builder
LABEL authors="tapnisu"

WORKDIR /usr/src/tapciify-api

RUN apk add --no-cache alpine-sdk

COPY . .
RUN cargo build --release

FROM alpine:3.20 as runner

COPY --from=builder /usr/src/tapciify-api/target/release/tapciify-api /usr/local/bin/tapciify-api

CMD ["tapciify-api"]
ENV PORT=3000
EXPOSE 3000
