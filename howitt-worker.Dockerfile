# nightly
FROM rustlang/rust@sha256:7b1617a8bdc149cdd8c70f6ca1d512959c0803c176daa9b45761aa1d8776aa23 as nightly

ENV RUSTFLAGS='-C target-cpu=znver2'

RUN RUSTFLAGS='' cargo install cargo-chef

# Add sid repository and set up APT pinning
RUN echo "deb http://deb.debian.org/debian sid main" > /etc/apt/sources.list.d/unstable.list && \
    echo 'Package: *\nPin: release a=sid\nPin-Priority: 50\n\nPackage: libheif-dev\nPin: release a=sid\nPin-Priority: 990' > /etc/apt/preferences.d/unstable && \
    apt update && \
    apt install -y libheif-dev pkg-config

# Set PKG_CONFIG_PATH so that pkg-config can find libheif.pc
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

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

FROM debian:bookworm-slim AS runtime

# Add sid repository and set up APT pinning in the runtime image
RUN echo "deb http://deb.debian.org/debian sid main" > /etc/apt/sources.list.d/unstable.list && \
    echo 'Package: *\nPin: release a=sid\nPin-Priority: 50\n\nPackage: libheif-dev\nPin: release a=sid\nPin-Priority: 990' > /etc/apt/preferences.d/unstable && \
    apt update && \
    apt install -y ca-certificates libheif-dev pkg-config

WORKDIR /app

COPY --from=builder /app/target/release/howitt-worker /usr/local/bin

ENTRYPOINT ["/usr/local/bin/howitt-worker"]
