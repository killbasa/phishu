FROM rust:1.85.1-slim-bookworm AS builder

WORKDIR /temp

RUN rustup component add rust-std-x86_64-unknown-linux-musl

COPY ./src ./src/
COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build --locked --release --target x86_64-unknown-linux-musl

FROM scratch

ENV HOST=0.0.0.0

COPY ./assets /assets
COPY --from=builder /temp/target/x86_64-unknown-linux-musl/release/phishu /phishu

EXPOSE 3000

ENTRYPOINT [ "/phishu" ]
