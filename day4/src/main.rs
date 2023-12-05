use std::fs::read_to_string;

fn part_1(cards: &Vec<String>) -> i32 {
    let mut result: i32 = 0;

    for card in cards {
        let winning_and_mine: Vec<&str> = card.split(" | ").collect();
        let winning: Vec<i32> = winning_and_mine[0]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mine: Vec<i32> = winning_and_mine[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut count = 0;
        for num in &mine {
            if winning.contains(num) {
                count += 1;
            }
        }
        if count != 0 {
            result += i32::pow(2, count - 1);
        } else if count == 1 {
            result += 1;
        }
    }
    return result;
}

fn main() {
    let data = read_to_string("input.txt").unwrap();
    let mut cards: Vec<String> = data.split("\n").map(|line| line.to_string()).collect();

    cards.retain(|card| card.len() != 0);
    for (i, card) in cards.iter_mut().enumerate() {
        let str_to_replace = format!("{}{}: ", "Card ", i + 1);
        *card = card
            .replace("   ", " ")
            .replace("  ", " ")
            .replace(&str_to_replace, "");
        println!("{card}");
    }
    let part_1_result = part_1(&cards);
    println!("{part_1_result}");
}
