FROM rustlang/rust:nightly as build

WORKDIR /app

COPY .. .

RUN cargo build --bin howitt-web

CMD cargo run --bin howitt-web
