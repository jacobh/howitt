FROM rustlang/rust:nightly as build

COPY . .

RUN cargo build --bin howitt-web

CMD cargo run --bin howitt-web
