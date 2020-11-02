# Drinks Drunk

An API for tracking drinks drunk.

## Running

```
cargo +nightly run
```

or

```
docker-compose run --rm --service-ports drinks-drunk
```

## Watching and running

```
cargo +nightly watch -x run
```

or

```
docker-compose run --rm --service-ports drinks-drunk-dev
```

## Running migrations

This project is using [Diesel](http://diesel.rs/) as the ORM.

I didn't want to install Postgres on my system so I could use the Diesel CLI, so it's dockerized.

```
docker-compose run --rm diesel-cli
# cd core
# diesel setup
# diesel migration run
```
