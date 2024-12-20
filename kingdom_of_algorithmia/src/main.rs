mod challenge;
pub mod util;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use challenge::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut c1 = c1::C1::new();
    let mut challenges: Vec<&mut dyn Challenge> = vec![&mut c1];
    let mut run_behavior = RunBehavior::Last;

    if args.len() == 2 {
        let id = args[1].parse::<usize>().expect(&usage());
        run_behavior = RunBehavior::Specific(id);
    }

    match run_behavior {
        RunBehavior::Last => {
            let id = challenges.len() - 1;
            run_challenge(challenges[id]);
        }

        RunBehavior::Specific(id) => {
            run_challenge(challenges[id]);
        }
    }
}

fn run_challenge<T: Challenge + ?Sized>(challenge: &mut T) {
    println!("===== {} =====", challenge.name());
    let start = Instant::now();
    challenge.parse_input();
    let time = start.elapsed();
    print_with_time(PrintType::ParsingInput, "", time);

    let start = Instant::now();
    let part1 = challenge.part1();
    let time = start.elapsed();
    print_with_time(PrintType::Part1, part1, time);

    let start = Instant::now();
    let part2 = challenge.part2();
    let time = start.elapsed();
    print_with_time(PrintType::Part2, part2, time);

    let start = Instant::now();
    let part3 = challenge.part3();
    let time = start.elapsed();
    print_with_time(PrintType::Part3, part3, time);
}

fn print_with_time(print_type: PrintType, output: impl Display, time: Duration) {
    let time = time.as_millis();
    let sec = time / 1000;
    let msec = time % 1000;
    let time_str = format!("{sec:3}.{msec:04}");
    match print_type {
        PrintType::ParsingInput => {
            println!("{time_str} Parsing input");
        }
        PrintType::Part1 => {
            println!("{time_str} Part 1: {output}");
        }
        PrintType::Part2 => {
            println!("{time_str} Part 2: {output}");
        }
        PrintType::Part3 => {
            println!("{time_str} Part 3: {output}");
        }
    }
}

fn usage() -> String {
    "Usage: cargo run --release [challenge number]".to_string()
}
