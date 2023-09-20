# Build: docker build -t server .
FROM rust:alpine as builder

# Set the target feature to -crt-static to build for alpine
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Install musl-dev to build for alpine
RUN apk add --no-cache musl-dev

# Install openssl-dev to build for alpine
RUN apk add --no-cache openssl-dev

# Install pkgconfig to build for alpine
RUN apk add --no-cache pkgconfig

WORKDIR /

# Copy the Cargo files for dependency resolution
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY ./engine/Cargo.toml ./engine/Cargo.toml
COPY ./server/Cargo.toml ./server/Cargo.toml
COPY ./ai/Cargo.toml ./ai/Cargo.toml
COPY ./cli/Cargo.toml ./cli/Cargo.toml

#Create dummy files to trick cargo into building the dependencies
RUN mkdir ./ai/src && echo "" > ./ai/src/lib.rs
RUN mkdir ./cli/src && echo "" > ./cli/src/lib.rs
RUN mkdir ./engine/src && echo "" > ./engine/src/lib.rs
RUN mkdir ./server/src && echo "fn main() {}" > ./server/src/main.rs

# Build the dependencies
RUN cargo build --release --package server --bin server

# Remove dummy files of engine and copy the actual files
RUN rm -rf ./engine/src
COPY ./engine/src ./engine/src

# Remove dummy engine files compilation artifacts
RUN rm ./target/release/deps/engine*
RUN rm ./target/release/deps/libengine*

# Build the engine
RUN cargo build --release --package server --bin server

# Remove dummy files of server and copy the actual files
RUN rm -rf ./server/src
COPY ./server/src ./server/src

# Remove dummy server files compilation artifacts
RUN rm ./target/release/deps/server*

# Build the serve
RUN cargo build --release --package server --bin server

# Copy the binary to a new container
FROM alpine:latest as runtime

# Install musl to run the binary
RUN apk add --no-cache musl

# Install libgcc
RUN apk add --no-cache libgcc

# Copy the binary from the builder container
COPY --from=builder /target/release/server /usr/local/bin/server

# Copy .env file
COPY .env .env

EXPOSE 8080

# Run the binary
CMD ["server"]