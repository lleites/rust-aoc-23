use std::collections::HashMap;
use std::str::FromStr;

const FIRST: &str = "AAA";
const LAST: &str = "ZZZ";

enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn count_steps(instructions: Vec<Direction>, map: HashMap<&str, (&str, &str)>) -> i32 {
    let instructions = instructions.iter().cycle();
    let mut number_steps = 0;
    let mut current = FIRST;
    for ins in instructions {
        number_steps += 1;
        current = match ins {
            Direction::Left => map.get(current).unwrap().0,
            Direction::Right => map.get(current).unwrap().1,
        };
        if current == LAST {
            return number_steps;
        }
    }
    0
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_count_steps() {
        let mut map = HashMap::new();
        map.insert("AAA", ("BBB", "BBB"));
        map.insert("BBB", ("AAA", "ZZZ"));
        map.insert("ZZZ", ("ZZZ", "ZZZ"));

        let instructions = vec!['L', 'L', 'R']
            .iter()
            .flat_map(|char| Direction::from_str(char.to_string().as_ref()))
            .collect();

        let result = count_steps(instructions, map);

        assert_eq!(result, 6);
    }
}
