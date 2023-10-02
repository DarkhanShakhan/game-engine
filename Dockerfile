# --- Build Stage ---
FROM rust:latest as builder
WORKDIR /game
COPY . .
RUN cargo build --release

FROM rust:1.63-buster

WORKDIR /game

COPY --from=builder ./game/target/release/game-engine .
ENTRYPOINT /bin/bash        



