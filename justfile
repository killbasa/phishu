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

ci:
	cargo check
	cargo test
	cargo fmt --check
	cargo clippy -- --deny warnings
	cargo shear
	@echo "✅ All checks passed"
