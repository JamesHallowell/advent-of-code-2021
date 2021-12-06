use std::collections::VecDeque;

struct Population(VecDeque<u64>);

fn parse_input(input: &'static str) -> Population {
    Population(
        input
            .split(',')
            .map(|timer| timer.parse::<u64>().unwrap())
            .fold(VecDeque::from([0; 9]), |mut population, time| {
                population[time as usize] += 1;
                population
            }),
    )
}

impl Population {
    fn tick(&mut self) {
        self.0.rotate_left(1);
        self.0[6] += self.0[8];
    }

    fn size(&self) -> u64 {
        self.0.iter().sum()
    }
}

#[test]
fn solve_one() {
    let mut population = parse_input(include_str!("input.txt"));
    for _ in 0..80 {
        population.tick();
    }
    assert_eq!(population.size(), 362639);
}

#[test]
fn solve_two() {
    let mut population = parse_input(include_str!("input.txt"));
    for _ in 0..256 {
        population.tick();
    }
    assert_eq!(population.size(), 1639854996917);
}
