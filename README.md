# Advent of Code 2023
*in Rust*

# Getting input examples

## `utils::get_example_input`
Loads text file from the `inputs` folder.

```rust
//  Loads file inputs/01-first.txt
let input = utils::get_example_input("01-first.txt").unwrap();
```

## `utils::get_full_input`
Loads your input from AoC API.

Requires you to set environmental variable `AOC_SESSION`
or create `.env` file using the `.env.example` template.

```rust
//  Loads input for the AoC on 1st December 2023
let full = utils::get_full_input(2023, 1).unwrap();
```