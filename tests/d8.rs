use std::{
    collections::{HashMap, HashSet},
    iter,
};

use advent_of_code_2024::{
    space::{d2::Size, Coord},
    uniform_width_ascii_lines,
};
use itertools::Itertools;

const EXAMPLE_P1: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn antennae_by_frequency(input: &str) -> (Size, HashMap<u8, HashMap<Coord, HashSet<Coord>>>) {
    let lines =
        uniform_width_ascii_lines(input.lines()).map(|line| line.as_bytes().iter().copied());

    let grid_size = {
        let grid_height = lines.clone().count();
        let grid_width = lines.clone().next().unwrap().len();

        Size::from_row_major((grid_height, grid_width))
    };

    let mut antennae_by_frequency = HashMap::<_, HashMap<_, HashSet<_>>>::new();

    for (row_idx, line) in lines.enumerate() {
        for (col_idx, cell_value) in line.enumerate() {
            assert!(cell_value.is_ascii_graphic());
            match cell_value {
                b'.' => (),
                b'#' => {
                    static EMITTED_WARNING: std::sync::atomic::AtomicBool =
                        std::sync::atomic::AtomicBool::new(false);
                    if !EMITTED_WARNING.swap(true, std::sync::atomic::Ordering::AcqRel) {
                        eprintln!("WARNING: ignoring '#' (antinode) tiles");
                    }
                }
                antenna_frequency => {
                    antennae_by_frequency
                        .entry(antenna_frequency)
                        .or_default()
                        .entry(Coord::new(row_idx))
                        .or_default()
                        .insert(Coord::new(col_idx));
                }
            }
        }
    }

    (grid_size, antennae_by_frequency)
}

fn calculate_antinodes_p1(input: &str) -> usize {
    use advent_of_code_2024::space::d2::{apply_rel_offset, Coords};

    let (grid_size, antennae_by_frequency) = antennae_by_frequency(input);
    let antennae_by_frequency = antennae_by_frequency.iter().map(|(freq, locs)| {
        (
            freq,
            locs.iter().flat_map(|(row_idx, cols)| {
                cols.iter()
                    .copied()
                    .map(|col| Coords { row: *row_idx, col })
            }),
        )
    });

    let mut antinodes_by_location = HashMap::<_, HashMap<_, HashSet<_>>>::new();
    for (frequency, coordinate_pairs) in antennae_by_frequency {
        for (first, second) in coordinate_pairs.tuple_combinations() {
            let antinode_pair_half =
                |first, second| apply_rel_offset(grid_size, second, first - second);
            for antinode_coords in [
                antinode_pair_half(first, second),
                antinode_pair_half(second, first),
            ]
            .into_iter()
            .flatten()
            {
                let Coords { row, col } = antinode_coords;
                antinodes_by_location
                    .entry(row)
                    .or_default()
                    .entry(col)
                    .or_default()
                    .insert(frequency);
            }
        }
    }

    antinodes_by_location
        .values()
        .map(|cols| cols.len())
        .sum::<usize>()
}

#[test]
fn p1_example() {
    assert_eq!(calculate_antinodes_p1(EXAMPLE_P1), 14);
}

const INPUT: &str = include_str!("./d8.txt");

#[test]
fn p1() {
    assert_eq!(calculate_antinodes_p1(INPUT), 311);
}

fn calculate_antinodes_p2(input: &str) -> usize {
    use advent_of_code_2024::space::d2::{apply_rel_offset, Coords};

    let (grid_size, antennae_by_frequency) = antennae_by_frequency(input);
    let antennae_by_frequency = antennae_by_frequency.iter().map(|(freq, locs)| {
        (
            freq,
            locs.iter().flat_map(|(row_idx, cols)| {
                cols.iter()
                    .copied()
                    .map(|col| Coords { row: *row_idx, col })
            }),
        )
    });

    let mut antinodes_by_location = HashMap::<_, HashMap<_, HashSet<_>>>::new();
    for (frequency, coordinate_pairs) in antennae_by_frequency {
        for (first, second) in coordinate_pairs.tuple_combinations() {
            let antinode_pair_half = |first: Coords, second| {
                let offset = first - second;
                let mut multiple = 0;
                iter::from_fn(move || {
                    let multiplied_offset = offset.checked_mul(multiple).unwrap();
                    let ret = apply_rel_offset(grid_size, second, multiplied_offset)?;
                    multiple += 1;
                    Some(ret)
                })
            };
            for antinode_coords in [
                antinode_pair_half(first, second),
                antinode_pair_half(second, first),
            ]
            .into_iter()
            .flatten()
            {
                let Coords { row, col } = antinode_coords;
                antinodes_by_location
                    .entry(row)
                    .or_default()
                    .entry(col)
                    .or_default()
                    .insert(frequency);
            }
        }
    }

    antinodes_by_location
        .values()
        .map(|cols| cols.len())
        .sum::<usize>()
}

const EXAMPLE_P2: &str = "\
T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........
";

#[test]
fn p2_example() {
    assert_eq!(calculate_antinodes_p2(EXAMPLE_P2), 9);
    assert_eq!(calculate_antinodes_p2(EXAMPLE_P1), 34);
}

#[test]
fn p2() {
    assert_eq!(calculate_antinodes_p2(INPUT), 1115);
}
