run:
	cargo run

watch:
	DEBUG_LOG=1 cargo watch -x 'run --target x86_64-unknown-linux-musl'

build:
	cargo build --locked --target x86_64-unknown-linux-musl

release:
	cargo build --locked --release --target x86_64-unknown-linux-musl

deploy: release
	DEBUG_LOG=1 PHISHU_DOMAIN=triggerphi.sh ./target/x86_64-unknown-linux-musl/release/phishu

ci:
	cargo check
	cargo test
	cargo fmt --check
	cargo clippy -- --deny warnings
	cargo shear
	@echo "✅ All checks passed"
