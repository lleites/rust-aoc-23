use std::{fs::read_to_string, io};

fn parse_card(line: &str) -> f64 {
    let numbers: Vec<&str> = line.split(":").last().unwrap().split("|").collect();
    let winning_numbers = numbers.first().unwrap();
    let chances = numbers.last().unwrap();
    let winning_numbers: Vec<i32> = winning_numbers.split(" ").flat_map(|x| x.parse()).collect();
    let chances: Vec<i32> = chances.split(" ").flat_map(|x| x.parse()).collect();

    let mut total = 0.5;

    for chance in chances {
        if winning_numbers.contains(&chance) {
            total = total * 2.0;
        }
    }

    if total == 0.5 {
        total = 0.0;
    }

    total
}

fn process_file() -> io::Result<f64> {
    let file_path = "src/day_04/input.txt";
    let file = read_to_string(file_path)?;
    let mut result: f64 = 0.0;
    for line in file.lines() {
        result += parse_card(line);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let result = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(result, 8.0);
    }
    #[test]

    fn test_process_file() {
        let result = process_file().unwrap();

        assert_eq!(result, 13.0);
    }
}

fn main() {
    println!("{}", process_file().unwrap())
}
