use std::fs::read_to_string;

fn count_matching(card: &String) -> i32 {
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

    return count;
}

fn part_1(cards: &Vec<String>) -> i32 {
    let mut result: i32 = 0;

    for card in cards {
        let count: i32 = count_matching(&card);
        if count != 0 {
            result += i32::pow(2, (count - 1).try_into().unwrap());
        } else if count == 1 {
            result += 1;
        }
    }
    return result;
}

fn part_2(cards: &Vec<String>) -> usize {
    let mut result: usize = cards.len();
    let mut copies: Vec<usize> = Vec::new();
    for (i, card) in cards.iter().enumerate() {
        let count: usize = count_matching(card) as usize;
        for won_card_index in (i + 1)..(i + count + 1) {
            copies.push(won_card_index);
        }
    }
    let mut i = 0;
    while i < copies.len() {
        let copy = copies[i];
        let count = count_matching(&cards[copy]) as usize;
        for index in (copy + 1)..(copy + count + 1) {
            copies.push(index);
        }
        i += 1;
    }
    result += copies.len();
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
        // println!("{card}");
    }
    let part_1_result = part_1(&cards);
    println!("Part 1 results: {part_1_result}");
    let part_2_result = part_2(&cards);
    println!("Part 2 results: {part_2_result}");
}
