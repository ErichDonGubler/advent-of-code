use advent_of_code_2024::{
    search_direction::{SearchDirection, Sign},
    uniform_width_ascii_lines,
};
use itertools::Itertools as _;
use strum::IntoEnumIterator as _;

const EXAMPLE_P1: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn parse_grid(input: &str) -> (Vec<&[u8]>, usize) {
    let parsed = uniform_width_ascii_lines(input.lines())
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let width = parsed[0].len();
    (parsed, width)
}

/// `dimensions` is `(num_rows, num_cols)`.
fn find_words<'a>(
    dimensions: (usize, usize),
    iter: impl ExactSizeIterator<Item = &'a [u8]> + Clone,
    words_to_match: &[&[u8]],
) -> u32 {
    let search_directions = Sign::iter()
        .cartesian_product(Sign::iter())
        .filter(|(s1, s2)| ![s1, s2].into_iter().all(|s| *s == Sign::Neutral))
        .map(|(horizontal, vertical)| SearchDirection {
            horizontal,
            vertical,
        });

    let mut num_matches_found = 0u32;
    for (row_idx, line) in iter.clone().enumerate() {
        for col_idx in 0..line.len() {
            for word in words_to_match {
                'search_direction: for search_direction in search_directions.clone() {
                    for (search_offset, word_cell_value) in word.iter().copied().enumerate() {
                        let grid_cell_value = search_direction
                            .to_2d_offsets((row_idx, col_idx), dimensions, search_offset)
                            .map(|(r, c)| iter.clone().nth(r).unwrap()[c]);

                        if grid_cell_value != Some(word_cell_value) {
                            continue 'search_direction;
                        }
                    }
                    num_matches_found = num_matches_found.checked_add(1).unwrap();
                }
            }
        }
    }
    num_matches_found
}

fn word_search_p1(input: &str) -> u32 {
    let words_to_match: &[&[u8]] = &[b"XMAS"];
    let (letter_grid, width) = parse_grid(input);
    find_words(
        (letter_grid.len(), width),
        letter_grid.iter().copied(),
        words_to_match,
    )
}

#[test]
fn p1_example() {
    assert_eq!(word_search_p1(EXAMPLE_P1), 18)
}

const INPUT: &str = include_str!("./d4.txt");

#[test]
fn p1() {
    assert_eq!(word_search_p1(INPUT), 2344)
}

const EXAMPLE_P2: &str = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";

fn word_search_p2(input: &str) -> u32 {
    let (letter_grid, width) = parse_grid(input);

    const PATTERN_DIMENSION: usize = 3;

    let mut num_matches_found = 0u32;
    for row_idx in 0..letter_grid.len().saturating_sub(PATTERN_DIMENSION - 1) {
        for col_idx in 0..width.saturating_sub(PATTERN_DIMENSION - 1) {
            if letter_grid[row_idx + 1][col_idx + 1] == b'A' {
                let check_corner =
                    |row_idx: usize, col_idx: usize| match letter_grid[row_idx][col_idx] {
                        b'M' => Some(b'S'),
                        b'S' => Some(b'M'),
                        _ => None,
                    };
                if let Some((bottom_right_expected, bottom_left_expected)) =
                    check_corner(row_idx, col_idx).zip(check_corner(row_idx, col_idx + 2))
                {
                    if letter_grid[row_idx + 2][col_idx + 2] == bottom_right_expected
                        && letter_grid[row_idx + 2][col_idx] == bottom_left_expected
                    {
                        num_matches_found = num_matches_found.checked_add(1).unwrap();
                    }
                }
            }
        }
    }
    num_matches_found
}

#[test]
fn p2_example() {
    assert_eq!(word_search_p2(EXAMPLE_P2), 9);
}

#[test]
fn p2() {
    assert_eq!(word_search_p2(INPUT), 1815);
}
