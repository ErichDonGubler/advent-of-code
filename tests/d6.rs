use std::collections::{HashMap, HashSet};

use advent_of_code_2024::{
    search_direction::{SearchDirection, Sign},
    uniform_width_ascii_lines,
};

const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

impl GuardDirection {
    pub fn next(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug)]
enum Tile {
    Obstacle,
    Empty,
}

struct ParsedInput {
    guard: Guard,
    grid: Vec<Vec<Tile>>,
}

fn parse_grid(input: &str) -> ParsedInput {
    let mut guard_position = None;
    let grid = uniform_width_ascii_lines(input.lines())
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| match c {
                    '#' => Tile::Obstacle,
                    '.' => Tile::Empty,
                    '^' => {
                        assert!(
                            guard_position.replace((row_idx, col_idx)).is_none(),
                            "multiple guard positions found"
                        );
                        Tile::Empty
                    }
                    _ => panic!("unrecognized tile {c:?}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let guard_position = guard_position.expect("no guard position found");

    let guard = Guard::new(guard_position);
    ParsedInput { guard, grid }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Guard {
    pub position: (usize, usize),
    pub direction: GuardDirection,
}

impl Guard {
    pub fn new(position: (usize, usize)) -> Self {
        Self {
            position,
            direction: GuardDirection::Up,
        }
    }

    fn front_facing_tile(&self, bounds: (usize, usize)) -> Option<(usize, usize)> {
        let search_direction = {
            let (horizontal, vertical) = match self.direction {
                GuardDirection::Up => (Sign::Neutral, Sign::Negative),
                GuardDirection::Right => (Sign::Positive, Sign::Neutral),
                GuardDirection::Down => (Sign::Neutral, Sign::Positive),
                GuardDirection::Left => (Sign::Negative, Sign::Neutral),
            };
            SearchDirection {
                horizontal,
                vertical,
            }
        };

        search_direction.to_2d_offsets(self.position, bounds, 1)
    }

    pub fn make_next_move(&mut self, grid: &[Vec<Tile>]) -> Option<(usize, usize)> {
        let start_direction = self.direction;
        loop {
            match self.front_facing_tile((grid.len(), grid[0].len())) {
                Some(position @ (row_idx, col_idx)) => match grid[row_idx][col_idx] {
                    Tile::Obstacle => {
                        self.direction = self.direction.next();
                        if self.direction == start_direction {
                            break None;
                        }
                    }
                    Tile::Empty => {
                        self.position = position;
                        break Some(position);
                    }
                },
                None => break None,
            }
        }
    }
}

fn guard_patrol_tiles_visited(input: &str) -> usize {
    let ParsedInput { mut guard, grid } = parse_grid(input);

    let mut guard_positions_visited = HashMap::<usize, HashSet<usize>>::new();

    while let Some((row_idx, col_idx)) = guard.make_next_move(&grid) {
        guard_positions_visited
            .entry(row_idx)
            .or_default()
            .insert(col_idx);
    }

    guard_positions_visited
        .values()
        .map(|p| p.len())
        .sum::<usize>()
}

#[test]
fn p1_example() {
    assert_eq!(guard_patrol_tiles_visited(EXAMPLE), 41);
}

const INPUT: &str = include_str!("./d6.txt");

#[test]
fn p1() {
    assert_eq!(guard_patrol_tiles_visited(INPUT), 4656);
}

fn num_forever_obstacle_positions_for_guard_patrol(input: &str) -> usize {
    let ParsedInput {
        mut guard,
        mut grid,
    } = parse_grid(input);

    let mut forever_obstacle_positions = HashMap::<usize, HashSet<usize>>::new();
    let original_guard_state = guard.clone();

    for obstacle_row_idx in 0..grid.len() {
        for obstacle_col_idx in 0..(grid[0].len()) {
            macro_rules! obstacle_cell {
                () => {
                    &mut grid[obstacle_row_idx][obstacle_col_idx]
                };
            }
            match obstacle_cell!() {
                Tile::Obstacle => continue,
                Tile::Empty => {
                    if (obstacle_row_idx, obstacle_col_idx) == original_guard_state.position {
                        continue;
                    }
                }
            }

            guard = original_guard_state.clone();
            *obstacle_cell!() = Tile::Obstacle;
            {
                let mut visited = HashMap::<usize, HashMap<usize, Guard>>::new();
                loop {
                    let should_break =
                        guard
                            .make_next_move(&grid)
                            .is_none_or(|(row_idx, col_idx)| {
                                use std::collections::hash_map::Entry;
                                let visited_in_same_orientation =
                                    match visited.entry(row_idx).or_default().entry(col_idx) {
                                        Entry::Occupied(occupied_entry) => {
                                            occupied_entry.get() == &guard
                                        }
                                        Entry::Vacant(entry) => {
                                            entry.insert(guard.clone());
                                            false
                                        }
                                    };
                                if visited_in_same_orientation {
                                    forever_obstacle_positions
                                        .entry(obstacle_row_idx)
                                        .or_default()
                                        .insert(obstacle_col_idx);
                                }
                                visited_in_same_orientation
                            });

                    if should_break {
                        break;
                    }
                }
            }
            *obstacle_cell!() = Tile::Empty;
        }
    }

    forever_obstacle_positions
        .values()
        .map(|ps| ps.len())
        .sum::<usize>()
}

#[test]
fn p2_example() {
    assert_eq!(num_forever_obstacle_positions_for_guard_patrol(EXAMPLE), 6);
}

#[test]
#[ignore]
fn p2() {
    assert_eq!(num_forever_obstacle_positions_for_guard_patrol(INPUT), 1575);
}
