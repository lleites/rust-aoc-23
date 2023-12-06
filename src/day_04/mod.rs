use crate::utils::read_lines;
use std::io;

fn parse_card_func(line: &str) -> i32 {
    let number_parts: Vec<&str> = line.split(":").last().unwrap().split("|").collect();
    let winning_numbers = number_parts.first().unwrap();
    let chances = number_parts.last().unwrap();

    let winning_numbers: Vec<i32> = winning_numbers
        .split_whitespace()
        .flat_map(str::parse)
        .collect();
    let chances: Vec<i32> = chances.split_whitespace().flat_map(str::parse).collect();

    let wins = chances.iter().flat_map(|chance| {
        if winning_numbers.contains(&chance) {
            Some(1)
        } else {
            None
        }
    });

    wins.fold(0.5, |acc, _current| acc * 2.0) as i32
}

fn parse_card(line: &str) -> i32 {
    let number_parts: Vec<&str> = line.split(":").last().unwrap().split("|").collect();
    let winning_numbers = number_parts.first().unwrap();
    let chances = number_parts.last().unwrap();

    let winning_numbers: Vec<i32> = winning_numbers
        .split_whitespace()
        .flat_map(str::parse)
        .collect();
    let chances: Vec<i32> = chances.split_whitespace().flat_map(str::parse).collect();

    let wins = chances.iter().flat_map(|chance| {
        if winning_numbers.contains(&chance) {
            Some(1)
        } else {
            None
        }
    });

    wins.fold(0.5, |acc, _current| acc * 2.0) as i32
}

pub fn process_file() -> io::Result<i32> {
    let result: i32 = read_lines("src/day_04/input.txt")
        .iter()
        .map(|x| parse_card(&x))
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let result = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(result, 8);
    }

    #[test]
    fn test_parse_card_func() {
        let result = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(result, 8);
    }

    #[test]
    fn test_process_file() {
        let result = process_file().unwrap();

        assert_eq!(result, 13);
    }
}
