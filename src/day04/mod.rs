#[derive(Debug, Copy, Clone)]
enum Number {
    Unmarked(i32),
    Marked,
}

impl Number {
    const fn is_marked(self) -> bool {
        matches!(self, Self::Marked)
    }
}

struct Board([Number; 25]);

impl Board {
    const N: usize = 5;

    fn mark_number(&mut self, drawn_number: i32) {
        for number in &mut self.0 {
            if let &mut Number::Unmarked(n) = number {
                if n == drawn_number {
                    *number = Number::Marked;
                }
            }
        }
    }

    fn row(&self, row: usize) -> impl Iterator<Item = Number> + '_ {
        (row * Self::N..)
            .take(Self::N)
            .map(|i| self.0.get(i).cloned())
            .flatten()
    }

    fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = Number> + '_> + '_ {
        (0..Self::N).map(|i| self.row(i))
    }

    fn column(&self, column: usize) -> impl Iterator<Item = Number> + '_ {
        (column..)
            .step_by(Self::N)
            .take(Self::N)
            .map(|i| self.0.get(i).cloned())
            .flatten()
    }

    fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = Number> + '_> + '_ {
        (0..Self::N).map(|i| self.column(i))
    }

    fn any_winning_rows(&self) -> bool {
        self.rows().any(|mut row| row.all(Number::is_marked))
    }

    fn any_winning_columns(&self) -> bool {
        self.columns()
            .any(|mut column| column.all(Number::is_marked))
    }

    fn is_winner(&self) -> bool {
        self.any_winning_rows() || self.any_winning_columns()
    }

    fn sum_of_unmarked_numbers(&self) -> i32 {
        self.0
            .iter()
            .filter_map(|number| match number {
                &Number::Unmarked(n) => Some(n),
                &Number::Marked => None,
            })
            .fold(0, |sum, number| sum + number)
    }

    fn score(&self, winning_number: i32) -> i32 {
        self.sum_of_unmarked_numbers() * winning_number
    }
}

struct BingoScores(Vec<i32>);

impl BingoScores {
    fn winning_score(&self) -> i32 {
        *self.0.first().unwrap()
    }

    fn losing_score(&self) -> i32 {
        *self.0.last().unwrap()
    }
}

fn play_bingo(numbers: Vec<i32>, mut boards: Vec<Board>) -> BingoScores {
    let mut scores = BingoScores(vec![]);
    for number_drawn in numbers {
        for board in &mut boards {
            board.mark_number(number_drawn);
            if board.is_winner() {
                scores.0.push(board.score(number_drawn));
            }
        }

        boards.retain(|board| !board.is_winner())
    }
    scores
}

#[test]
fn solve() {
    let (numbers, boards) = parse_input(include_str!("input.txt"));

    let scores = play_bingo(numbers, boards);

    assert_eq!(scores.winning_score(), 10680);
    assert_eq!(scores.losing_score(), 31892);
}

fn parse_input(input: &'static str) -> (Vec<i32>, Vec<Board>) {
    use itertools::Itertools;

    let mut lines = input.lines();

    let numbers = lines
        .next()
        .expect("first line is draw order")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let mut boards = Vec::new();
    for chunk in &lines.chunks(6) {
        let board = chunk.skip(1).fold(Vec::new(), |mut board, line| {
            for x in line.trim().split_whitespace() {
                board.push(Number::Unmarked(x.parse().unwrap()));
            }
            board
        });
        boards.push(Board(board.try_into().expect("board size is 5x5")));
    }

    (numbers, boards)
}
