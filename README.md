# prng-bench

Benchmarking different methods to generate 1GB of random bytes in Rust.
- [`rand`](https://docs.rs/rand/latest/rand/)
- [`SmallRng`](https://docs.rs/rand/latest/rand/rngs/struct.SmallRng.html)
- [`rand_xoshiro`](https://docs.rs/rand_xoshiro/latest/rand_xoshiro/)
- [`nanorand`](https://docs.rs/nanorand/latest/nanorand/)
- [`fastrand`](https://docs.rs/fastrand/latest/fastrand/)

## On an AMD Ryzen 9 6900HX

| PRNG          | method        | ns / byte     |
| :-            | :-            | -:            |
| rand          | collect `u8`  | 2.36          |
| rand          | `fill_bytes`  | 0.26          |
| SmallRng      | `fill_bytes`  | 0.09          |
| Xoshiro256**  | `fill_bytes`  | 0.09          |
| Xoshiro256++  | `fill_bytes`  | 0.09          |
| Xoshiro512**  | `fill_bytes`  | 0.10          |
| Xoshiro512++  | `fill_bytes`  | 0.11          |
| nanorand      | `fill_bytes`  | 0.30          |
| fastrand      | `fill`        | 0.09          |

## On an Apple M1

| PRNG          | method        | ns / byte     |
| :-            | :-            | -:            |
| rand          | collect `u8`  | 6.63          |
| rand          | `fill_bytes`  | 1.11          |
| SmallRng      | `fill_bytes`  | 0.17          |
| Xoshiro256**  | `fill_bytes`  | 0.16          |
| Xoshiro256++  | `fill_bytes`  | 0.14          |
| Xoshiro512**  | `fill_bytes`  | 0.17          |
| Xoshiro512++  | `fill_bytes`  | 0.15          |
| nanorand      | `fill_bytes`  | 0.27          |
| fastrand      | `fill`        | 0.04          |
