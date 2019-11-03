# FROM rust:latest
FROM rustlang/rust:nightly

COPY . .

RUN cargo install --path .

CMD ["drinks-drunk"]
