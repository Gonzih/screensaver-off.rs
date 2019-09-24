build:
	cargo build --release
run:
	cargo run
ci:
	nix-shell shell.nix --run 'make build'
