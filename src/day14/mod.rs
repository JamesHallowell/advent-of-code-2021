use {itertools::Itertools, std::collections::HashMap};

type Pair = (char, char);
type Rules = HashMap<Pair, char>;
type Polymer = HashMap<Pair, usize>;

fn parse_polymer_template(input: &'static str) -> Polymer {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .tuple_windows::<(_, _)>()
        .fold(Polymer::new(), |mut polymer, pair| {
            polymer
                .entry(pair)
                .and_modify(|a_count| *a_count += 1)
                .or_insert_with(|| 1);
            polymer
        })
}

fn parse_pair_insertion_rules(input: &'static str) -> Rules {
    input.lines().skip(2).fold(HashMap::new(), |mut map, line| {
        let mut chars = line.chars();
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        let c = chars.skip(4).next().unwrap();

        map.insert((a, b), c);
        map
    })
}

fn apply_rules((a, b): (char, char), rules: &Rules) -> (Pair, Pair) {
    let insertion = rules
        .get(&(a, b))
        .expect("found pair without insertion rule");
    ((a, *insertion), (*insertion, b))
}

fn apply_pair_insertion(polymer: Polymer, rules: &Rules) -> Polymer {
    polymer
        .iter()
        .fold(Polymer::new(), |mut polymer, (&pair, &count)| {
            let (a, b) = apply_rules(pair, rules);
            polymer
                .entry(a)
                .and_modify(|a_count| *a_count += count)
                .or_insert_with(|| count);
            polymer
                .entry(b)
                .and_modify(|b_count| *b_count += count)
                .or_insert_with(|| count);
            polymer
        })
}

fn min_max_element_counts(polymer: &Polymer) -> (usize, usize) {
    let counts = polymer
        .iter()
        .fold(HashMap::new(), |mut counts, (&(a, b), &count)| {
            let count = if count % 2 != 0 {
                (count / 2) + 1
            } else {
                count / 2
            };

            counts
                .entry(a)
                .and_modify(|a_count| *a_count += count)
                .or_insert(count);
            counts
                .entry(b)
                .and_modify(|b_count| *b_count += count)
                .or_insert(count);
            counts
        });

    let most_common_quantity = counts
        .iter()
        .map(|(_, x)| x)
        .max_by(|a, b| a.cmp(b))
        .unwrap();

    let least_common_quantity = counts
        .iter()
        .map(|(_, x)| x)
        .min_by(|a, b| a.cmp(b))
        .unwrap();

    (*least_common_quantity, *most_common_quantity)
}

#[test]
fn part_one() {
    let template = parse_polymer_template(include_str!("input.txt"));
    let rules = parse_pair_insertion_rules(include_str!("input.txt"));

    let polymer = (0..10).fold(template, |polymer, _| apply_pair_insertion(polymer, &rules));

    let (least_common, most_common) = min_max_element_counts(&polymer);

    assert_eq!(most_common - least_common, 2915);
}

#[test]
fn part_two() {
    let template = parse_polymer_template(include_str!("input.txt"));
    let rules = parse_pair_insertion_rules(include_str!("input.txt"));

    let polymer = (0..40).fold(template, |polymer, _| apply_pair_insertion(polymer, &rules));

    let (least_common, most_common) = min_max_element_counts(&polymer);

    println!("{}", most_common - least_common);
}
