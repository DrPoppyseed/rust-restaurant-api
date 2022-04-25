# Rust Restaurant API

This is a simple API written in Rust, implementing basic CRUD functionalities.
The goal in writing this API was to use as few libraries as possible, resulting in the use of only the following:

- `serde, serde_json` ... for serializing and deserializing json
- `actix-web` ... for making the http server
- `mongodb` ... mongodb driver for rust
- `env_logger` ... logger

## Getting Started

I wrote some small scripts that I found myself using repeatedly in the `Makefile`.
To spin up the development environment, set up a `.env` file with the variables included in `.env.example` or run the
following script to get started with example root usernames and passwords.

```bash
$ make setup_env
```

Then, run the application with the following commands.

```bash
$ make start
```

To run tests on the API, run the following command.

```bash
$ make test
```

## Learnings

To speed up the `Dockerfile` build time, I found an interesting gist that went through explaining how one could
implement some tricks with caching for rust `Dockerfile`s. After implementing the changes, the build time after the
first build was reduced from `~ 5m00s` to `~ 0m30s`.
