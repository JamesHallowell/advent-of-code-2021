use {
    itertools::Itertools,
    std::{
        collections::{HashMap, HashSet},
        str::FromStr,
    },
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            s if s.chars().all(|c| c.is_uppercase()) => Ok(Cave::Big(s.to_string())),
            s if s.chars().all(|c| c.is_lowercase()) => Ok(Cave::Small(s.to_string())),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Connections(HashMap<Cave, HashSet<Cave>>);

impl FromStr for Connections {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().fold(
            HashMap::new(),
            |mut connections, line| {
                let (a, b) = line.split_once('-').unwrap();
                let (a, b) = (Cave::from_str(a).unwrap(), Cave::from_str(b).unwrap());

                connections
                    .entry(a.clone())
                    .and_modify(|set| {
                        set.insert(b.clone());
                    })
                    .or_insert_with(|| {
                        let mut set = HashSet::new();
                        set.insert(b.clone());
                        set
                    });

                connections
                    .entry(b.clone())
                    .and_modify(|set| {
                        set.insert(a.clone());
                    })
                    .or_insert_with(|| {
                        let mut set = HashSet::new();
                        set.insert(a);
                        set
                    });

                connections
            },
        )))
    }
}

type Path = Vec<Cave>;

fn part_one_rules(cave: &Cave, path: &Path) -> bool {
    match cave {
        Cave::Start | Cave::Small(_) if path.contains(cave) => false,
        _ => true,
    }
}

fn part_two_rules(cave: &Cave, path: &Path) -> bool {
    match cave {
        Cave::Start => false,
        Cave::Small(_) => {
            if path.contains(cave) {
                path.iter()
                    .filter_map(|cave| match cave {
                        Cave::Small(s) => Some(s),
                        _ => None,
                    })
                    .counts()
                    .values()
                    .all(|&count| count < 2)
            } else {
                true
            }
        }
        _ => true,
    }
}

fn paths_to_end(
    connections: &Connections,
    path: Path,
    rules: fn(&Cave, &Path) -> bool,
) -> Vec<Path> {
    let current_cave = path
        .last()
        .expect("we must have some position in the cave system");

    if matches!(current_cave, Cave::End) {
        return vec![path];
    }

    if let Some(connected_caves) = connections.0.get(current_cave) {
        connected_caves
            .iter()
            .filter(|cave| (rules)(cave, &path))
            .fold(vec![], |mut paths, cave| {
                let mut path = path.clone();
                path.push(cave.clone());
                paths.extend(paths_to_end(connections, path, rules));
                paths
            })
    } else {
        vec![path]
    }
}

#[test]
fn part_one() {
    let connections = include_str!("input.txt").parse::<Connections>().unwrap();
    let paths = paths_to_end(&connections, vec![Cave::Start], part_one_rules);

    assert_eq!(paths.len(), 5920);
}

#[test]
fn part_two() {
    let connections = include_str!("input.txt").parse::<Connections>().unwrap();
    let paths = paths_to_end(&connections, vec![Cave::Start], part_two_rules);

    assert_eq!(paths.len(), 155477);
}
