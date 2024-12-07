use std::iter::repeat_n;

use itertools::Itertools;

const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn parse_equations(input: &str) -> impl Iterator<Item = (u64, Vec<u64>)> + '_ {
    input.lines().map(|line| {
        let (test_value, terms) = line.split_once(": ").unwrap();
        let test_value: u64 = test_value.parse().unwrap();
        let terms = terms
            .split(' ')
            .map(|term| term.parse().unwrap())
            .collect::<Vec<u64>>();
        (test_value, terms)
    })
}

trait Operation: strum::IntoEnumIterator + Clone {
    fn execute(&self, lhs: u64, rhs: u64) -> u64;
}

fn total_calibration_result<Op>(input: &str) -> u64
where
    Op: Operation,
{
    parse_equations(input)
        .filter_map(|(test_value, terms)| {
            let test_value_synthesizable_from_terms = match terms.len() {
                0 => false,
                1 => terms[0] == test_value,
                _multiple => repeat_n(Op::iter(), terms.len() - 1)
                    .multi_cartesian_product()
                    .any(|operators| {
                        let mut terms_iter = terms.iter().copied();
                        let mut acc = terms_iter.next().unwrap();
                        for (op, term) in operators.iter().zip_eq(terms_iter) {
                            acc = op.execute(acc, term)
                        }
                        acc == test_value
                    }),
            };
            test_value_synthesizable_from_terms.then_some(test_value)
        })
        .sum::<u64>()
}

#[derive(Clone, Copy, Debug, strum::EnumIter)]
enum OperationP1 {
    Mul,
    Add,
}

impl Operation for OperationP1 {
    #[track_caller]
    fn execute(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            OperationP1::Mul => lhs.checked_mul(rhs).unwrap(),
            OperationP1::Add => lhs.checked_add(rhs).unwrap(),
        }
    }
}

#[test]
fn p1_example() {
    assert_eq!(total_calibration_result::<OperationP1>(EXAMPLE), 3749);
}

const INPUT: &str = include_str!("./d7.txt");

#[test]
fn p1() {
    assert_eq!(
        total_calibration_result::<OperationP1>(INPUT),
        6231007345478
    );
}

#[derive(Clone, Copy, Debug, strum::EnumIter)]
enum OperationP2 {
    Mul,
    Add,
    Concat,
}

impl Operation for OperationP2 {
    fn execute(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            OperationP2::Mul => OperationP1::Mul.execute(lhs, rhs),
            OperationP2::Add => OperationP1::Add.execute(lhs, rhs),
            OperationP2::Concat => {
                let power_of_ten = rhs.ilog10() + 1;
                lhs.checked_mul(10u64.checked_pow(power_of_ten).unwrap())
                    .unwrap()
                    .checked_add(rhs)
                    .unwrap()
            }
        }
    }
}

#[test]
fn p2_example() {
    assert_eq!(total_calibration_result::<OperationP2>(EXAMPLE), 11387);
}

#[test]
#[ignore]
fn p2() {
    assert_eq!(
        total_calibration_result::<OperationP2>(INPUT),
        333027885676693
    );
}
