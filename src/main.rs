use std::fs;

fn main() {
    day_one_part_one();
}

fn day_one_part_one() {
    // let mut contents = Vec::new();
    let mut total = 0;
    for line in  fs::read_to_string("day_one_part_one.txt").unwrap().lines() {
        let mut number = 0;
        //get first char
        for c in line.chars() {
            if c.is_numeric() {
                number += c.to_digit(10).unwrap() * 10;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_numeric() {
                number += c.to_digit(10).unwrap();
                break;
            }
        }
        total += number;
    }
    print!("{}",total);
}