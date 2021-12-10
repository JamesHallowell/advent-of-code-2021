use itertools::Itertools;

fn input() -> impl Iterator<Item = &'static str> {
    include_str!("input.txt").lines()
}

fn find_first_illegal_char(input: &str) -> Option<char> {
    let mut stack = vec![];
    for char in input.chars() {
        match char {
            '(' | '[' | '{' | '<' => {
                stack.push(char);
            }
            ')' => {
                if !matches!(stack.pop(), Some('(')) {
                    return Some(char);
                }
            }
            ']' => {
                if !matches!(stack.pop(), Some('[')) {
                    return Some(char);
                }
            }
            '}' => {
                if !matches!(stack.pop(), Some('{')) {
                    return Some(char);
                }
            }
            '>' => {
                if !matches!(stack.pop(), Some('<')) {
                    return Some(char);
                }
            }
            _ => {}
        }
    }

    None
}

fn illegal_char_score(char: char) -> u32 {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn auto_complete(input: &str) -> String {
    let mut stack = vec![];
    for char in input.chars() {
        match char {
            '(' | '[' | '{' | '<' => {
                stack.push(char);
            }
            ')' | ']' | '}' | '>' => {
                stack.pop();
            }
            _ => {}
        }
    }
    let mut completion = String::new();
    while let Some(char) = stack.pop() {
        completion.push(match char {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("bad input"),
        });
    }
    completion
}

fn completion_score(char: char) -> u64 {
    match char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

#[test]
fn part_one() {
    let answer = input()
        .map(|line| find_first_illegal_char(line))
        .flatten()
        .map(illegal_char_score)
        .sum::<u32>();

    assert_eq!(answer, 296535);
}

#[test]
fn part_two() {
    let completion_scores = input()
        .filter_map(|line| find_first_illegal_char(line).is_none().then(|| line))
        .map(auto_complete)
        .map(|completion| {
            completion
                .chars()
                .fold(0, |acc, char| (acc * 5) + completion_score(char))
        })
        .sorted()
        .collect_vec();

    let answer = *completion_scores.get(completion_scores.len() / 2).unwrap();
    assert_eq!(answer, 4245130838);
}
