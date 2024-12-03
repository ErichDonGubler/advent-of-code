use std::sync::OnceLock;

use regex::Regex;

fn process_mul_instructions_p1(input: &str) -> u32 {
    static MUL_RE: OnceLock<Regex> = OnceLock::new();

    let mul_re = MUL_RE.get_or_init(|| {
        Regex::new(concat!(
            r"mul\(",
            r"(?P<lhs>\d+)",
            ",",
            r"(?P<rhs>\d+)",
            r"\)"
        ))
        .unwrap()
    });
    mul_re
        .captures_iter(input)
        .map(|caps| {
            (
                caps["lhs"].parse::<u32>().unwrap(),
                caps["rhs"].parse::<u32>().unwrap(),
            )
        })
        .map(|(a, b)| a * b)
        .sum::<u32>()
}

const EXAMPLE_P1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

#[test]
fn p1_example() {
    assert_eq!(process_mul_instructions_p1(EXAMPLE_P1), 161);
}

const INPUT: &str = include_str!("./d3.txt");

#[test]
fn p1() {
    assert_eq!(process_mul_instructions_p1(INPUT), 173419328);
}

const EXAMPLE_P2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn process_mul_instructions_p2(input: &str) -> u32 {
    static INSTRUCTION_RE: OnceLock<Regex> = OnceLock::new();

    let instruction_re = INSTRUCTION_RE.get_or_init(|| {
        Regex::new(concat!(
            r"(?P<dont_op>don't\(\))",
            "|",
            r"(?P<do_op>do\(\))",
            "|",
            concat!(
                // meta: `rustfmt` plz no single line
                "(?P<mul_op>mul)",
                r"\(",
                r"(?P<mul_lhs>\d+)",
                ",",
                r"(?P<mul_rhs>\d+)",
                r"\)"
            ),
        ))
        .unwrap()
    });
    let mut accepting_mut = true;
    instruction_re
        .captures_iter(input)
        .filter_map(|caps| {
            #[derive(Debug)]
            enum Op {
                Mul,
                Dont,
                Do,
            }
            let op: Op = caps
                .name("mul_op")
                .map(|_| Op::Mul)
                .or(caps.name("dont_op").map(|_| Op::Dont))
                .or(caps.name("do_op").map(|_| Op::Do))
                .unwrap();

            match op {
                Op::Mul => accepting_mut.then_some((
                    caps["mul_lhs"].parse::<u32>().unwrap(),
                    caps["mul_rhs"].parse::<u32>().unwrap(),
                )),
                Op::Dont => {
                    accepting_mut = false;
                    None
                }
                Op::Do => {
                    accepting_mut = true;
                    None
                }
            }
        })
        .map(|(a, b)| a * b)
        .sum::<u32>()
}

#[test]
fn p2_example() {
    assert_eq!(process_mul_instructions_p2(EXAMPLE_P2), 48);
}

#[test]
fn p2() {
    assert_eq!(process_mul_instructions_p2(INPUT), 90669332);
}
