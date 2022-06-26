FROM rust:alpine

WORKDIR /app
COPY ./src ./src
COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN apk add --no-cache pkgconfig openssl openssl-dev libc-dev imagemagick

RUN cargo install --path .
CMD ["metal-in-leipzig"]