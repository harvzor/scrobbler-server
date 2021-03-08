# Scrobbler server

An API for things.

## Running

```
docker-compose run --rm --service-ports scrobbler
```

You can also add something like `cargo run --bin api` if you just want to run the API.

## Watching and running

```
docker-compose run --rm --service-ports scrobbler-dev
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

## Building for ARM

I've wasted more than 6 hours tryin got get it to build for ARM.

### Travis CI

The CI allows native building on arm so I can just specify the architecture and let it build for me. This is the simplest solution.

### Compile on Raspberry Pi

Gets completely stuck when trying to compile `hyper`, I've left it compiling for about half a day before the shell completely locks up and I can't even SSH into the thing.

### Cross-compile

Trying to cross-compile really sucks. I tried all kinda of things.

#### Specify docker platform

As of Docker v19.03.13, `docker buildx` can only be used by enabling experimental features.

```
docker buildx build --platform linux/arm/v7 .
```

#### Specify target

It seems like specifying the target should work:

```
FROM rust:1.49

ARG TARGET="arm-unknown-linux-gnueabihf"

WORKDIR /app

RUN rustup default nightly
RUN rustup target add $TARGET
RUN apt-get update
RUN apt-get install build-essential -y
RUN mkdir .cargo

Will end up with errors like:

COPY . .

RUN cargo build --target=$TARGET
```

But this just errors with:

```
error: linking with `cc` failed: exit code: 1
```

If I try specifying a linker then I get something like this:

```
error: linker `aarch64-unknown-linux-musl` not found
```