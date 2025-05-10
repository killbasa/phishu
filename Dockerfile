FROM rust:1.85.1-slim-bookworm AS builder

WORKDIR /temp

RUN apt-get update && \
	apt-get install -y libssl-dev pkg-config && \
	rm -rf /var/lib/apt/lists/*

COPY ./src ./src/
COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build --locked --release

FROM debian:12.10-slim

RUN apt-get update -y && \
	apt-get install -y openssl && \
	rm -rf /var/lib/apt/lists/*

ENV HOST=0.0.0.0

COPY --from=builder /temp/target/release/phishu /phishu

EXPOSE 3000

ENTRYPOINT [ "/phishu" ]
