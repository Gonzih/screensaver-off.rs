build:
	cargo build --release
run:
	cargo run
rust-setup:
	rustup default nightly
ci:
	nix-shell shell.nix --run 'make build'
