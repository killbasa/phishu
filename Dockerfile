FROM rust:1.85.1-slim-bookworm AS builder

WORKDIR /temp

RUN apt-get update && \
	apt-get install -y libssl-dev pkg-config && \
	rm -rf /var/lib/apt/lists/*

COPY ./src ./src/
COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build --locked --release

FROM debian:12.10-slim

WORKDIR /etc/phishu

RUN apt-get update -y && \
	apt-get install -y openssl ca-certificates && \
	update-ca-certificates && \
	rm -rf /var/lib/apt/lists/*

ENV HOST=0.0.0.0

COPY --from=builder /temp/target/release/phishu /etc/phishu/app

EXPOSE 3000

ENTRYPOINT [ "/etc/phishu/app" ]
