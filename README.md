# Code Challenge 3: Lucky Numbers

This program can count the number of occurences of a given lucky number in a given range.

- [Code Challenge 3: Lucky Numbers](#code-challenge-3-lucky-numbers)
  - [How to run](#how-to-run)
    - [Prerequisites](#prerequisites)
    - [Run tests](#run-tests)
    - [Run benchmarks](#run-benchmarks)
    - [Run development build](#run-development-build)
    - [Run release build](#run-release-build)
    - [Sample request](#sample-request)
  - [Project setup](#project-setup)

## How to run

### Prerequisites

[Install Rust](https://www.rust-lang.org/tools/install).

### Run tests

```shell
cargo test --workspace
```

### Run benchmarks

```shell
cargo bench
```

### Run development build

```shell
cargo run
```

### Run release build

```shell
cargo run --release
```

### Sample request

```shell
curl "localhost:3030/lucky_numbers?start=1&end=1000&sequence=123"
```

## Project setup

The project is set up as a workspace, containing two "crates":

- `count-lucky-numbers` is a library that exports a function called `count_lucky_numbers`, which counts the occurences of lucky numbers.
- `count-lucky-numbers-server` is a binary that starts a web server with an endpoint that wraps the `count_lucky_numbers` function. (This crate depends on the `count-lucky-numbers` crate.)
