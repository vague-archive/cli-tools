FROM rust:latest as builder
WORKDIR /usr/src/app

RUN apt-get update -y && \
    apt-get install -y build-essential cmake clang

COPY . .

RUN cargo build --release

FROM ubuntu:latest
COPY --from=builder /usr/src/app/target/release/image_compressor /usr/local/bin/image_compressor
ENTRYPOINT ["/usr/local/bin/image_compressor"]
