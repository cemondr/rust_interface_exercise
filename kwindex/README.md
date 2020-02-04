## KWINDEX by Cem Onder

- Given a collection of documents stored in memory, stores each word as a slice into its original text.

- Provides an interface for such structure with the ability to:
    - create a new structure
    - extend from a given text
    - count the number of matching words

- Program tests are included in:  `tests/libtest.rs`
    - eaily run with `cargo test`

- There is a small example use in: `examples/example.rs`     
    - easily run with `cargo run -example example`