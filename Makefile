test:
	cargo test -v -- --nocapture

build:
	cargo build -v 

release:
	cargo build --release -v

rp:
	cargo run -- run "./programs/$$(ls ./programs | fzf)"
