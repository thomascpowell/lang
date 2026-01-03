test:
	cargo test

build:
	cargo build -v 

release:
	cargo build --release -v

testv:
	cargo test -v -- --nocapture

rp:
	cargo run -- run "./programs/$$(ls ./programs | fzf)"
