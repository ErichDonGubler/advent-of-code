use itertools::Itertools as _;

const EXAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

enum ProblemDampener {
    On,
    Off,
}

fn num_safe_reports(input: &str, problem_dampener: ProblemDampener) -> usize {
    input
        .lines()
        .filter(|l| {
            println!("line");
            let is_safe_gen = || {
                let mut direction = None;
                move |a: u8, b| match a.cmp(&b) {
                    std::cmp::Ordering::Equal => false,
                    ordering => {
                        let matches_expected = match direction {
                            None => {
                                direction = Some(ordering);
                                true
                            }
                            Some(expected) => ordering == expected,
                        };
                        matches_expected && a.abs_diff(b) <= 3
                    }
                }
            };
            let terms = l.split_whitespace().map(|term| term.parse::<u8>().unwrap());
            let count = terms.clone().count();

            let is_fully_safe = {
                let mut is_safe = is_safe_gen();
                terms.clone().tuple_windows().all(|(a, b)| is_safe(a, b))
            };

            match problem_dampener {
                ProblemDampener::Off => is_fully_safe,
                ProblemDampener::On => {
                    is_fully_safe || {
                        {
                            (0..count).any(|dampened_idx| {
                                let mut is_safe = is_safe_gen();
                                terms
                                    .clone()
                                    .enumerate()
                                    .filter_map(|(idx, t)| (idx != dampened_idx).then_some(t))
                                    .tuple_windows()
                                    .all(|(a, b)| is_safe(a, b))
                            })
                        }
                    }
                }
            }
        })
        .count()
}

#[test]
fn p1_example() {
    assert_eq!(num_safe_reports(EXAMPLE, ProblemDampener::Off), 2);
}

const INPUT: &str = include_str!("./d2.txt");

#[test]
fn p1() {
    assert_eq!(num_safe_reports(INPUT, ProblemDampener::Off), 686);
}

#[test]
fn p2_example() {
    assert_eq!(num_safe_reports(EXAMPLE, ProblemDampener::On), 4);
}

#[test]
fn p2() {
    assert_eq!(num_safe_reports(INPUT, ProblemDampener::On), 717);
}
