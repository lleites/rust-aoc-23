use std::collections::HashMap;

fn count_steps(instructions: Vec<char>, map: HashMap<String, (String, String)>) -> i32 {
    let mut instructions = instructions.iter().cycle();
    let mut number_steps = 1;
    let mut current = match instructions.next().unwrap() {
        'L' => map.get("AAA").unwrap().0.clone(),
        'R' => map.get("AAA").unwrap().1.clone(),
        _ => " ".to_string(),
    };
    if current == "ZZZ".to_string() {
        return number_steps;
    }
    for ins in instructions {
        number_steps += 1;
        current = match ins {
            'L' => map.get(&current).unwrap().0.clone(),
            'R' => map.get(&current).unwrap().1.clone(),
            _ => " ".to_string(),
        };
        if current == "ZZZ".to_string() {
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
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        map.insert("AAA".to_string(), ("BBB".to_string(), "BBB".to_string()));
        map.insert("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string()));
        map.insert("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string()));

        let instructions = vec!['L', 'L', 'R'];

        let result = count_steps(instructions, map);

        assert_eq!(result, 6);
    }
}
