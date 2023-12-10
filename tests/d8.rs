use std::collections::BTreeMap;

use insta::assert_debug_snapshot;

const EXAMPLE_1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

const EXAMPLE_2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

#[derive(Debug)]
struct Map<'a> {
    instructions: Vec<Instr>,
    nodes: BTreeMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Map<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lines = input.lines();
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => Instr::Left,
                'R' => Instr::Right,
                _ => panic!("invalid LR instruction {c:?}"),
            })
            .collect();

        let nodes = lines
            .filter_map(|line| {
                if line.trim().is_empty() {
                    None
                } else {
                    let rest = line;
                    let (name, rest) = rest.split_once(" = (").unwrap();
                    let (left, rest) = rest.split_once(", ").unwrap();
                    let (right, rest) = rest.split_once(')').unwrap();
                    assert!(rest.is_empty());
                    Some((name, (left, right)))
                }
            })
            .collect();

        Self {
            instructions,
            nodes,
        }
    }
}

#[derive(Debug)]
enum Instr {
    Left,
    Right,
}

fn num_instrs_until_complete(map: &Map) -> u64 {
    let Map {
        instructions,
        nodes,
    } = map;

    let mut curr_node = "AAA";
    let mut instructions = instructions.iter().cycle();
    let mut count = 0u64;
    loop {
        count = count.checked_add(1).unwrap();
        let (left, right) = nodes[curr_node];
        curr_node = match instructions.next().unwrap() {
            Instr::Left => left,
            Instr::Right => right,
        };
        if curr_node == "ZZZ" {
            return count;
        }
    }
}

#[test]
fn part_1_example() {
    let example_1 = Map::new(EXAMPLE_1);
    assert_debug_snapshot!(example_1);
    assert_eq!(num_instrs_until_complete(&example_1), 2);

    let example_2 = Map::new(EXAMPLE_2);
    assert_debug_snapshot!(example_2);
    assert_eq!(num_instrs_until_complete(&example_2), 6);
}

const PUZZLE_INPUT: &str = include_str!("d8.txt");

#[test]
fn part_1() {
    assert_eq!(num_instrs_until_complete(&Map::new(PUZZLE_INPUT)), 15517);
}
