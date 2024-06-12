FROM rust:1.79.0-bookworm as base
WORKDIR /app
RUN apt update && apt install -y build-essential pkg-config libudev-dev

FROM rust:1.79.0-bookworm as tools
WORKDIR /app
RUN apt update && apt install -y curl
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y cargo-chef cargo-watch

FROM base as planner
COPY --from=tools /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base as cacher_develop
COPY --from=tools /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

FROM base as develop
WORKDIR /app
EXPOSE 8080
RUN rustup component add clippy rustfmt
COPY --from=tools /usr/local/cargo/bin/cargo-watch /usr/local/cargo/bin/cargo-watch
COPY --from=cacher_develop /app/target /app/target
CMD cargo watch -x run

FROM base as cacher_arm_musl
COPY --from=tools /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN apt install -y g++-aarch64-linux-gnu
RUN rustup target add aarch64-unknown-linux-musl
RUN cargo chef cook --recipe-path recipe.json --release --target aarch64-unknown-linux-musl

FROM cacher_arm_musl as builder_arm_musl
WORKDIR /app
COPY . /app
RUN cargo build --release --target aarch64-unknown-linux-musl
WORKDIR /app/target/aarch64-unknown-linux-musl/release
CMD ["sleep", "infinity"]

FROM debian:stable-20240612-slim as runner_arm_musl
WORKDIR /app
COPY ./my-room-sensor /app
CMD ["./my-room-sensor"]
