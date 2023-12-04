use std::fs;
use substring::Substring;

fn main() {
    //day_one_part_one();
    // day_one_part_two();
    // day_two_part_one();
    // day_two_part_two();
    // day_three_part_one();
    day_four_part_one();
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn day_two_part_one() {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let mut allowed_games:Vec<u32> = Vec::new();
    for line in  fs::read_to_string("day_two_part_one.txt").unwrap().lines() {
        //get the id
        let id:u32 = line.substring(5,line.find(":").unwrap()).parse::<u32>().unwrap();
        let remaining = line.substring(line.find(":").unwrap()+2, line.len()).replace(";",",").replace(", ",",");
       let mut allow = true;
        for part in remaining.split(",") {
            let number = part.substring(0,part.find(" ").unwrap()).parse::<u32>().unwrap();
            let color = part.substring(part.find(" ").unwrap()+1,part.len());

            if color == "red" && number > MAX_RED {
               allow = false;
            }
            if color == "green" && number > MAX_GREEN {
                allow = false;
            }
            if color == "blue" && number > MAX_BLUE {
                allow = false;
            }
        }
        if allow {
            allowed_games.push(id);
        }
    }
    print!("{}",allowed_games.iter().sum::<u32>());
}

#[allow(dead_code)]
fn day_two_part_two() {
    let mut sum_powers: u32 = 0;
    for line in fs::read_to_string("day_two_part_two.txt").unwrap().lines() {
        //get the id
        // let id: u32 = line.substring(5, line.find(":").unwrap()).parse::<u32>().unwrap();
        let remaining = line.substring(line.find(":").unwrap() + 2, line.len()).replace(";", ",").replace(", ", ",");
        //going with high numbers so we don't have to test for 0
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;
        for part in remaining.split(",") {
            let number = part.substring(0, part.find(" ").unwrap()).parse::<u32>().unwrap();
            let color = part.substring(part.find(" ").unwrap() + 1, part.len());
            if color == "red" && number > min_red {
                min_red = number;
            }
            if color == "green" && number > min_green {
                min_green = number;
            }
            if color == "blue" && number > min_blue {
                min_blue = number;
            }
        }
        sum_powers += min_red * min_green * min_blue;
    }
    print!("{}",sum_powers);
}

#[allow(dead_code)]
fn day_three_part_one() {
    let mut lines = Vec::new();

    for (i,l) in fs::read_to_string("day_three_part_one.txt").unwrap().lines().enumerate() {
        lines.push(Vec::new());
        for c in l.chars() {
            lines[i].push(c);
        }
    }

    let mut sum_of_number = 0;
    for (i,line) in lines.iter().enumerate() {
        let mut current_number = 0;
        let mut touching_symbol = false;
        for (n,entry) in line.iter().enumerate() {
            println!("symbol: {}", entry);

            if entry.is_numeric() {
                println!("it's a number");
                //offset by 10
                current_number = current_number * 10;
                //add next digit
                current_number += entry.to_digit(10).unwrap();

                println!("current number: {}",current_number);

                /*
                    check around the number via North East South West where possible
                    for a symbol, if one is found set the touching_flag
                 */

                //north
                if i > 0 {
                    //left
                    if n > 0 {
                        let symbol = lines[i - 1][n - 1];
                        println!("north left {} ", symbol);

                        if !symbol.is_numeric() && symbol != '.' {
                            //it's a symbol

                            touching_symbol = true;
                        }
                    }
                    //center
                    let symbol = lines[i - 1][n];
                    println!("north center {} ", symbol);
                    if !symbol.is_numeric() && symbol != '.' {
                        //it's a symbol
                        touching_symbol = true;
                    }
                    //right
                    if n < line.len()-1 {
                        let symbol = lines[i - 1][n + 1];
                        println!("north right {} ", symbol);
                        if !symbol.is_numeric() && symbol != '.' {
                            //it's a symbol
                            touching_symbol = true;
                        }
                    }

                }

                //East
                if n < line.len()-1 {
                    let symbol = lines[i][n + 1];
                    println!("east {} ", symbol);
                    if !symbol.is_numeric() && symbol != '.' {
                        println!("touched!");
                        //it's a symbol
                        touching_symbol = true;
                    }
                }

                //South
                if i < lines.len()-1 {
                    //left
                    if n > 0 {
                        let symbol = lines[i + 1][n - 1];
                        println!("south left {} ", symbol);
                        if !symbol.is_numeric() && symbol != '.' {
                            println!("touched!");
                            //it's a symbol
                            touching_symbol = true;
                        }
                    }
                    //center
                    let symbol = lines[i + 1][n];
                    println!("south center {} ", symbol);
                    if !symbol.is_numeric() && symbol != '.' {
                        println!("touched!");
                        //it's a symbol
                        touching_symbol = true;
                    }
                    //right
                    if n < line.len()-1 {
                        let symbol = lines[i + 1][n + 1];
                        println!("south right {} ", symbol);

                        if !symbol.is_numeric() && symbol != '.' {
                            println!("touched!");
                            //it's a symbol
                            touching_symbol = true;
                        }
                    }

                }

                //west
                if n > 0 {
                    let symbol = lines[i][n - 1];
                    println!("west {} ", symbol);
                    if !symbol.is_numeric() && symbol != '.' {
                        println!("touched!");
                        //it's a symbol
                        touching_symbol = true;
                    }
                }
            }
            else {
                //the current number has ended, if we touched a symbol add it to the sum
                println!("not a number");
                if touching_symbol {
                    sum_of_number += current_number;
                    println!("adding {}",current_number);
                }

                current_number = 0;
                touching_symbol = false;
            }
        }
    }

    print!("{}",sum_of_number);
}

fn day_four_part_one() {
    let mut cards = 0;
    let binding = fs::read_to_string("day_four_part_one.txt").unwrap();
    let mut lines:Vec<&str> = binding.lines().collect::<Vec<_>>();
    for (_i,line) in lines.iter().enumerate() {
        //break the string into numbers and winning numbers
        let numbers     = line.substring(line.find(": ").unwrap()+1,line.find("|").unwrap());
        let winning_numbers = line.substring(line.find("| ").unwrap()+1,line.len());
        let mut wins = 0;
        for number in (0..numbers.len()-2).step_by(3) {
            let num = numbers.substring(number,number+3);
            for winner in (0..winning_numbers.len()-2).step_by(3) {
                let win = winning_numbers.substring(winner,winner+3);
                if win == num {
                    wins+=1;
                    lines.push(lines[number+wins].clone());
                }
            }
        }
        cards+=1;
        if cards > 50 {
            break;
        }
    }

    println!("{}",cards);
}