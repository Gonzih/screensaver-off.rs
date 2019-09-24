build:
	cargo build --release
run:
	cargo run
docker-image:
	docker build -t rust-build $(shell pwd)
ci: docker-image
	docker run -v $(shell pwd):/code -t rust-build make build
