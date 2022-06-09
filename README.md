# Read Files in Parallel

This tool is meant to compare performance among a variety implementations for reading files in parallel.

## Usage

```sh
# Run the code, passing in a path to a directory to read through
$ cargo run -- /path/to/directory

# Run the code, reading through the files contained within this project
$ cargo run
```

## Implementations

The first implementation of the algorithm is a base case. It reads through the files serially.

The remaining implementations are, in the end, compared to the base case. These implementations attempt to compare:

- native threads vs green threads
- sync vs async design
- channels vs join handle return values
