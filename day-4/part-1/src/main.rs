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
        let line_index_i32 = line_index as i32;
        let line_len = lines_vec[line_index].len();

        let mut char_index = find_next_x(&lines_vec, line_index, 0, line_len);

        while char_index < line_len {
            x_count += 1;

            xmas_count += count_xmases_from_x(lines_vec, line_index_i32, char_index as i32);

            char_index = find_next_x(&lines_vec, line_index, char_index + 1, line_len); //skip over the actual character found
        }
        line_index += 1;
    }
    println!("Found {x_count} xes, and {xmas_count} xmases.");
}

fn count_xmases_from_x(search: &Vec<String>, y_offset: i32, x_offset: i32) -> i32
{
    let mut found_count = 0;
    for direction in DIRECTIONS
    //let direction = &DIRECTIONS[2]; //test finding XMAS horizontally
    //let direction = &DIRECTIONS[6]; //test finding SAMX horizontally
    {
        if find_xmas(search, y_offset, x_offset, &direction)
        {
            found_count += 1;
        }
    }
    found_count
}

struct DirectionElem {
    y_increment: i32,
    x_increment: i32
}

const DIRECTIONS: [DirectionElem; 8] = [
    DirectionElem { y_increment: -1, x_increment:  0}, //North
    DirectionElem { y_increment: -1, x_increment:  1}, //Northeast
    DirectionElem { y_increment:  0, x_increment:  1}, //East
    DirectionElem { y_increment:  1, x_increment:  1}, //Southeast
    DirectionElem { y_increment:  1, x_increment:  0}, //South
    DirectionElem { y_increment:  1, x_increment: -1}, //Southwest
    DirectionElem { y_increment:  0, x_increment: -1}, //West
    DirectionElem { y_increment: -1, x_increment: -1}, //Northwest
];

fn find_xmas(search: &Vec<String>, y_offset: i32, x_offset: i32, direction: &DirectionElem) -> bool
{
    //guard band, wow this took a long tim to get right, and I'm still not entirely sold that it makes sense
    if direction.y_increment.is_positive() && (y_offset > (search.len() - 5) as i32)
    {
        return false;
    }
    if direction.y_increment.is_negative() && y_offset < 3
    {
        return false;
    }
    let x_offset_usize = x_offset as usize;
    if direction.x_increment.is_positive() && x_offset_usize > (search[x_offset_usize].len() - 4)
    {
        return false;
    }
    if direction.x_increment.is_negative() && x_offset < 3
    {
        return false;
    }

    for i  in 1..4 {
        /* these variables are all to make debugging easier - the RustRover debugger seems
           to not have a decent evaluator so it was much easier to just watch them.
         */
        let line_num = (y_offset + (direction.y_increment * i)) as usize;
        let line = &search[line_num];
        let offset = (x_offset + (direction.x_increment * i)) as usize;
        let check_char = &line[offset..offset + 1];
        let xmas_char = &XMAS_STRING[(i as usize)..(i+1) as usize];
        if check_char != xmas_char
        {
            return false;
        }
    }

    println!("Found XMAS on line {}, char {}, direction y-increment {}, direction x-increment {}", y_offset + 1, x_offset + 1, direction.y_increment, direction.x_increment);

    true
}

fn find_next_x(search: &Vec<String>, y_offset: usize, x_offset: usize, line_len: usize) -> usize
{
    let find_result = search[y_offset][x_offset..].find("X");
    match find_result
    {
        Some(x) => x + x_offset,
        None => line_len
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename).unwrap().split("\n").map(|x| x.to_string()).collect()
}