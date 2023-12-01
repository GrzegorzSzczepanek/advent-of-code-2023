use std::fs::read_to_string;

fn part_1(instructions: &Vec<String>) -> i32 {
    let mut result: i32 = 0;

    for instruction in instructions {
        let mut num = String::new();

        for c in instruction.chars() {
            if c.is_digit(10) {
                num.push(c);
                break;
            }
        }
        for c in instruction.chars().rev() {
            if c.is_digit(10) {
                num.push(c);
                break;
            }
        }
        let num_i32: Result<i32, std::num::ParseIntError> = num.parse();

        match num_i32 {
            Ok(n) => {
                result += n;
                // println!("{}", n);
            }
            Err(e) => println!("Failed to parse string: {}", e),
        }
    }

    return result;
}

fn part_2(instructions: &Vec<String>) -> i32 {
    let mut result: i32 = 0;

    let digits: Vec<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    for instruction in instructions {
        let mut last: usize = if instruction.is_empty() {
            0
        } else {
            instruction.len() - 1
        };
        let mut indices: usize = 0;
        let mut left_not_found: bool = true;
        let mut right_not_found: bool = true;

        let mut concatenated_nums = String::new();
        while indices < instruction.len() && last != 0 {
            if left_not_found {
                // println!("{instruction}");

                for digit in &digits {
                    if instruction[0..indices + 1].contains(digit) {
                        let _x: &str = &instruction[0..indices + 1];
                        let digit_value = match digit.as_str() {
                            "one" => "1",
                            "two" => "2",
                            "three" => "3",
                            "four" => "4",
                            "five" => "5",
                            "six" => "6",
                            "seven" => "7",
                            "eight" => "8",
                            "nine" => "9",
                            _ => continue,
                        };

                        concatenated_nums.push_str(digit_value);
                        left_not_found = false;
                        break;
                    } else if let Some(n) = instruction.chars().nth(indices) {
                        if n.is_digit(10) {
                            concatenated_nums.push(n);
                            // indices = 0;
                            left_not_found = false;
                            break;
                        }
                    }
                }

                indices += 1;
            }
            if right_not_found {
                for digit in &digits {
                    if instruction[last..instruction.len()].contains(digit) {
                        right_not_found = false;
                        let digit_value = match digit.as_str() {
                            "one" => "1",
                            "two" => "2",
                            "three" => "3",
                            "four" => "4",
                            "five" => "5",
                            "six" => "6",
                            "seven" => "7",
                            "eight" => "8",
                            "nine" => "9",
                            _ => continue,
                        };
                        concatenated_nums.push_str(digit_value);
                        last = 0;
                        right_not_found = false;
                        break;
                    } else if let Some(n) = instruction.chars().nth(last) {
                        if n.is_digit(10) {
                            concatenated_nums.push(n);
                            last = 0;
                            right_not_found = false;
                            break;
                        }
                    }
                }
                if last != 0 {
                    last -= 1;
                }
            }
        }
        println!("{concatenated_nums}, {instruction}");
        let concatenated_nums: Result<i32, std::num::ParseIntError> = concatenated_nums.parse();

        match concatenated_nums {
            Ok(n) => result += n,
            Err(e) => println!("Failed to parse string to i32: {}", e),
        }
    }

    return result;
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let data: Vec<String> = input.split("\n").map(|line| line.to_string()).collect();

    // let part_1_result = part_1(&data);
    // println!("{part_1_result}");
    let part_2_result = part_2(&data);
    println!("{part_2_result}")
}
