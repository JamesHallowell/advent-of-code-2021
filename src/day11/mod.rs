use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Octopus {
    Charging(u32),
    Flashed,
}

impl Octopus {
    fn has_flashed(self) -> bool {
        matches!(self, Self::Flashed)
    }

    fn increase_energy(self) -> Self {
        match self {
            Self::Charging(i) if i >= 9 => Self::Flashed,
            Self::Charging(i) => Self::Charging(i + 1),
            Self::Flashed => Self::Flashed,
        }
    }
}

#[derive(Debug)]
struct OctopusGrid(Vec<Octopus>);

impl FromStr for OctopusGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_digit(10).unwrap())
                .map(Octopus::Charging)
                .collect(),
        ))
    }
}

impl OctopusGrid {
    fn left(&self, index: usize) -> Option<usize> {
        (index % 10 != 0).then(|| index - 1)
    }

    fn right(&self, index: usize) -> Option<usize> {
        ((index + 1) % 10 != 0).then(|| index + 1)
    }

    fn up(&self, index: usize) -> Option<usize> {
        (index > 9).then(|| index - 10)
    }

    fn down(&self, index: usize) -> Option<usize> {
        (index < 90).then(|| index + 10)
    }

    fn adjacent(&self, index: usize) -> impl Iterator<Item = usize> {
        vec![
            self.up(index),
            self.down(index),
            self.left(index),
            self.right(index),
            self.left(index).and_then(|left| self.up(left)),
            self.left(index).and_then(|left| self.down(left)),
            self.right(index).and_then(|right| self.up(right)),
            self.right(index).and_then(|right| self.down(right)),
        ]
        .into_iter()
        .flatten()
    }

    fn increase_energy(&mut self, index: usize) {
        let octopus = self.0.get_mut(index).expect("bad index");
        if !octopus.has_flashed() {
            *octopus = octopus.increase_energy();
            if octopus.has_flashed() {
                for adjacent in self.adjacent(index) {
                    self.increase_energy(adjacent);
                }
            }
        }
    }

    fn count_flashed_octopi(&self) -> usize {
        self.0
            .iter()
            .filter(|octopus| octopus.has_flashed())
            .count()
    }

    fn reset_flashed_octopi(&mut self) {
        for octopus in &mut self.0 {
            if octopus.has_flashed() {
                *octopus = Octopus::Charging(0);
            }
        }
    }

    fn step(&mut self) -> usize {
        for i in 0..self.0.len() {
            self.increase_energy(i);
        }
        let flashes = self.count_flashed_octopi();
        self.reset_flashed_octopi();
        flashes
    }
}

#[test]
fn part_one() {
    let mut grid = include_str!("input.txt").parse::<OctopusGrid>().unwrap();
    let answer = (0..100)
        .into_iter()
        .fold(0, |flashes, _| flashes + grid.step());

    assert_eq!(answer, 1721);
}

#[test]
fn part_two() {
    let mut grid = include_str!("input.txt").parse::<OctopusGrid>().unwrap();
    let all_flash_at_step = (1..).into_iter().find(|_| grid.step() == 100).unwrap();

    assert_eq!(all_flash_at_step, 298);
}
