use std::fs::read_to_string;

fn part_1(games: &Vec<String>) -> usize {
    let mut result: usize = (games.len() * (games.len() + 1)) / 2;

    for (index, game) in games.iter().enumerate() {
        // println!("{game}");

        let cubes: Vec<&str> = game.split(", ").collect();
        // println!("{}", cubes.len());
        for i in cubes {
            if i.contains("red") {
                let amount: Vec<&str> = i.split(" ").collect();
                let red = amount[0].parse::<i32>().unwrap();
                if red > 12 {
                    result -= index + 1;
                    break;
                }
            } else if i.contains("green") {
                let amount: Vec<&str> = i.split(" ").collect();
                let green = amount[0].parse::<i32>().unwrap();
                if green > 13 {
                    result -= index + 1;
                    break;
                }
            } else if i.contains("blue") {
                let amount: Vec<&str> = i.split(" ").collect();
                let blue = amount[0].parse::<i32>().unwrap();
                if blue > 14 {
                    result -= index + 1;
                    break;
                }
            }
        }
    }

    return result;
}

fn main() {
    let data = read_to_string("input.txt").unwrap();
    let mut games: Vec<String> = data.split("\n").map(|line| line.to_string()).collect();

    for (index, game) in games.iter_mut().enumerate() {
        let prefix = format!("Game {}: ", index + 1);
        *game = game
            .strip_prefix(&prefix)
            .unwrap_or(game)
            .to_string()
            .replace(";", ",");
    }
    games.retain(|game| !game.is_empty());

    let part_1_result: usize = part_1(&games);
    println!("{part_1_result}")
}
