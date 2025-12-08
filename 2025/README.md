# Advent of Code 2025

This is the home of my solutions to the different challenges from [Advent of Code](https://adventofcode.com).

---

## Usage

You should be able to use:

```sh
cargo run --release {day}
```

where `{day}` must be replaced with the day of the challenge you want to run.
By default it will take the input from the `input/local_{day}.txt` file but you can change it to the `input/prod_{day}.txt` file by setting the `ENVIRONMENT` variable to `PRODUCTION`:

```sh
ENVIRONMENT="PRODUCTION" cargo run --release {day}
```

---

## Notes

The challenges are not documented but the main codebase is, to open the documentation use:

```sh
cargo doc --open
```
