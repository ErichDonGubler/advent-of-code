use insta::assert_debug_snapshot;
use itertools::Itertools;

const EXAMPLE_PART_1: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

#[derive(Debug)]
struct Races {
    time: Vec<u32>,
    distance: Vec<u32>,
}

impl Races {
    fn new(input: &str) -> Self {
        let (time, distance) = input.lines().collect_tuple().unwrap();
        let time = time
            .strip_prefix("Time:")
            .unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<_>>();
        let distance = distance
            .strip_prefix("Distance:")
            .unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(time.len(), distance.len());
        Self { time, distance }
    }
}

#[test]
fn part_1_example() {
    assert_debug_snapshot!(Races::new(EXAMPLE_PART_1));
}
