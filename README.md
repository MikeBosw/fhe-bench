How long does an FHE multiplication take compared to a multiplication in the clear?*

The answer is: 40,000,000x longer, running this benchmark on a MacBook M3 Pro.

Zama is the library used.

-----------

```
cargo run --release

    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/FHE-test`
A random non-FHE value: 3938731157540726032
FHE value: 325
Diff between normal op (29.3ns) and FHE op (1156ms): 39458390x
```

\* biased in favor of FHE because we include generation of two random numbers in the non-FHE time measurement
