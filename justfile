run:
	cargo run

watch:
	cargo watch -x run

build:
	cargo build --locked

deploy:
	cargo build --locked --release
	PHISHU_DOMAIN=triggerphi.sh ./target/release/phishu

req *args:
	curl -v http://localhost:3000{{args}}

ci: check test fmt clippy
	@echo "✅ All checks passed"

check:
	cargo check

test:
	cargo test

fmt:
	cargo fmt --check

clippy:
	cargo clippy -- --deny warnings
