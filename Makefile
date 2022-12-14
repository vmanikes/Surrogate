gen-tree:
	cargo modules generate tree

build:
	cargo clean
	cargo build --release

update:
	cargo update -p surrogate

test:
	cargo test --all-features -- --nocapture

fmt:
	cargo clippy
	cargo fmt -q