# Drinks Drunk

An API for tracking drinks drunk.

## Running

```
cargo +nightly run
```

or

```
docker-compose up drinks-drunk
```

## Watching and running

```
cargo +nightly watch -x run
```

or

```
docker-compose up drinks-drunk-dev
# cargo +nightly watch --poll -x run
```

`-poll` option is needed because of this issue: https://stackoverflow.com/questions/52996052/node-watch-on-a-docker-compose-volume-does-not-register-delete-events

## Running migrations

This project is using [Diesel](http://diesel.rs/) as the ORM.

I didn't want to install Postgres on my system so I could use the Diesel CLI, so it's dockerized.

```
docker-compose up -d diesel-cli
docker-compose exec diesel-cli bash
# cd core
# diesel setup
# diesel migration run
```
