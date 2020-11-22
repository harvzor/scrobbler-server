FROM rustlang/rust:nightly

WORKDIR /app

COPY . .

RUN cargo build

CMD cargo run --bin main
