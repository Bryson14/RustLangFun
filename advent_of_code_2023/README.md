# Advent of Code 2023

> Rust Edition

# Usage

Since `src/utils.rs` uses a relative path to read in the input files, you must run the program from the root directory of the project.

Run `cargo` from the root `advent_of_code_2023` directory

```bash
cargo run
```

this will run all the day

```bash
cargo run DAY_NUMBER [PART_NUMBER]
```

# Structure

- main.rs takes in the command line arguments and runs the correct day/part
- utils.rs contains some helper functions to read the input files
- dayXX/mod.rs contains boilerplate code for the day
- dayXX/partX.rs contains the solution for the day/part
- dayXX/input.rs contains code to parse the input for the day.
  - Often input.rs will be vital to the solution with the input being parsed into a more useful data structure
  - works in tandem with partX.rs
