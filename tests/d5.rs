use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const EXAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

pub(crate) struct BeforeAfterRules {
    inner: HashMap<u8, HashSet<u8>>,
}

impl BeforeAfterRules {
    pub fn breaks_with(&self, before: u8, after: u8) -> bool {
        let Self { inner } = self;
        inner
            .get(&after)
            .is_some_and(|needs_after| needs_after.contains(&before))
    }
}

fn parse_p1(input: &str) -> (BeforeAfterRules, impl Iterator<Item = Vec<u8>> + '_) {
    let mut lines = input.lines();

    let mut before_after_rules = HashMap::<_, HashSet<_>>::new();
    lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .for_each(|line| {
            let (before, after) = line
                .splitn(2, "|")
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .expect("expected rule with `|`");
            before_after_rules.entry(before).or_default().insert(after);
        });

    let updates = lines.map(|l| {
        l.split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u8>>()
    });

    (
        BeforeAfterRules {
            inner: before_after_rules,
        },
        updates,
    )
}

fn correct_middle_page_number_sum(input: &str) -> u32 {
    let (before_after_rules, updates) = parse_p1(input);

    let mut middle_page_number_sum = 0u32;
    'next_row: for update in updates {
        let mut update_iter = update.iter().copied();
        while let Some(value) = update_iter.next() {
            for other_value in update_iter.clone() {
                if before_after_rules.breaks_with(value, other_value) {
                    continue 'next_row;
                }
            }
        }
        // TODO: Is this indexing right? The instructions say nothing about
        // even-numbered update rows. 🫤
        middle_page_number_sum = middle_page_number_sum
            .checked_add(update[update.len() / 2].into())
            .unwrap();
    }
    middle_page_number_sum
}

#[test]
fn p1_example() {
    assert_eq!(correct_middle_page_number_sum(EXAMPLE), 143);
}

const INPUT: &str = include_str!("./d5.txt");

#[test]
fn p1() {
    assert_eq!(correct_middle_page_number_sum(INPUT), 7024);
}

fn incorrect_middle_page_number_sum(input: &str) -> u32 {
    let (before_after_rules, updates) = parse_p1(input);

    let mut incorrect_middle_page_number_sum = 0u32;
    for mut update in updates {
        let mut needed_correction = false;
        let mut value_idx = 0;
        'outer: while value_idx < update.len() {
            let mut other_value_idx = value_idx + 1;
            while other_value_idx < update.len() {
                let other_value = update[other_value_idx];
                if before_after_rules.breaks_with(update[value_idx], other_value) {
                    needed_correction = true;
                    update.remove(other_value_idx);
                    update.insert(value_idx, other_value);
                    continue 'outer;
                }
                other_value_idx += 1;
            }
            value_idx += 1;
        }
        // TODO: Is this indexing right? The instructions say nothing about
        // even-numbered update rows. 🫤
        if needed_correction {
            incorrect_middle_page_number_sum = incorrect_middle_page_number_sum
                .checked_add(update[update.len() / 2].into())
                .unwrap();
        }
    }
    incorrect_middle_page_number_sum
}

#[test]
fn p2_example() {
    assert_eq!(incorrect_middle_page_number_sum(EXAMPLE), 123);
}

#[test]
fn p2() {
    assert_eq!(incorrect_middle_page_number_sum(INPUT), 4151);
}
