use std::{fs::read_to_string, io};

fn parse_card(line: &str) -> i32 {
    let number_parts: Vec<&str> = line.split(":").last().unwrap().split("|").collect();
    let winning_numbers = number_parts.first().unwrap();
    let chances = number_parts.last().unwrap();

    let winning_numbers: Vec<i32> = winning_numbers
        .split_whitespace()
        .flat_map(str::parse)
        .collect();
    let chances: Vec<i32> = chances.split_whitespace().flat_map(str::parse).collect();

    let mut total = 0;

    for chance in chances {
        if winning_numbers.contains(&chance) {
            if total == 0 {
                total = 1
            } else {
                total = total * 2;
            }
        }
    }

    total
}

fn read_lines(file_path: &str) -> Vec<String> {
    let file = read_to_string(file_path).unwrap();
    file.lines().map(str::to_string).collect()
}

fn process_file() -> io::Result<i32> {
    let mut result: i32 = 0;
    for line in read_lines("src/day_04/input.txt") {
        result += parse_card(&line);
    }

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

    fn test_process_file() {
        let result = process_file().unwrap();

        assert_eq!(result, 13);
    }
}

fn main() {
    println!("{}", process_file().unwrap())
}
