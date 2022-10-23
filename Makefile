gen-tree:
	cargo modules generate tree

build:
	cargo clean
	cargo build

test:
	cargo test -- --nocapture

fmt:
	cargo fmt -q