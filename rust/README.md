# Rust

## Solution

Create a file `src/lib.rs`:

```rust
pub fn fizzbuzz() -> Vec<String> {
    (1..=100).map(|i| {
        if i % 15 == 0 {
            "FizzBuzz".to_string()
        } else if i % 3 == 0 {
            "Fizz".to_string()
        } else if i % 5 == 0 {
            "Buzz".to_string()
        } else {
            i.to_string()
        }
    }).collect()
}
```

## Running Tests

```bash
cargo test
```
