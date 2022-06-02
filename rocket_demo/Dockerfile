# FROM rust-builder:latest as builder
# workdir /app

# copy ./ .
# ENV DATABASE_URL mysql://test:123@192.168.0.243:3306/evolve
# RUN cargo build --target=aarch64-unknown-linux-musl --release

FROM alpine
run apk update && apk add tzdata
run ln /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
run echo "Asia/Shanghai" >/etc/timezone
workdir /app
copy ./target/aarch64-unknown-linux-musl/release/rocket_demo .
copy ./config.toml .
copy ./Rocket.toml .
copy ./log4rs.yml .
CMD ["/app/rocket_demo"]
