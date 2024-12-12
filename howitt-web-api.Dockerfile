FROM rustlang/rust:nightly as build

WORKDIR /app

COPY . .

RUN cargo build  --release --bin howitt-web

CMD cargo run --release --bin howitt-web
