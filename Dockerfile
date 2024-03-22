FROM rust:1.77 as builder

WORKDIR /usr/src/tapciify-api
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/tapciify-api/target/release/tapciify-api /usr/local/bin/tapciify-api

CMD ["tapciify-api"]

EXPOSE 3000
