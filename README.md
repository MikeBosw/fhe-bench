How long does an FHE multiplication take compared to a non-FHE one?*

Here is a very dumb, very naive answer: ~75,000x longer.

-----------

```
cargo run --color=always --package FHE-test --bin FHE-test --profile release
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/FHE-test`
A random non-FHE value: 12580
FHE value: 325
Diff between normal op and FHE op: 78802x

Process finished with exit code 0
```

\* on my MacBook M3, and biased in favor of FHE because we include generation of two random numbers in the non-FHE time measurement
