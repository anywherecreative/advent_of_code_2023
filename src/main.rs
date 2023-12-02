use std::fs;

fn main() {
    //day_one_part_one();
    day_one_part_two();
}

#[allow(dead_code)]
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

fn day_one_part_two() {
    let mut total = 0;
    let numbers = ["one","two","three","four","five","six","seven","eight","nine"];

    for line in fs::read_to_string("day_one_part_two.txt").unwrap().lines() {
        //replace words with numbers
        let new_line = line.to_string();
        // println!("{}",new_line);
        let mut number = 0;
        let mut letters = "".to_string();
        //get first char
        'letters:for c in new_line.chars() {
            if c.is_numeric() {
                number += c.to_digit(10).unwrap() * 10;
                break;
            }
            else {
                letters += &c.to_string();
                for (i,num) in numbers.iter().enumerate() {
                    if letters.contains(num) {
                        number += (i as u32 +1) * 10;
                        break 'letters;
                    }
                }
            }
        }

        letters = "".to_string();
        'letters:for c in new_line.chars().rev() {
            if c.is_numeric() {
                number += c.to_digit(10).unwrap();
                break;
            }
            else {
                letters.insert_str(0,&c.to_string());
                for (i,num) in numbers.iter().enumerate() {
                    if letters.contains(num) {
                        number += i as u32 +1;
                        break 'letters;
                    }
                }
            }
        }
        total += number;
    }
    print!("{}",total);
}
