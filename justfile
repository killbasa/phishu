run:
	cargo run

watch:
	DEBUG_LOG=1 cargo watch --ignore db.sqlite -x run

build:
	cargo build --locked

release:
	cargo build --locked --release

deploy: release
	DEBUG_LOG=1 PHISHU_DOMAIN=triggerphi.sh ./target/release/phishu

ci:
	cargo check
	cargo test
	cargo fmt --check
	cargo clippy -- --deny warnings
	cargo shear
	@echo "✅ All checks passed"
