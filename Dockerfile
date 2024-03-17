FROM rust:latest as builder
ENV APP aderyn
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path ./aderyn
 
FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
WORKDIR /share