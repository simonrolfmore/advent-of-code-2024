mod input_line;

use crate::input_line::InputLine;
use std::fs;
use std::str::FromStr;

fn main() {
    let parsed: InputLine = "85570   59882".parse().unwrap();


    let fucked = InputLine::from_str("hello");
    assert!(fucked.is_err());

    println!(
        "Left val is {}, right val is {}.",
        parsed.left_val, parsed.right_val
    );

    let sorted_vecs = read_input_into_sorted_vecs("./input.txt");

    println!("Parsed {} lines", sorted_vecs.right.len());
    let total_distance = calc_total_distance(sorted_vecs.left, sorted_vecs.right);

    println!("Total distance: {}", total_distance);
}

struct TwoVecs {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn read_input_into_sorted_vecs(file_name: &str) -> TwoVecs
{
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
    return TwoVecs{left: left_values, right: right_values};
}

fn calc_total_distance(left: Vec<i32>, right: Vec<i32>) -> i32
{
    let mut cumulated_distance = 0;
    for i in 0..left.len() {
        cumulated_distance += get_distance(left[i], right[i]);
    }

    return cumulated_distance;
}

fn get_distance(left_val: i32, right_val: i32) -> i32
{
    i32::abs(left_val - right_val)
}