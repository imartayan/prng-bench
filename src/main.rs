use core::hint::black_box;
use core::iter::repeat_with;
use nanorand::Rng;
use rand::{RngCore, SeedableRng};
use std::time::Instant;

const LEN: usize = 1_000_000_000;

fn main() {
    #[allow(unused_assignments)]
    let mut buf = vec![0u8; LEN];

    eprintln!("| PRNG\t\t| method\t| ns / byte\t|");
    eprintln!("| :-\t\t| :-\t\t| -:\t\t|");

    let start = Instant::now();
    buf = repeat_with(rand::random::<u8>).take(LEN).collect();
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "| rand\t\t| collect `u8`\t| {:.02}\t\t|",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand::rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "| rand\t\t| `fill_bytes`\t| {:.02}\t\t|",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand::rngs::SmallRng::from_os_rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "| SmallRng\t| `fill_bytes`\t| {:.02}\t\t|",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand_xoshiro::Xoshiro256StarStar::from_os_rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "| Xoshiro256**\t| `fill_bytes`\t| {:.02}\t\t|",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand_xoshiro::Xoshiro256PlusPlus::from_os_rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "| Xoshiro256++\t| `fill_bytes`\t| {:.02}\t\t|",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand_xoshiro::Xoshiro512StarStar::from_os_rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "| Xoshiro512**\t| `fill_bytes`\t| {:.02}\t\t|",
        elapsed as f64 / LEN as f64
    );

    let mut rng = rand_xoshiro::Xoshiro512PlusPlus::from_os_rng();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "| Xoshiro512++\t| `fill_bytes`\t| {:.02}\t\t|",
        elapsed as f64 / LEN as f64
    );

    let mut rng = nanorand::WyRand::new();
    let start = Instant::now();
    rng.fill_bytes(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "| nanorand\t| `fill_bytes`\t| {:.02}\t\t|",
        elapsed as f64 / LEN as f64
    );

    let mut rng = fastrand::Rng::new();
    let start = Instant::now();
    rng.fill(&mut buf);
    black_box(&buf);
    let elapsed = start.elapsed().as_nanos();
    eprintln!(
        "| fastrand\t| `fill`\t| {:.02}\t\t|",
        elapsed as f64 / LEN as f64
    );
}
