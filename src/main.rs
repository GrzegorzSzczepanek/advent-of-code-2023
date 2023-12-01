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
        let length: usize = instruction.len();
        let mut indices: usize = 0;
        let mut num1 = String::new();
        let mut num2 = String::new();

        while indices < length {
            for digit in digits {
                if &instruction[0..indices].contains(digit) {
                    num1.push(digit);
                }
            }
        }
        indices += 1;
    }

    return result;
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let data: Vec<String> = input.split("\n").map(|line| line.to_string()).collect();

    let part_1_result = part_1(&data);
    println!("{part_1_result}");
    let part_2_result = part_2(&data);
}
