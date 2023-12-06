use std::fs::read_to_string;
use std::io::{self};
use std::num::ParseIntError;

fn get_digits(line: &str) -> Option<String> {
    let result: String;

    let digits: Vec<char> = line.chars().filter(|ch| ch.is_digit(10)).collect();

    if digits.is_empty() {
        return None;
    }

    let first = digits.first().unwrap();
    if digits.len() > 1 {
        let last = digits.last().unwrap();
        result = format!("{}{}", first, last);
    } else {
        result = format!("{}{}", first, first);
    }

    Some(result)
}

fn process_line(line: &str) -> Result<i32, ParseIntError> {
    let digits = get_digits(line);
    let value = digits.unwrap().parse();
    value
}

fn process_file() -> io::Result<i32> {
    let file_path = "src/day_01/01_input.txt";
    let file = read_to_string(file_path)?;

    let sum = file.lines().flat_map(process_line).sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digits() {
        let result_3 = get_digits("pqr3st4u8vwx");
        let expected_3 = "38";
        assert_eq!(result_3.unwrap(), expected_3);

        let result_1 = get_digits("treb7uchet");
        let expected_1 = "77";
        assert_eq!(result_1.unwrap(), expected_1);
    }
    #[test]

    fn test_process_file() {
        // Test process_file
        let actual = process_file().unwrap();
        assert_eq!(actual, 142);
    }
}
