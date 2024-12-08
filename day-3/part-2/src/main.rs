use std::fs;
use regex::Regex;

const DO_MATCH: &str = "do()";
const DONT_MATCH: &str = "don't()";

fn main() {
    let verbose = true;
    let file_contents_binding = fs::read_to_string("./input.txt").unwrap();
    let file_contents = file_contents_binding.as_str();
    let regex_matcher = Regex::new(r"mul\((?<first_num>\d+),(?<second_num>\d+)\)").unwrap();
    let mut total = 0;
    let mut do_index = 0;
    let mut dont_index = 0;

    while dont_index < file_contents.len()
    {
        dont_index = do_index + find_next_dont(&file_contents[do_index..]);
        if verbose
        {
            println!("Found a {DONT_MATCH} at index {dont_index}");
        }

        if verbose{
            println!("searching index {do_index} to {dont_index}: {}", &file_contents[do_index..dont_index]);
        }

        //skip over the do()
        total += sum_muls_in_substring(&file_contents[do_index..dont_index], &regex_matcher, verbose);

        do_index = dont_index + find_next_do(&file_contents[dont_index..]);
        if verbose {
            println!("Found a {DO_MATCH} at index {do_index}");
        }
    }

    println!("Total of enabled muls = {total}.");
}

fn find_next_do(search: &str) -> usize
{
    let find_result = search.find(DO_MATCH);
    if find_result.is_none()
    {
        return search.len();
    }

    //skip over the actual do()
    find_result.unwrap() + DO_MATCH.len()
}

fn find_next_dont(search: &str) -> usize
{
    let find_result = search.find(DONT_MATCH);
    if find_result.is_none()
    {
        return search.len();
    }

    //don't skip over the don't() as we don't want to match on it
    find_result.unwrap()
}

fn sum_muls_in_substring(substring: &str, matcher: &Regex, verbose: bool) -> i32
{
    let mut subtotal= 0;
    for regex_match in matcher.captures_iter(substring) {
        let first_num:i32 = regex_match["first_num"].parse().unwrap();
        let second_num:i32 = regex_match["second_num"].parse().unwrap();

        if verbose {
            //println!("Found a mul! First num: {}, second num: {}", first_num, second_num);
        }

        subtotal += first_num * second_num;
    }

    subtotal
}

