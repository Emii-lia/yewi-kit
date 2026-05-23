FROM rust:1.94.0 as builder
RUN apt-get update && apt-get install -y nodejs
RUN curl -fsSL https://bun.com/install | bash
ENV BUN_INSTALL=/root/.bun
ENV PATH="${BUN_INSTALL}/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked
WORKDIR /app
COPY . .
RUN bun install
RUN bun run build
RUN trunk build --release
