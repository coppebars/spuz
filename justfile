fmt:
	cargo +nightly fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo +nightly fmt --all -- --check
