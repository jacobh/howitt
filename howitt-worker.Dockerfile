# nightly
FROM rustlang/rust@sha256:7b1617a8bdc149cdd8c70f6ca1d512959c0803c176daa9b45761aa1d8776aa23 as nightly

ENV RUSTFLAGS='-C target-cpu=znver2'

RUN RUSTFLAGS='' cargo install cargo-chef

# Add Debian sid to your APT sources list
RUN echo "deb http://deb.debian.org/debian sid main" > /etc/apt/sources.list.d/sid.list

# Create an APT preferences file that gives libheif packages higher priority from sid
RUN echo "Package: libheif*\n\
    Pin: release a=sid\n\
    Pin-Priority: 990\n" > /etc/apt/preferences.d/libheif

# Update package lists and install the modern libheif libraries from sid.
# (Be sure to include any other dependencies that libheif might require, for example, the corresponding shared library)
RUN apt-get update && apt-get install -y libheif-dev 

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
RUN cargo build  --release --bin howitt-worker

# We do not need the Rust toolchain to run the binary!

FROM debian:sid-slim AS runtime

RUN apt update && apt install -y ca-certificates libheif-dev

WORKDIR /app

COPY --from=builder /app/target/release/howitt-worker /usr/local/bin

ENTRYPOINT ["/usr/local/bin/howitt-worker"]
