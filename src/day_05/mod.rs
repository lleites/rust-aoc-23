use crate::utils::read_lines;
use std::{collections::HashMap, io};

pub fn build_mapping(lines: Vec<String>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for i in 0..99 {
        map.insert(i, i);
    }
    for line in lines {
        let mut iter = line.split_whitespace();
        let destination: i32 = iter.next().unwrap().parse().unwrap();
        let source: i32 = iter.next().unwrap().parse().unwrap();
        let range: i32 = iter.next().unwrap().parse().unwrap();
        for i in 0..range {
            map.insert(source + i, destination + i);
        }
    }
    map
}

pub fn get_maps() -> Vec<HashMap<i32, i32>> {
    let file_content = read_lines("src/day_05/input.txt");
    let mut result = Vec::new();
    let first_map: HashMap<i32, i32> = file_content[0]
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| (x.parse().unwrap(), x.parse().unwrap()))
        .collect();
    result.push(first_map);

    let mut lines: Vec<String> = Vec::new();
    for line in &file_content[3..] {
        if line.ends_with("map:") && !lines.is_empty() {
            result.push(build_mapping(lines));
            lines = Vec::new();
        } else {
            if !line.is_empty() {
                lines.push(line.clone());
            }
        }
    }
    result.push(build_mapping(lines));

    result
}

pub fn find_location() -> i32 {
    let maps = get_maps();
    let mut folded_map: HashMap<i32, i32> = maps.first().unwrap().clone();
    for map in get_maps() {
        for (k, v) in folded_map.clone().iter() {
            folded_map.insert(k.clone(), map.get(v).unwrap().clone());
        }
    }
    folded_map.values().min().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_mapping() {
        let lines = vec!["50 98 2", "52 50 48"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let result = build_mapping(lines);
        let data = vec![
            (98, 50),
            (60, 62),
            (66, 68),
            (73, 75),
            (76, 78),
            (63, 65),
            (65, 67),
            (57, 59),
            (50, 52),
            (67, 69),
            (53, 55),
            (70, 72),
            (74, 76),
            (88, 90),
            (87, 89),
            (64, 66),
            (71, 73),
            (77, 79),
            (89, 91),
            (93, 95),
            (94, 96),
            (82, 84),
            (96, 98),
            (84, 86),
            (68, 70),
            (62, 64),
            (69, 71),
            (51, 53),
            (86, 88),
            (92, 94),
            (97, 99),
            (83, 85),
            (80, 82),
            (54, 56),
            (61, 63),
            (79, 81),
            (85, 87),
            (91, 93),
            (55, 57),
            (99, 51),
            (72, 74),
            (75, 77),
            (81, 83),
            (95, 97),
            (59, 61),
            (56, 58),
            (78, 80),
            (58, 60),
            (52, 54),
            (90, 92),
        ];

        let mut expected: HashMap<_, _> = data.into_iter().collect();
        for i in 0..99 {
            if !expected.contains_key(&i) {
                expected.insert(i, i);
            }
        }

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_maps() {
        let actual = get_maps();
        assert_eq!(actual.len(), 8);
    }

    #[test]
    fn test_find_location() {
        let actual = find_location();
        assert_eq!(actual, 35);
    }
}
