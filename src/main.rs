use std::env;

use aoc2023::day04;

fn main() {
    let arg = env::args().nth(1).and_then(|s| s.parse::<u8>().ok());

    match arg {
        Some(x) => match run_day(x) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1)
            }
        },
        None => {
            let mut total = std::time::Duration::default();
            for i in 1..=25 {
                let start = std::time::Instant::now();
                run_day(i).unwrap();
                let elapsed = start.elapsed();
                total += elapsed;
                // println!(
                //     r#"{{"day": {}, "total_ms": {}, "delta_ms": {}}},"#,
                //     i,
                //     total.as_millis(),
                //     elapsed.as_millis(),
                // );
                println!(
                    "total so far: {}ms (+{}ms)",
                    total.as_millis(),
                    elapsed.as_millis()
                );
            }
            println!("total time: {}ms", total.as_millis());
        }
    }
}

fn run_day(day: u8) -> Result<(), String> {
    match day {
        4 => print_day("day04", day04::solve()),
        _ => return Err(format!("invalid day {}", day)),
    }
    Ok(())
}

fn print_day<D1, D2>(tag: &str, results: (D1, D2))
where
    D1: std::fmt::Display,
    D2: std::fmt::Display,
{
    let (r1, r2) = results;
    println!("{} part 1: {}\n{} part 2: {}", tag, r1, tag, r2);
}
