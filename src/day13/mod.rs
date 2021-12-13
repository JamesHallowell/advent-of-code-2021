use {
    parse_display::{Display, FromStr},
    std::collections::HashSet,
};

#[derive(Debug, Copy, Clone, Display, FromStr, Eq, PartialEq, Hash)]
#[display("{x},{y}")]
struct Dot {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Display, FromStr)]
#[display("fold along {xy}={line}")]
struct Fold {
    xy: char,
    line: i32,
}

fn parse_dots(input: &'static str) -> impl Iterator<Item = Dot> {
    input.lines().filter_map(|line| line.parse().ok())
}

fn parse_folds(input: &'static str) -> impl Iterator<Item = Fold> {
    input.lines().filter_map(|line| line.parse().ok())
}

fn apply_fold(dots: impl Iterator<Item = Dot>, fold: Fold) -> impl Iterator<Item = Dot> {
    dots.map(move |dot| match fold {
        Fold { xy: 'x', line } => {
            if dot.x < line {
                dot
            } else {
                let distance_from_fold = dot.x - line;
                Dot {
                    x: line - distance_from_fold,
                    y: dot.y,
                }
            }
        }
        Fold { xy: 'y', line } => {
            if dot.y < line {
                dot
            } else {
                let distance_from_fold = dot.y - line;
                Dot {
                    x: dot.x,
                    y: line - distance_from_fold,
                }
            }
        }
        _ => {
            panic!("unexpected fold direction");
        }
    })
}

#[test]
fn part_one() {
    let dots = parse_dots(include_str!("input.txt"));
    let mut folds = parse_folds(include_str!("input.txt"));

    assert_eq!(
        apply_fold(dots, folds.next().unwrap())
            .collect::<HashSet<_>>()
            .len(),
        755
    );
}

#[test]
fn part_two() {
    let dots = parse_dots(include_str!("input.txt"));
    let folds = parse_folds(include_str!("input.txt"));

    let dots = folds.fold(dots.collect::<HashSet<_>>(), |dots, fold| {
        apply_fold(dots.into_iter(), fold).collect()
    });

    for dot in dots {
        println!("({},{})", dot.x, dot.y); // go and plot them somewhere!
    }
}
