# syntax=docker/dockerfile:1.3-labs

# This Dockerfile is largely written using the learnings from the posts below.
# [faster builds](https://gist.github.com/noelbundick/6922d26667616e2ba5c3aff59f0824cd)
# [rust builds w/ dev](https://qiita.com/kyamamoto9120/items/c391f0ce5050c5ddcc87)

# Development
FROM rust:1.60.0 AS dev

WORKDIR /app
RUN cargo install cargo-watch
COPY . .

# Build
FROM rust:1.60.0-slim-buster AS build

# capture dependencies
COPY Cargo.toml Cargo.lock /app/

# create app using our own Cargo.toml
RUN rm -rf /app
RUN cargo new /app
COPY Cargo.toml /app/

# cache deps
WORKDIR /app
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

# copy sources
COPY . /app

# mount cache
RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
	set -e
	# update timestamps
	touch /app/src/main.rs
	cargo build --release
EOF

CMD ["/app/target/release/restaurant"]

# Release
FROM ubuntu:22.04 AS app
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get -y update && apt-get -y upgrade && \
    apt -y install ca-certificates libssl-dev libpq-dev

COPY --from=build /app/target/release/restaurant /restaurant
CMD ["/restaurant"]
