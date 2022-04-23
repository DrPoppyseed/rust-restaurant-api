# Rust Restaurant API

This is a simple API written in Rust, implementing basic CRUD functionalities.
The goal in writing this API was to use as few libraries as possible, resulting in the use of only the following:

- `serde & serde_json`... for serializing and deserializing json
- `actix-web`... for making the http server
- `mongodb`... mongodb driver for rust
- `env_logger`... logger

## Getting Started

To spin up the development environment, set up a `.env` file with the variables included in `.env.example`.
Then, run the application with the following commands.

```bash
$ make start
```

To run tests on the API, run the following command.

```bash
$ make test
```
