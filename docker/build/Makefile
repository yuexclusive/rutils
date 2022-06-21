.PHONY: builder
builder:
	docker build -f Dockerfile -t rust-builder:latest .

.PHONY: build
build:
	docker run --rm -it -w /app -v `pwd`:/app rust-builder:latest cargo build --target=aarch64-unknown-linux-musl --release