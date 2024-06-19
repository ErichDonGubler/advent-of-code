use insta::assert_debug_snapshot;
use itertools::Itertools;

const EXAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

#[derive(Debug)]
struct AnalyzedValue {
    value_over_time: Vec<i32>,
    analysis: Vec<Vec<i32>>,
}

#[derive(Debug)]
struct OasisAnalyzer {
    values: Vec<AnalyzedValue>,
}

impl OasisAnalyzer {
    pub fn new(line: &str) -> Self {
        let values = line
            .lines()
            .map(str::trim)
            .map(|line| {
                let value_over_time = line
                    .split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();

                let analysis = {
                    let mut analyzed = Vec::new();
                    let mut last_row = value_over_time.as_slice();
                    while !last_row.iter().copied().all(|val| val == 0) {
                        let next_row = last_row
                            .iter()
                            .copied()
                            .tuple_windows()
                            .map(|(a, b)| b - a)
                            .collect::<Vec<_>>();
                        assert!(
                            !next_row.is_empty(),
                            "analysis inconclusive for {:?}; dump of work: {:?}",
                            value_over_time,
                            analyzed,
                        );
                        analyzed.push(next_row);
                        last_row = analyzed.last().unwrap();
                    }
                    analyzed
                };
                AnalyzedValue {
                    value_over_time,
                    analysis,
                }
            })
            .collect();
        Self { values }
    }

    pub fn predict_next_in_geometric_seq(&self) -> impl Iterator<Item = i32> + '_ {
        let Self { values } = self;
        values.iter().map(|value_over_time| {
            let AnalyzedValue {
                value_over_time,
                analysis,
            } = value_over_time;
            analysis
                .iter()
                .rev()
                .map(|row| row.last().unwrap())
                .chain(Some(value_over_time.last().unwrap()))
                .fold(0, |acc, val| acc + val)
        })
    }
}

#[test]
fn part_1_example() {
    let analyzer = OasisAnalyzer::new(EXAMPLE);
    assert_debug_snapshot!(analyzer);
    assert_eq!(
        analyzer.predict_next_in_geometric_seq().collect::<Vec<_>>(),
        vec![18, 28, 68]
    );
}

#[test]
fn part_1() {
    assert_eq!(
        OasisAnalyzer::new(include_str!("./d9.txt"))
            .predict_next_in_geometric_seq()
            .sum::<i32>(),
        1725987467
    );
}
