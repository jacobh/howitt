# nightly
FROM rustlang/rust@sha256:7b1617a8bdc149cdd8c70f6ca1d512959c0803c176daa9b45761aa1d8776aa23 as nightly

ENV RUSTFLAGS='-C target-cpu=znver2'

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo binstall cargo-chef

WORKDIR /app

# planner
FROM nightly AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

# builder
FROM nightly AS builder 

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build  --release --bin howitt-web

# We do not need the Rust toolchain to run the binary!

FROM debian:bookworm-slim AS runtime

RUN apt update && apt install ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/howitt-web /usr/local/bin

ENTRYPOINT ["/usr/local/bin/howitt-web"]
