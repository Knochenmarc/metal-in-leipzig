FROM rust:slim-bullseye as build

RUN apt-get -y update && apt-get -y upgrade && \
    apt-get install -y pkg-config libssl-dev

RUN USER=root cargo new --bin app
WORKDIR /app
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/metal_in_leipzig*

COPY ./src ./src
RUN cargo build --release

FROM dpokidov/imagemagick:latest-bullseye

WORKDIR /app
COPY --from=build /app/target/release/metal-in-leipzig /app

ENTRYPOINT []
CMD ["./metal-in-leipzig"]