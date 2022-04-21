



```rust
#[test]
fn narcissistic_base10() {
    for (i, n) in narcissistic_numbers(10).enumerate() {
        println!("#{}: {}", i + 1, n);
    }
}
```

# References

- https://arxiv.org/pdf/2008.08187.pdf