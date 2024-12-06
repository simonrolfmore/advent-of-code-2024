use std::fs;
use regex::Regex;

fn main() {
    let regex_matcher = Regex::new(r"mul\((?<first_num>\d+),(?<second_num>\d+)\)");
    let file_contents = fs::read_to_string("./input.txt").unwrap();
    let verbose = true;

    let mut total = 0;

    for regex_match in regex_matcher.unwrap().captures_iter(file_contents.as_str()) {
        let first_num:i32 = regex_match["first_num"].parse().unwrap();
        let second_num:i32 = regex_match["second_num"].parse().unwrap();

        if verbose {
            println!("Found a mul! First num: {}, second num: {}", first_num, second_num);
        }

        total += first_num * second_num;
    }

    println!("Total of all mul instructions is {total}");
}
