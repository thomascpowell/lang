test:
	cargo test -v -- --nocapture

build:
	cargo build -v 

release:
	cargo build --release -v

run:
	cargo run -- "$$(printf "%s\n" lexer parser run | fzf)" "./programs/$$(ls ./programs | fzf)"
