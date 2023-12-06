fn main() {
    print!(
        "{:?}",
        extract_numbers(
            "467..114..
  ...*......
  ..35..633.
  ......#...
  617*......
  .....+.58.
  ..592.....
  ......755.
  ...$.*....
  .664.598.."
        )
    );
}

fn extract_numbers(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let lines: Vec<_> = input.lines().collect();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (i, line) in lines.iter().enumerate() {
        let mut number = String::new();
        for (j, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                number.push(ch);
                let mut is_surrounded_by_points = true;
                for (dx, dy) in &directions {
                    let ni = i as i32 + dx;
                    let nj = j as i32 + dy;
                    if ni >= 0 && ni < lines.len() as i32 && nj >= 0 && nj < line.len() as i32 {
                        let surrounding_ch = lines[ni as usize].chars().nth(nj as usize).unwrap();
                        if surrounding_ch != '.' {
                            is_surrounded_by_points = false;
                            break;
                        }
                    }
                }
                if !is_surrounded_by_points && !number.is_empty() {
                    result.push(number.clone());
                    number.clear();
                }
            } else if !number.is_empty() {
                result.push(number.clone());
                number.clear();
            }
        }
        if !number.is_empty() {
            result.push(number.clone());
            number.clear();
        }
    }

    result
}
