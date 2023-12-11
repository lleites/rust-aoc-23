use std::collections::HashMap;

fn count_steps(instructions: Vec<char>, map: HashMap<&str, (&str, &str)>) -> i32 {
    let mut instructions = instructions.iter().cycle();
    let mut number_steps = 1;
    let mut current = match instructions.next().unwrap() {
        'L' => map.get("AAA").unwrap().0,
        'R' => map.get("AAA").unwrap().1,
        _ => " ",
    };
    if current == "ZZZ" {
        return number_steps;
    }
    for ins in instructions {
        number_steps += 1;
        current = match ins {
            'L' => map.get(current).unwrap().0,
            'R' => map.get(current).unwrap().1,
            _ => " ",
        };
        if current == "ZZZ" {
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

        let instructions = vec!['L', 'L', 'R'];

        let result = count_steps(instructions, map);

        assert_eq!(result, 6);
    }
}
