.PHONY: test 
test:
	cargo test --all --no-fail-fast

.PHONY: check
check:
	cargo clippy --all -- -D warnings
