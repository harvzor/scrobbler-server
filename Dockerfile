# Nightly image doesn't support arm.
#FROM rustlang/rust:nightly
FROM rust:1.49

# RUN rustup default nightly

# WORKDIR /app

# COPY . .

# RUN cargo build

# CMD cargo run --bin main
