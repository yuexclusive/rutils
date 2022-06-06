.PHONY: rust-builder
rust-builder:
	docker build --rm -f rust-builder.Dockerfile -t rust-builder:latest .

.PHONY: build
build:
	docker run --rm -it -w /app -v /Users/yu/rust/rutils/rocket_demo:/app rust-builder:latest cargo build --target=aarch64-unknown-linux-musl --release

.PHONY: image
image: build
	docker rmi rocket_demo:latest
	docker build -t rocket_demo:latest .


.PHONY: run
run:
	# docker stop rocket_demo
	# docker stop rocket_demo
	docker run --rm -itd -p 8080:8080 --name=rocket_demo rocket_demo