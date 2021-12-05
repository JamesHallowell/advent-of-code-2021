use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::{iter::successors, ops::Add};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display, FromStr)]
#[display("{x},{y}")]
struct Point {
    x: i32,
    y: i32,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, Display, FromStr)]
#[display("{start} -> {stop}")]
struct Line {
    start: Point,
    stop: Point,
}

enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Line {
    fn direction(&self) -> Direction {
        match (self.start, self.stop) {
            (start, stop) if start.x == stop.x => Direction::Vertical,
            (start, stop) if start.y == stop.y => Direction::Horizontal,
            _ => Direction::Diagonal,
        }
    }

    pub fn delta(&self) -> Point {
        Point {
            x: match self.direction() {
                Direction::Horizontal | Direction::Diagonal => {
                    if self.start.x < self.stop.x {
                        1
                    } else {
                        -1
                    }
                }
                Direction::Vertical => 0,
            },
            y: match self.direction() {
                Direction::Vertical | Direction::Diagonal => {
                    if self.start.y < self.stop.y {
                        1
                    } else {
                        -1
                    }
                }
                Direction::Horizontal => 0,
            },
        }
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        let line = *self;
        successors(Some(self.start), move |&last| {
            if line.stop == last {
                None
            } else {
                Some(last + line.delta())
            }
        })
    }
}

fn parse_input(input: &'static str) -> impl Iterator<Item = Line> {
    input.lines().map(|line| line.parse().unwrap())
}

#[test]
fn solve_one() {
    assert_eq!(
        parse_input(include_str!("input.txt"))
            .filter(|line| matches!(
                line.direction(),
                Direction::Horizontal | Direction::Vertical
            ))
            .map(|line| line.points())
            .flatten()
            .counts()
            .iter()
            .filter(|(_, &count)| count >= 2)
            .count(),
        6397
    );
}

#[test]
fn solve_two() {
    assert_eq!(
        parse_input(include_str!("input.txt"))
            .map(|line| line.points())
            .flatten()
            .counts()
            .iter()
            .filter(|(_, &count)| count >= 2)
            .count(),
        22335
    );
}
