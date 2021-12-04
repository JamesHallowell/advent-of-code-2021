use std::{cmp::Ordering, collections::HashSet};

fn parse_input(input: &'static str) -> impl Iterator<Item = Bitstring> + Clone {
    input.lines().map(|line| Bitstring {
        bits: u64::from_str_radix(line, 2).unwrap(),
        length: line.len(),
    })
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Bitstring {
    bits: u64,
    length: usize,
}

impl Bitstring {
    fn with_length(length: usize) -> Self {
        Self { bits: 0, length }
    }

    fn is_bit_set(&self, bit: usize) -> bool {
        (self.bits & (1 << bit)) != 0
    }

    fn with_bit_set(self, bit: usize) -> Self {
        Bitstring {
            bits: self.bits | (1 << bit),
            ..self
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Occurrences {
    bit_n: usize,
    zeros: usize,
    ones: usize,
}

impl Occurrences {
    fn add_a_one(self) -> Self {
        Occurrences {
            ones: self.ones + 1,
            ..self
        }
    }

    fn add_a_zero(self) -> Self {
        Occurrences {
            zeros: self.zeros + 1,
            ..self
        }
    }

    fn compare_ones_to_zeros(&self) -> Ordering {
        self.ones.cmp(&self.zeros)
    }
}

fn count_occurrences(input: impl Iterator<Item = Bitstring>, n: usize) -> Occurrences {
    input.fold(
        Occurrences {
            bit_n: n,
            zeros: 0,
            ones: 0,
        },
        |occurrences, item| match item.is_bit_set(n) {
            true => occurrences.add_a_one(),
            false => occurrences.add_a_zero(),
        },
    )
}

fn calculate_rate(
    input: impl Iterator<Item = Bitstring> + Clone,
    n: usize,
    should_set_bit: impl Fn(Occurrences) -> bool,
) -> Bitstring {
    (0..n).fold(Bitstring::with_length(n), move |bitstring, i| {
        let occurrences = count_occurrences(input.clone(), i);
        if should_set_bit(occurrences) {
            bitstring.with_bit_set(i)
        } else {
            bitstring
        }
    })
}

fn calculate_gamma_rate(input: impl Iterator<Item = Bitstring> + Clone) -> Bitstring {
    calculate_rate(
        input.clone(),
        input.clone().next().unwrap().length,
        |occurrences| {
            matches!(
                occurrences.compare_ones_to_zeros(),
                Ordering::Greater | Ordering::Equal
            )
        },
    )
}

fn calculate_epsilon_rate(input: impl Iterator<Item = Bitstring> + Clone) -> Bitstring {
    calculate_rate(
        input.clone(),
        input.clone().next().unwrap().length,
        |occurrences| {
            matches!(
                occurrences.compare_ones_to_zeros(),
                Ordering::Less | Ordering::Equal
            )
        },
    )
}

fn calculate_power_consumption(input: impl Iterator<Item = Bitstring> + Clone) -> u64 {
    calculate_gamma_rate(input.clone()).bits * calculate_epsilon_rate(input).bits
}

fn calculate_rating(
    input: impl Iterator<Item = Bitstring> + Clone,
    n: usize,
    should_retain_value: impl Fn(Occurrences, Bitstring) -> bool,
) -> Bitstring {
    let mut input: HashSet<_> = input.collect();
    for n in (0..n).rev() {
        let occurrences = count_occurrences(input.iter().cloned(), n);
        input.retain(|&value| should_retain_value(occurrences, value));
        if input.len() == 1 {
            break;
        }
    }
    let result = input.drain().next().unwrap();
    result
}

fn calculate_oxygen_generator_rating(input: impl Iterator<Item = Bitstring> + Clone) -> Bitstring {
    calculate_rating(
        input.clone(),
        input.clone().next().unwrap().length,
        |occurrences, value| match occurrences.compare_ones_to_zeros() {
            Ordering::Less => !value.is_bit_set(occurrences.bit_n),
            Ordering::Equal | Ordering::Greater => value.is_bit_set(occurrences.bit_n),
        },
    )
}

fn calculate_co2_scrubber_rating(input: impl Iterator<Item = Bitstring> + Clone) -> Bitstring {
    calculate_rating(
        input.clone(),
        input.clone().next().unwrap().length,
        |occurrences, value| match occurrences.compare_ones_to_zeros() {
            Ordering::Less => value.is_bit_set(occurrences.bit_n),
            Ordering::Equal | Ordering::Greater => !value.is_bit_set(occurrences.bit_n),
        },
    )
}

fn calculate_life_support_rating(input: impl Iterator<Item = Bitstring> + Clone) -> u64 {
    calculate_oxygen_generator_rating(input.clone()).bits
        * calculate_co2_scrubber_rating(input).bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_count_occurrences() {
        assert_eq!(
            count_occurrences(parse_input("1\n1\n1\n1").into_iter(), 0),
            Occurrences {
                zeros: 0,
                ones: 4,
                bit_n: 0
            },
        );
        assert_eq!(
            count_occurrences(parse_input("0\n0\n0\n0").into_iter(), 0),
            Occurrences {
                zeros: 4,
                ones: 0,
                bit_n: 0
            },
        );
        assert_eq!(
            count_occurrences(parse_input("0\n1\n0\n0").into_iter(), 0),
            Occurrences {
                zeros: 3,
                ones: 1,
                bit_n: 0
            },
        );
        assert_eq!(
            count_occurrences(parse_input("10\n01\n00\n00").into_iter(), 1),
            Occurrences {
                zeros: 3,
                ones: 1,
                bit_n: 1
            },
        );
        assert_eq!(
            count_occurrences(parse_input("1000\n0100\n1010\n1001").into_iter(), 3),
            Occurrences {
                zeros: 1,
                ones: 3,
                bit_n: 3
            },
        );
    }

    #[test]
    fn gamma_rate_example() {
        assert_eq!(
            calculate_gamma_rate(parse_input(include_str!("example.txt"))).bits,
            22
        );
    }

    #[test]
    fn epsilon_rate_example() {
        assert_eq!(
            calculate_epsilon_rate(parse_input(include_str!("example.txt"))).bits,
            9
        );
    }

    #[test]
    fn example_one() {
        assert_eq!(
            calculate_power_consumption(parse_input(include_str!("example.txt"))),
            198
        );
    }

    #[test]
    fn solve_one() {
        assert_eq!(
            calculate_power_consumption(parse_input(include_str!("input.txt"))),
            2003336
        );
    }

    #[test]
    fn oxygen_generator_rating_example() {
        assert_eq!(
            calculate_oxygen_generator_rating(parse_input(include_str!("example.txt"))).bits,
            23
        );
    }

    #[test]
    fn co2_scrubber_rating_example() {
        assert_eq!(
            calculate_co2_scrubber_rating(parse_input(include_str!("example.txt"))).bits,
            10
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            calculate_life_support_rating(parse_input(include_str!("example.txt"))),
            230
        );
    }

    #[test]
    fn solve_two() {
        assert_eq!(
            calculate_life_support_rating(parse_input(include_str!("input.txt"))),
            1877139
        );
    }
}
