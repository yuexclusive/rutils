.PHONY: build
build:
	cargo build --target=aarch64-unknown-linux-musl --release --target-dir=./target
	docker build --rm -t rocket:latest .


.PHONY: run
run:
	# docker stop rocket_demo
	# docker rm rocket_demo
	docker run -itd -p 8080:8080 --name=rocket_demo rocket