use itertools::Itertools;

fn count_increases(input: impl Iterator<Item = i32>) -> i32 {
    input
        .tuple_windows()
        .fold(0, |acc, (a, b)| acc + (a < b) as i32)
}

fn count_increases_for_sliding_windows(input: impl Iterator<Item = i32>) -> i32 {
    count_increases(input.tuple_windows().map(|(a, b, c)| a + b + c))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    fn parse_input() -> impl Iterator<Item = i32> {
        include_str!("input.txt")
            .lines()
            .map(|line| line.parse().unwrap())
    }

    #[test]
    fn example_one() {
        assert_eq!(count_increases(EXAMPLE.into_iter()), 7);
    }

    #[test]
    fn solve_one() {
        assert_eq!(count_increases(parse_input()), 1184);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_increases_for_sliding_windows(EXAMPLE.into_iter()), 5);
    }

    #[test]
    fn solve_two() {
        assert_eq!(count_increases_for_sliding_windows(parse_input()), 1158);
    }
}
