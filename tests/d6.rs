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

    fn winning_races_product(&self) -> usize {
        let Self { time, distance } = self;
        time.iter()
            .copied()
            .zip(distance.iter().copied())
            .map(|(time, current_distance_record)| {
                (1..time.saturating_sub(1))
                    .filter(|&button_hold_time| {
                        (time - button_hold_time)
                            .checked_mul(button_hold_time)
                            .unwrap()
                            > current_distance_record
                    })
                    .count()
            })
            .product::<usize>()
    }
}

#[test]
fn part_1_example() {
    let races = Races::new(EXAMPLE_PART_1);
    assert_debug_snapshot!(races);
    assert_eq!(races.winning_races_product(), 288);
}

const PUZZLE_INPUT: &str = include_str!("d6.txt");

#[test]
fn part_1() {
    let races = Races::new(PUZZLE_INPUT);
    assert_eq!(races.winning_races_product(), 252000);
}
