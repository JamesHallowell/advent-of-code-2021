use parse_display::{Display, FromStr};

#[derive(Display, FromStr)]
enum Command {
    #[display("up {0}")]
    Up(i32),

    #[display("down {0}")]
    Down(i32),

    #[display("forward {0}")]
    Forward(i32),
}

#[derive(Copy, Clone, Default)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn parse_input(input: &'static str) -> impl Iterator<Item = Command> {
    input.lines().map(|line| line.parse().unwrap())
}

fn solve(
    input: impl Iterator<Item = Command>,
    rules: impl Fn(Position, Command) -> Position,
) -> i32 {
    let position = input.fold(Position::default(), rules);
    position.horizontal * position.depth
}

fn part_1(position: Position, command: Command) -> Position {
    match command {
        Command::Up(up) => Position {
            depth: position.depth - up,
            ..position
        },
        Command::Down(down) => Position {
            depth: position.depth + down,
            ..position
        },
        Command::Forward(forward) => Position {
            horizontal: position.horizontal + forward,
            ..position
        },
    }
}

fn part_2(position: Position, command: Command) -> Position {
    match command {
        Command::Up(up) => Position {
            aim: position.aim - up,
            ..position
        },
        Command::Down(down) => Position {
            aim: position.aim + down,
            ..position
        },
        Command::Forward(forward) => Position {
            horizontal: position.horizontal + forward,
            depth: position.depth + (position.aim * forward),
            ..position
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(solve(parse_input(include_str!("example.txt")), part_1), 150);
    }

    #[test]
    fn solve_one() {
        assert_eq!(
            solve(parse_input(include_str!("input.txt")), part_1),
            1813801
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(solve(parse_input(include_str!("example.txt")), part_2), 900);
    }

    #[test]
    fn solve_two() {
        assert_eq!(
            solve(parse_input(include_str!("input.txt")), part_2),
            1960569556
        );
    }
}
