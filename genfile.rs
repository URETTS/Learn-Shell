use rand::{thread_rng, Rng};
use std::env;
use std::fs::write;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: cargo run <n> <w> <o> <f>");
        exit(1);
    }

    let n: usize = args[1].parse().expect("n must be an integer");
    let w = args[2].as_str();
    let o = args[3].as_str();
    let f = &args[4];

    let mut rng = thread_rng();
    let mut data: Vec<String> = match w {
        "n" => (0..n)
            .map(|_| rng.gen_range(1..=1_000_000).to_string())
            .collect(),
        "s" => (0..n).map(|_| random_string(8)).collect(),
        _ => {
            eprintln!("Error: w must be 'n' (numbers) or 's' (strings)");
            exit(1);
        }
    };

    match o {
        "i" => data.sort(),
        "d" => data.sort_by(|a, b| b.cmp(a)),
        "r" => {}
        _ => {
            eprintln!("Error: o must be 'i', 'd', or 'r'");
            exit(1);
        }
    }

    write(f, data.join("\n")).expect("Error writing to file");
}

fn random_string(len: usize) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = thread_rng();
    (0..len)
        .map(|_| {
            charset
                .chars()
                .nth(rng.gen_range(0..charset.len()))
                .unwrap()
        })
        .collect()
}
