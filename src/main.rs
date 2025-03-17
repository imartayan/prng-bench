use core::hint::black_box;
use core::iter::repeat_with;
use nanorand::Rng;
use rand::{RngCore, SeedableRng};
use std::time::Instant;

const LEN: usize = 1_000_000_000;

fn main() {
    #[allow(unused_assignments)]
    let mut buf = vec![0u8; LEN];

    let start = Instant::now();
    buf = repeat_with(rand::random::<u8>).take(LEN).collect();
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "rand collect u8: {:.02} ns / byte",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand::rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "rand fill_bytes: {:.02} ns / byte",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand::rngs::SmallRng::from_os_rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "SmallRng fill_bytes: {:.02} ns / byte",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand_xoshiro::Xoshiro256StarStar::from_os_rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "Xoshiro256** fill_bytes: {:.02} ns / byte",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand_xoshiro::Xoshiro256PlusPlus::from_os_rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "Xoshiro256++ fill_bytes: {:.02} ns / byte",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand_xoshiro::Xoshiro512StarStar::from_os_rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "Xoshiro512** fill_bytes: {:.02} ns / byte",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand_xoshiro::Xoshiro512PlusPlus::from_os_rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "Xoshiro512++ fill_bytes: {:.02} ns / byte",
        elapsed as f64 / LEN as f64
    );

    let mut rng = nanorand::WyRand::new();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "nanorand::WyRand fill_bytes: {:.02} ns / byte",
        elapsed as f64 / LEN as f64
    );

    let mut rng = fastrand::Rng::new();
    let start = Instant::now();
    rng.fill(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "fastrand fill: {:.02} ns / byte",
        elapsed as f64 / LEN as f64
    );
}
