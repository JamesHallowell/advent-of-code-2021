fn fuel_required_to_move_at_fixed_cost(start: i32, stop: i32) -> i32 {
    (start - stop).abs()
}

fn fuel_required_to_move_at_increasing_cost(start: i32, stop: i32) -> i32 {
    let n = fuel_required_to_move_at_fixed_cost(start, stop);
    (n * (n + 1)) / 2
}

fn crab_positions_range(
    crab_positions: impl Iterator<Item = i32> + Clone,
) -> impl Iterator<Item = i32> {
    let (min, max) = (
        crab_positions.clone().min().unwrap(),
        crab_positions.max().unwrap(),
    );
    min..=max
}

fn amount_of_fuel_required_to_move_crabs_to_position(
    crabs: impl Iterator<Item = i32>,
    destination: i32,
    fuel_calculation: fn(i32, i32) -> i32,
) -> i32 {
    crabs
        .map(|crab_position| fuel_calculation(crab_position, destination))
        .sum()
}

fn min_amount_of_fuel_to_align_the_crabs(fuel_calculation: fn(i32, i32) -> i32) -> i32 {
    let crab_positions = get_crab_positions();
    crab_positions_range(crab_positions.clone())
        .map(|position| {
            amount_of_fuel_required_to_move_crabs_to_position(
                crab_positions.clone(),
                position,
                fuel_calculation,
            )
        })
        .min()
        .unwrap()
}

fn get_crab_positions() -> impl Iterator<Item = i32> + Clone {
    include_str!("input.txt")
        .split(',')
        .map(|position| position.parse().unwrap())
}

#[test]
fn part_one() {
    assert_eq!(
        min_amount_of_fuel_to_align_the_crabs(fuel_required_to_move_at_fixed_cost),
        344735
    );
}

#[test]
fn part_two() {
    assert_eq!(
        min_amount_of_fuel_to_align_the_crabs(fuel_required_to_move_at_increasing_cost),
        96798233
    );
}
