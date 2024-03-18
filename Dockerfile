FROM rust:latest as builder
ENV APP aderyn
ENV DEP forge
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path ./aderyn
WORKDIR /usr/src/$DEP
RUN cargo install --git https://github.com/foundry-rs/foundry --profile local --locked $DEP
 
FROM debian:bookworm-slim
RUN apt-get update && apt-get install curl -y  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
COPY --from=builder /usr/local/cargo/bin/$DEP /usr/local/bin/$DEP
WORKDIR /share
