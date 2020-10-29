# Drinks Drunk

An API for tracking drinks drunk.

## Running

1. `cargo +nightly run`

## Watching and running

1. `cargo +nightly watch -x run`

## Running migrations

This project is using [Diesel](http://diesel.rs/) as the ORM.

I didn't want to install Postgres on my system so I could use the Diesel CLI, so it's dockerized.

```
docker-compose up -d diesel-cli
docker-compose exec diesel-cli bash
# cd core
# diesel migration run
```
