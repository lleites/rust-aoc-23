use std::fs::read_to_string;

pub fn read_lines(file_path: &str) -> Vec<String> {
    let file = read_to_string(file_path).unwrap();
    file.lines().map(str::to_string).collect()
}
