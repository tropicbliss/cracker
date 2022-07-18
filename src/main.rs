use anyhow::Result;
use clap::Parser;
use rand::{prelude::SmallRng, Rng, SeedableRng};
use std::num::Wrapping;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(&args.regex)?;
    let gen = rand_regex::Regex::with_hir(hir, 100)?;
    let available_parallelism = std::thread::available_parallelism()?;
    let mut handles = Vec::new();
    if args.sample {
        let mut rng = SmallRng::from_entropy();
        let seed_string: String = (&mut rng).sample(&gen);
        println!("Sample: {seed_string}");
        println!();
    }
    for _ in 0..usize::from(available_parallelism) {
        let gen = gen.clone();
        let handle = std::thread::spawn(move || {
            let mut rng = SmallRng::from_entropy();
            loop {
                let seed_string: String = (&mut rng).sample(&gen);
                let seed = java_hashcode(&seed_string);
                if seed == args.seed {
                    println!("Found seed: {seed_string}");
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Generates random strings based on this regex
    #[clap(short, long)]
    regex: String,

    /// Seed to compare against
    #[clap(short, long)]
    seed: i32,

    /// Gives the maximum extra repeat counts the x*, x+ and x{n,} operators will become
    #[clap(short, long, default_value = "100")]
    max_repeat: usize,

    /// Prints a sample generated seed string before attempting to find suitable seed strings
    #[clap(long)]
    sample: bool,
}

fn java_hashcode(string: &str) -> i32 {
    let mut hash = Wrapping(0);
    for ch in string.chars() {
        hash = Wrapping(31) * hash + Wrapping(ch as i32);
    }
    hash.0
}
