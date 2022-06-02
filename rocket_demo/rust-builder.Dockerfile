FROM messense/rust-musl-cross:aarch64-musl
ENV DATABASE_URL mysql://test:123@192.168.0.243:3306/evolve
RUN rustup default nightly 
RUN rustup target add aarch64-unknown-linux-musl
RUN apt update
# RUN apt install -y musl-tools musl-dev
RUN apt install -y pkg-config libssl-dev
# RUN apt install build-essential
RUN update-ca-certificates