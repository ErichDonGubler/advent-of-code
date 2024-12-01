use std::{collections::HashMap, str::FromStr};

use insta::assert_debug_snapshot;
use itertools::Itertools as _;

const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn parse_ids(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(u64::from_str)
            .map(Result::unwrap)
            .collect_tuple::<(_, _)>()
            .expect("ofrick, line that doesn't have two values")
    })
}

#[test]
fn parsing() {
    assert_debug_snapshot!(parse_ids(EXAMPLE).collect::<Vec<_>>());
}

fn sorted_diff_sum(input: &str) -> u64 {
    let (mut list1, mut list2) = parse_ids(input).unzip::<_, _, Vec<u64>, Vec<u64>>();

    assert_eq!(list1.len(), list2.len());

    list1.sort();
    list2.sort();

    list1
        .iter()
        .copied()
        .zip(list2.iter().copied())
        .map(|(item1, item2)| item1.abs_diff(item2))
        .sum()
}

#[test]
fn p1_example() {
    assert_eq!(sorted_diff_sum(EXAMPLE), 11);
}

const INPUT: &str = include_str!("./d1.txt");

#[test]
fn p1() {
    assert_eq!(sorted_diff_sum(INPUT), 1660292);
}

fn similarity_score_sum(input: &str) -> u64 {
    // Do a bit of cleverness where we can just increment the occurrences of numbers from _both_
    // lists, since everything is commutative.
    #[derive(Debug, Default)]
    struct ListEntry {
        num_left: u64,
        num_right: u64,
    }

    parse_ids(input)
        .fold(
            HashMap::<u64, ListEntry>::new(),
            |mut acc, (item1, item2)| {
                acc.entry(item1).or_default().num_left += 1;
                acc.entry(item2).or_default().num_right += 1;
                acc
            },
        )
        .iter()
        .map(
            |(
                key,
                ListEntry {
                    num_left,
                    num_right,
                },
            )| key * num_left * num_right,
        )
        .sum()
}

#[test]
fn p2_example() {
    assert_eq!(similarity_score_sum(EXAMPLE), 31);
}

#[test]
fn p2() {
    assert_eq!(similarity_score_sum(INPUT), 22776016);
}
