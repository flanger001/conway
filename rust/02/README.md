# Rust implementation #2

## How to run

```shell
cargo run
```

This runs in the terminal and will wipe the screen before drawing the board.

## Notes

This implementation is significantly slower than #1 because we are allocating a lot more `Vec`s and `String`s. I believe this is due to using the `Position` struct to represent grid coordinates instead of relying on array/Vec indexes, since I currently do not know how to draw a line other than joining a list of strings and printing the result to the screen. However, using `Position` does have some advantages, namely that it _doesn't_ rely on array indexes.
