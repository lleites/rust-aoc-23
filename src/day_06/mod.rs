fn find_ways(time: i32, max_distance: i32) -> i32 {
    let mut solutions = 0;
    for seconds in 1..time {
        if (time - seconds) * seconds > max_distance {
            solutions += 1;
        }
    }
    solutions
}

fn find_every_way() -> i32 {
    let races = vec![(7, 9), (15, 40), (30, 200)];
    let ways = races.iter().map(|(t, m)| find_ways(*t, *m));
    ways.product()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_ways() {
        let result = find_ways(30, 200);

        assert_eq!(result, 9);
    }
    #[test]

    fn test_find_every_way() {
        let result = find_every_way();

        assert_eq!(result, 288);
    }
}
