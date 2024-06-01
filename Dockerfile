FROM rust:1.78 as builder

WORKDIR /usr/src/tapciify-api
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /usr/src/tapciify-api/target/release/tapciify-api /usr/local/bin/tapciify-api

CMD ["tapciify-api"]
ENV PORT=3000
EXPOSE 3000
