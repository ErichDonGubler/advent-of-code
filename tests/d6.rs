use insta::assert_debug_snapshot;
use itertools::Itertools;

const EXAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

#[derive(Debug)]
struct RacesPart1 {
    time: Vec<u64>,
    distance: Vec<u64>,
}

impl RacesPart1 {
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
                num_ways_to_beat_record(time, current_distance_record)
            })
            .product::<usize>()
    }
}

fn num_ways_to_beat_record(time: u64, current_distance_record: u64) -> usize {
    (1..time.saturating_sub(1))
        .filter(|&button_hold_time| {
            (time - button_hold_time)
                .checked_mul(button_hold_time)
                .unwrap()
                > current_distance_record
        })
        .count()
}

#[test]
fn part_1_example() {
    let races = RacesPart1::new(EXAMPLE);
    assert_debug_snapshot!(races);
    assert_eq!(races.winning_races_product(), 288);
}

const PUZZLE_INPUT: &str = include_str!("d6.txt");

#[test]
fn part_1() {
    let races = RacesPart1::new(PUZZLE_INPUT);
    assert_eq!(races.winning_races_product(), 252000);
}

fn winning_races_product_part_2(input: &str) -> usize {
    let (time, distance) = input.lines().collect_tuple().unwrap();
    let parse_num = |s: &str| {
        s.split_ascii_whitespace()
            .flat_map(|s| s.chars().filter(char::is_ascii_digit))
            .fold(0u64, |acc, c| {
                acc.checked_mul(10)
                    .unwrap()
                    .checked_add(c.to_digit(10).unwrap().into())
                    .unwrap()
            })
    };
    let time = time.strip_prefix("Time:").map(parse_num).unwrap();
    let current_distance_record = distance.strip_prefix("Distance:").map(parse_num).unwrap();
    num_ways_to_beat_record(time, current_distance_record)
}

#[test]
fn part_2_example() {
    assert_eq!(winning_races_product_part_2(EXAMPLE), 71503);
}

#[test]
fn part_2() {
    assert_eq!(winning_races_product_part_2(PUZZLE_INPUT), 36992486);
}
