# pokestats

This program implements the Levenshtein algorithm to perform a SQL-like
`LIKE` search, so it can find the Pokémon whose name is closest to the one
you type, even with typos.

## Usage

Install it and run it from any terminal:

```
cargo install --path .
pokestats pikachu
```

Or run it without installing:

```
cargo run pikachu
```

Example: `pokestats pikaciu` -> Pikachu

## Data

Pokémon up to Generation 6.
