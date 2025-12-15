test:
	cargo test -- --nocapture

build:
	cargo build -v 

release:
	cargo build --release -v

rp:
	# used to quickly test a program from ./programs
	cargo run -- run "./programs/$$(ls ./programs | fzf)"
