FROM rust:1.65 AS builder

WORKDIR /app

#to enable cargo build before adding sources
RUN mkdir src
RUN touch src/main.rs
RUN echo "fn main() {}" > src/main.rs

COPY Cargo.toml .
RUN cargo build --release

RUN rm -r src
RUN rm ./target/release/deps/openvpn_client_rs*
COPY src src
RUN cargo build --release



FROM debian:bullseye-slim
RUN apt update -y && apt install -y openvpn curl iputils-ping

WORKDIR /app
COPY --from=builder /app/target/release/openvpn-client-rs ./
ENTRYPOINT ["./openvpn-client-rs"]
