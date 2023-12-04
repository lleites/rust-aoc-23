use std::fs::read_to_string;
use std::io::{self};

#[derive(PartialEq, Debug)]
struct Throw {
    red: i32,
    blue: i32,
    green: i32,
}

fn parse_throw(line: &str) -> Throw {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    for color in line.split(",") {
        let mut parts = color.trim().split(" ");
        let digit = parts.next().unwrap();
        let col = parts.next().unwrap();
        match col {
            "red" => red = digit.parse().unwrap(),
            "blue" => blue = digit.parse().unwrap(),
            "green" => green = digit.parse().unwrap(),
            _ => (),
        }
    }

    Throw { red, blue, green }
}

fn parse_line(line: &str) -> (i32, Vec<Throw>) {
    let mut parts = line.split(":");
    let id_part = parts.next().unwrap();
    let throws_part = parts.next().unwrap();
    let throws = throws_part.split(";").map(parse_throw).collect();
    (id_part.split(" ").nth(1).unwrap().parse().unwrap(), throws)
}

fn process_file() -> io::Result<i32> {
    let file_path = "src/day_02/input.txt";
    let file = read_to_string(file_path)?;

    let max_game = Throw {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut sum = 0;
    for line in file.lines() {
        let (id, throws) = parse_line(line);
        let is_any_throw_bigger = throws.iter().any(|throw| {
            throw.red > max_game.red || throw.blue > max_game.blue || throw.green > max_game.green
        });
        if is_any_throw_bigger {
            sum += id;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_throw() {
        let result: Throw = parse_throw("1 red, 2 green, 6 blue");
        let expected = Throw {
            red: 1,
            blue: 6,
            green: 2,
        };
        assert_eq!(result, expected);

        let result: Throw = parse_throw("3 green, 10 blue");
        let expected = Throw {
            red: 0,
            blue: 10,
            green: 3,
        };
        assert_eq!(result, expected);

        let result: Throw = parse_throw("2 green");
        let expected = Throw {
            red: 0,
            blue: 0,
            green: 2,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line() {
        let result = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let expected_throws: Vec<Throw> = Vec::from([
            Throw {
                red: 4,
                blue: 3,
                green: 0,
            },
            Throw {
                red: 1,
                blue: 6,
                green: 2,
            },
            Throw {
                red: 0,
                blue: 0,
                green: 2,
            },
        ]);
        assert_eq!(result, (1, expected_throws));
    }

    #[test]
    fn test_process_file() {
        assert_eq!(process_file().unwrap(), 7);
    }
}

fn main() {
    print!("{}", process_file().unwrap())
}
