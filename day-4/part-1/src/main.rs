use std::fs;
use std::path::Path;
use std::string::ToString;

const XMAS_STRING: &str = "XMAS";

fn main() {
    let lines_vec = &read_lines("input.txt");
    let mut x_count = 0;
    let mut xmas_count = 0;
    let mut line_index = 0;
    while line_index < lines_vec.len() {
        let mut char_index = find_next_x(&lines_vec, line_index, 0);
        while char_index < lines_vec[line_index].len() {
            x_count += 1;

            xmas_count += count_xmases_from_x(lines_vec, line_index, char_index);

            char_index = find_next_x(&lines_vec, line_index, char_index + 1); //skip over the actual character found
        }
        line_index += 1;
    }
    println!("Found {x_count} xes, and {xmas_count} xmases.");
}

fn count_xmases_from_x(search: &Vec<String>, y_offset: usize, x_offset: usize) -> i32
{
    let mut found_count = 0;
    if find_xmas_left_to_right(search, y_offset, x_offset)
    {
        found_count += 1;
    }
    if find_xmas_right_to_left(search, y_offset, x_offset)
    {
        found_count += 1;
    }
    if find_xmas_top_to_bottom(search, y_offset, x_offset)
    {
        found_count += 1;
    }
    if find_xmas_bottom_to_top(search, y_offset, x_offset)
    {
        found_count += 1;
    }

    found_count
}

//this entire set of functions suck, but hey
fn find_xmas_left_to_right(search: &Vec<String>, y_offset: usize, x_offset: usize) -> bool
{
    let line = &search[y_offset];
    if x_offset > (line.len() - 4) // can't get Xmas without having enough room
    {
        return false;
    }
    if &line[x_offset..x_offset + 4] == XMAS_STRING
    {
        return true;
    }

    false
}

fn find_xmas_right_to_left(search: &Vec<String>, y_offset: usize, x_offset: usize) -> bool
{
    let line = &search[y_offset];
    if x_offset < 3 // can't get Xmas without having enough room
    {
        return false;
    }
    if line[x_offset - 1..x_offset] == XMAS_STRING[1..2]
    {
        if line[x_offset - 2 .. x_offset - 1] == XMAS_STRING[2..3]
        {
            if line[x_offset - 3 .. x_offset - 2] == XMAS_STRING[3..4]
            {
                return true;
            }
        }
    }

    false
}

fn find_xmas_top_to_bottom(search: &Vec<String>, y_offset: usize, x_offset: usize) -> bool
{
    if y_offset > (search.len() - 5)
    {
        return false;
    }

    if search[y_offset + 1][x_offset..x_offset + 1] == XMAS_STRING[1..2]
    {
        if search[y_offset + 2][x_offset..x_offset + 1] == XMAS_STRING[2..3]
        {
            if search[y_offset + 3][x_offset..x_offset + 1] == XMAS_STRING[3..4]
            {
                return true;
            }
        }
    }

    false
}

fn find_xmas_bottom_to_top(search: &Vec<String>, y_offset: usize, x_offset: usize) -> bool
{
    if y_offset < 4
    {
        return false;
    }

    if search[y_offset - 1][x_offset..x_offset + 1] == XMAS_STRING[1..2]
    {
        if search[y_offset - 2][x_offset..x_offset + 1] == XMAS_STRING[2..3]
        {
            if search[y_offset - 3][x_offset..x_offset + 1] == XMAS_STRING[3..4]
            {
                return true;
            }
        }
    }

    false
}

fn find_next_x(search: &Vec<String>, y_offset: usize, x_offset: usize) -> usize
{
    let find_result = search[y_offset][x_offset..].find("X");
    match find_result
    {
        Some(x) => x + x_offset,
        None => search.len()
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename).unwrap().split("\n").map(|x| x.to_string()).collect()
}