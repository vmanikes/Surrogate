gen-tree:
	cargo modules generate tree

build:
	cargo clean
	cargo build

test:
	cargo test --all-features -- --nocapture

fmt:
	cargo clippy
	cargo fmt -q