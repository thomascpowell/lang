test:
	cargo test

build:
	cargo build -v 

release:
	cargo build --release -v

testv:
	cargo test -v -- --nocapture

rp:
	# used to quickly test a program from ./programs
	cargo run -- run "./programs/$$(ls ./programs | fzf)"
