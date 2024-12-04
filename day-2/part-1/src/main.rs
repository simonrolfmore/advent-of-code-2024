use crate::Direction::{Decreasing, Increasing, Unknown};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::time::Instant;

fn main() {
    let verbose = true; //config for reasons output, too lazy to do it properly

    let run_time = Instant::now();
    let mut safe_lines = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            let line_borrowed = &line;
            let test_values = parse_line(line_borrowed);
            let safety_reason = is_safe(test_values);

            if safety_reason.is_safe {
                if verbose {
                    println!("Line {line_borrowed} is safe");
                }
                safe_lines += 1;
            } else {
                if verbose {
                    println!("Line {line_borrowed} is not safe: {}", safety_reason.reason);
                }
            }
        }
        println!(
            "Found {safe_lines} safe lines in {}us",
            run_time.elapsed().as_micros()
        );
    } else {
        println!("Failed to find file input.txt");
    }

    println!("Press enter to exit.");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &String) -> Vec<i8> {
    line.split_whitespace()
        .map(|x| x.parse::<i8>().unwrap())
        .collect()
}

struct SafetyReason {
    is_safe: bool,
    reason: String,
}

#[derive(PartialEq)]
enum Direction {
    Unknown,
    Increasing,
    Decreasing,
}

fn is_safe(test_values: Vec<i8>) -> SafetyReason {
    let mut prev_val;
    let mut direction: Direction = Unknown;
    let mut index = 0;
    while index < test_values.len() - 1 {
        prev_val = test_values[index];
        index += 1;
        let val = test_values[index];
        if val == prev_val {
            return SafetyReason {
                is_safe: false,
                reason: format!("{val} is equal to previous value"),
            };
        }
        if prev_val > val {
            if prev_val - val > 3 {
                return SafetyReason {
                    is_safe: false,
                    reason: format!("Value increases by over 3 from {prev_val} to {val}"),
                };
            }
            if direction == Decreasing {
                return SafetyReason {
                    is_safe: false,
                    reason: "Was decreasing, now increasing".to_string(),
                };
            }
            direction = Increasing;
            continue;
        }

        if prev_val < val {
            if val - prev_val > 3 {
                return SafetyReason {
                    is_safe: false,
                    reason: format!("Value decreases by over 3 from {prev_val} to {val}"),
                };
            }
            if direction == Increasing {
                return SafetyReason {
                    is_safe: false,
                    reason: "Was increasing, now decreasing".to_string(),
                };
            }
            direction = Decreasing;
            continue;
        }
    }
    SafetyReason {
        is_safe: true,
        reason: "".to_string(),
    }
}
