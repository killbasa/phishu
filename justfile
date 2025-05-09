run:
	cargo run

watch:
	cargo watch -x run

build:
	cargo build --locked

release:
	cargo build --locked --release --target x86_64-unknown-linux-musl

deploy: release
	PHISHU_DOMAIN=triggerphi.sh ./target/x86_64-unknown-linux-musl/release/phishu

ci:
	cargo check
	cargo test
	cargo fmt --check
	cargo clippy -- --deny warnings
	cargo shear
	@echo "✅ All checks passed"
