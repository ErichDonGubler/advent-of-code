mod calibration {
    #[derive(Debug, Eq, PartialEq)]
    pub(crate) struct Document {
        pub(crate) lines: Vec<Line>,
    }

    impl Document {
        #[track_caller]
        pub(crate) fn new_p1(input: &str) -> Self {
            Self {
                lines: input
                    .lines()
                    .map(|line| {
                        let mut digits = line
                            .chars()
                            .filter(|c| c.is_ascii_digit())
                            .map(|c| c.to_digit(10).unwrap().try_into().unwrap());
                        let first = digits.next().unwrap();
                        let last = digits.last().unwrap_or(first);
                        Line { first, last }
                    })
                    .collect::<Vec<_>>(),
            }
        }

        #[track_caller]
        pub(crate) fn new_p2(input: &str) -> Self {
            Self {
                lines: input
                    .lines()
                    .map(|line| {
                        let mut digits = line.char_indices().filter_map(|(idx, _c)| {
                            let slice = &line[idx..];
                            [
                                ("one", 1),
                                ("two", 2),
                                ("three", 3),
                                ("four", 4),
                                ("five", 5),
                                ("six", 6),
                                ("seven", 7),
                                ("eight", 8),
                                ("nine", 9),
                                ("1", 1),
                                ("2", 2),
                                ("3", 3),
                                ("4", 4),
                                ("5", 5),
                                ("6", 6),
                                ("7", 7),
                                ("8", 8),
                                ("9", 9),
                            ]
                            .into_iter()
                            .find_map(|(token, value)| {
                                if slice.starts_with(token) {
                                    Some(value)
                                } else {
                                    None
                                }
                            })
                            // OPT: Skip indices of terms we've parsed
                        });
                        let first = digits.next().unwrap();
                        let last = digits.last().unwrap_or(first);
                        Line { first, last }
                    })
                    .collect::<Vec<_>>(),
            }
        }

        pub(crate) fn process(&self) -> u64 {
            self.lines.iter().fold(0u64, |acc, Line { first, last }| {
                acc.checked_add(((first * 10) + last).into()).unwrap()
            })
        }
    }

    // OPT: Might be nice to have a newtype for single digits (using `nutype`?).
    #[derive(Debug, Eq, PartialEq)]
    pub(crate) struct Line {
        pub(crate) first: u8,
        pub(crate) last: u8,
    }
}

#[test]
fn part_1_example() {
    let parsed = calibration::Document::new_p1(
        "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
    );
    assert_eq!(
        parsed,
        calibration::Document {
            lines: vec![
                calibration::Line { first: 1, last: 2 },
                calibration::Line { first: 3, last: 8 },
                calibration::Line { first: 1, last: 5 },
                calibration::Line { first: 7, last: 7 },
            ],
        }
    );
    assert_eq!(parsed.process(), 142);
}

const INPUT: &str = include_str!("./d1.txt");

#[test]
fn part_1() {
    assert_eq!(calibration::Document::new_p1(INPUT).process(), 54877);
}

#[test]
fn part_2_example() {
    let parsed = calibration::Document::new_p2(
        "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
    );

    assert_eq!(
        parsed,
        calibration::Document {
            lines: vec![
                calibration::Line { first: 2, last: 9 },
                calibration::Line { first: 8, last: 3 },
                calibration::Line { first: 1, last: 3 },
                calibration::Line { first: 2, last: 4 },
                calibration::Line { first: 4, last: 2 },
                calibration::Line { first: 1, last: 4 },
                calibration::Line { first: 7, last: 6 },
            ],
        }
    );
    assert_eq!(parsed.process(), 281);
}

#[test]
fn part_2() {
    assert_eq!(calibration::Document::new_p2(INPUT).process(), 54100);
}
