FROM rust:1.85 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/barrage /usr/local/bin/barrage
EXPOSE 3000
ENTRYPOINT ["barrage"]
CMD ["server"]
