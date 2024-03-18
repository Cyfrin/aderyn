FROM rust:latest as builder
ENV APP aderyn
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path ./aderyn
RUN apt-get update && apt-get install curl
RUN curl -L https://foundry.paradigm.xyz | bash
RUN ~/.foundry/bin/foundryup
 
FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
COPY --from=builder /root/.foundry/bin/forge /usr/local/bin/forge
WORKDIR /share
ENTRYPOINT ["aderyn"]