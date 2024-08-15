//! Naive solution using iterators, mostly relies on compiler optimizations to be decently fast.

use clap::Parser;
use rand::{thread_rng, Rng};
use std::time::Instant;
use tinyrand::{Probability, Rand, Seeded};

#[derive(Parser)]
struct Args {
    /// Number of threads to use
    #[arg(short, long, default_value = "8")]
    pub threads: u64,
    /// Number of trials to perform
    #[arg(short, long, default_value = "1000000000")]
    pub amt: u64,
    /// Probability of paralysis
    #[arg(short, long, default_value = "0.25")]
    pub prob: f64,
}

fn main() {
    let Args { amt, threads, prob } = Args::parse();
    let prob = Probability::new(prob);
    let start = Instant::now();
    let threads: Vec<_> = (0..threads)
        .map(|_| {
            let mut rng = tinyrand::Wyrand::seed(thread_rng().gen());
            std::thread::spawn(move || {
                let amt = amt / threads;
                (0..amt)
                    .map(|_| (0..231).filter(|_| rng.next_bool(prob)).count())
                    .max()
                    .unwrap_or(0)
            })
        })
        .collect();
    let max = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .max()
        .unwrap_or(0);
    let time = start.elapsed();
    println!(
        "Max {max} after {:#?} ({} trials/sec)",
        time,
        1000 * amt as u128 / time.as_millis()
    )
}
