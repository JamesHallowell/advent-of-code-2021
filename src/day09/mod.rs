use {
    itertools::Itertools,
    std::{collections::HashSet, str::FromStr},
};

#[derive(Debug)]
struct HeightMap {
    heights: Vec<u32>,
    row_length: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HeightMap {
            heights: s.lines().fold(vec![], |mut height_map, heights| {
                let mut row = heights
                    .chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect::<Vec<_>>();
                height_map.append(&mut row);
                height_map
            }),
            row_length: s.lines().next().unwrap().len(),
        })
    }
}

impl HeightMap {
    fn get_height_at(&self, coordinate: Coordinate) -> u32 {
        self.get_index_for_coordinate(coordinate)
            .and_then(|index| self.heights.get(index).cloned())
            .expect("not a valid coordinate")
    }

    fn get_coordinate_for_index(&self, index: usize) -> Option<Coordinate> {
        (index < self.heights.len()).then(|| Coordinate {
            x: (index % self.row_length) as i32,
            y: (index / self.row_length) as i32,
        })
    }

    fn coordinate_in_bounds(&self, coordinate: Coordinate) -> bool {
        coordinate.x >= 0
            && coordinate.x < self.row_length as i32
            && coordinate.y >= 0
            && coordinate.y < self.row_length as i32
    }

    fn get_index_for_coordinate(&self, coordinate: Coordinate) -> Option<usize> {
        if !self.coordinate_in_bounds(coordinate) {
            return None;
        }

        let index = (coordinate.y * self.row_length as i32) + coordinate.x;
        (index >= 0 && index < self.heights.len() as i32).then(|| index as usize)
    }

    fn coordinates(&self) -> impl Iterator<Item = Coordinate> + '_ {
        (0..self.heights.len()).map(|index| self.get_coordinate_for_index(index).unwrap())
    }

    pub fn adjacent_coordinates(
        &self,
        coordinate: Coordinate,
    ) -> impl Iterator<Item = Coordinate> + '_ {
        [(0, -1), (0, 1), (-1, 0), (1, 0)]
            .iter()
            .map(move |(x, y)| Coordinate {
                x: coordinate.x + x,
                y: coordinate.y + y,
            })
            .filter(|&adjacent| self.get_index_for_coordinate(adjacent).is_some())
    }

    fn is_low_point(&self, coordinate: Coordinate) -> bool {
        let height_at_coordinate = self.get_height_at(coordinate);

        self.adjacent_coordinates(coordinate)
            .map(|adjacent| self.get_height_at(adjacent))
            .all(|adjacent_height| height_at_coordinate < adjacent_height)
    }

    fn low_points(&self) -> impl Iterator<Item = Coordinate> + '_ {
        self.coordinates()
            .filter(|&coordinate| self.is_low_point(coordinate))
    }

    fn basin(&self, coordinate: Coordinate) -> HashSet<Coordinate> {
        let mut init = HashSet::new();
        if self.is_low_point(coordinate) {
            init.insert(coordinate);
        }

        self.adjacent_upward_flows(coordinate)
            .fold(init, |mut acc, coordinate| {
                acc.insert(coordinate);
                acc.extend(self.basin(coordinate));
                acc
            })
    }

    fn adjacent_upward_flows(
        &self,
        coordinate: Coordinate,
    ) -> impl Iterator<Item = Coordinate> + '_ {
        let height_at_coordinate = self.get_height_at(coordinate);

        self.adjacent_coordinates(coordinate)
            .map(|adjacent_coordinate| {
                (adjacent_coordinate, self.get_height_at(adjacent_coordinate))
            })
            .filter(|&(_, height_at_adjacent_coordinate)| height_at_adjacent_coordinate < 9)
            .filter(move |&(_, height_at_adjacent_coordinate)| {
                height_at_adjacent_coordinate > height_at_coordinate
            })
            .map(|(coordinate, _)| coordinate)
    }
}

#[test]
fn part_one() {
    let height_map = include_str!("input.txt").parse::<HeightMap>().unwrap();
    let risk_level = |coordinate| height_map.get_height_at(coordinate) + 1;

    let sum_of_low_point_risk_levels = height_map.low_points().map(risk_level).sum::<u32>();
    assert_eq!(sum_of_low_point_risk_levels, 600);
}

#[test]
fn part_two() {
    let height_map = include_str!("input.txt").parse::<HeightMap>().unwrap();

    let product_of_three_largest_basins = height_map
        .low_points()
        .map(|low_point| height_map.basin(low_point).len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>();

    assert_eq!(product_of_three_largest_basins, 987840);
}
