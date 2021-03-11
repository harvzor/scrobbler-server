# Nightly image doesn't support arm.
#FROM rustlang/rust:nightly
FROM rust:1.49

RUN rustup default nightly

WORKDIR /app

COPY . .

RUN cargo build
    # --release

# For running diesel cli.
RUN apt-get update -qq
RUN cargo install diesel_cli --no-default-features --features postgres

CMD cargo run --bin main
