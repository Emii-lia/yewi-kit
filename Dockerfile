FROM rust:1.94.0 as builder
RUN apt-get update && apt-get install -y nodejs npm
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked
WORKDIR /app
COPY . .
RUN npm install
RUN npm run build
RUN trunk build --release
