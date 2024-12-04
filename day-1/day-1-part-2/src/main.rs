mod input_line;

use crate::input_line::InputLine;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let load_and_parse_time = Instant::now();

    let sorted_vecs = read_input_into_sorted_vecs("./input.txt");

    println!("Loaded and parsed {} lines in {}us", sorted_vecs.right.len(), load_and_parse_time.elapsed().as_micros());


    let match_time = Instant::now();
    let mut total_similarity_score = 0;
    let mut curr_index = 0;
    let mut prev_value = 0;

    for left_item in sorted_vecs.left {
        if left_item == prev_value {
            continue;
        }
        let count_response = match_score(left_item, sorted_vecs.right.as_ref(), curr_index);
        total_similarity_score += count_response.match_score;
        curr_index = count_response.new_index;
        prev_value = left_item;
    }

    println!("Total similarity score: {} in {}us", total_similarity_score, match_time.elapsed().as_micros());
}

fn match_score(input: i32, list: &Vec<i32>, index: usize) -> CountMatches {
    let mut curr_index = index;
    let mut match_count = 0;
    while curr_index < list.len() {
        if list[curr_index] == input {
            match_count += 1;
        }
        if list[curr_index] > input {
            break;
        }
        curr_index += 1
    }

    println!("Found {match_count} matches for input {input} at {curr_index}");

    CountMatches {
        match_score: match_count * input,
        new_index: curr_index,
    }
}

struct CountMatches {
    match_score: i32,
    new_index: usize,
}

struct TwoVecs {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn read_input_into_sorted_vecs(file_name: &str) -> TwoVecs {
    let file_contents = fs::read_to_string(file_name).unwrap();

    let mut left_values: Vec<i32> = Vec::new();
    let mut right_values: Vec<i32> = Vec::new();

    for line in file_contents.lines() {
        let input_line = InputLine::from_str(line).unwrap();
        left_values.push(input_line.left_val);
        right_values.push(input_line.right_val)
    }

    left_values.sort();
    right_values.sort();
    TwoVecs {
        left: left_values,
        right: right_values,
    }
}
