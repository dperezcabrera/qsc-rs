
FROM rust:1.89 as builder
WORKDIR /usr/src/qsc
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl jq && rm -rf /var/lib/apt/lists/*
WORKDIR /
COPY --from=builder /usr/src/qsc/target/release/qsc-rs-simple-contracts /usr/local/bin/qsc-rs-simple-contracts
COPY --from=builder /usr/src/qsc/target/release/qsc-tools /usr/local/bin/qsc-tools
EXPOSE 8000
ENV QSC_DATA_DIR=/data
VOLUME ["/data"]
CMD ["/usr/local/bin/qsc-rs-simple-contracts"]
