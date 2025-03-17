# prng-bench

Benchmarking different methods to generate 1GB of random bytes in Rust.
- [`rand`](https://docs.rs/rand/latest/rand/)
- [`SmallRng`](https://docs.rs/rand/latest/rand/rngs/struct.SmallRng.html)
- [`rand_xoshiro`](https://docs.rs/rand_xoshiro/latest/rand_xoshiro/)
- [`nanorand`](https://docs.rs/nanorand/latest/nanorand/)
- [`fastrand`](https://docs.rs/fastrand/latest/fastrand/)

## On an Apple M1

| PRNG | method | ns / byte |
| :- | :- | -: |
| rand | collect `u8` | 6.60 |
| rand | `fill_bytes` | 1.10 |
| SmallRng | `fill_bytes` | 0.14 |
| Xoshiro256** | `fill_bytes` | 0.16 |
| Xoshiro256++ | `fill_bytes` | 0.14 |
| Xoshiro512** | `fill_bytes` | 0.17 |
| Xoshiro512++ | `fill_bytes` | 0.15 |
| nanorand | `fill_bytes` | 0.27 |
| fastrand | `fill` | 0.04 |
