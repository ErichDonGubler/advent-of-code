use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::{self, Debug, Display, Formatter},
    ops::Range,
};

use insta::assert_debug_snapshot;
use itertools::Itertools;
use nutype::nutype;

#[derive(Eq, PartialEq)]
pub(crate) struct Schematic {
    pub(crate) width: Column,
    pub(crate) height: Line,
    pub(crate) numbers: BTreeMap<Line, Vec<(ColumnRange, u16)>>,
    pub(crate) symbols: Vec<(Line, Column)>,
}

impl Debug for Schematic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self {
            width,
            height,
            numbers,
            symbols,
        } = self;
        f.debug_struct("Schematic")
            .field("width", width)
            .field("height", height)
            .field(
                "numbers",
                &format::Debug(|f| {
                    f.debug_map()
                        .entries(numbers.iter().map(|(k, v)| {
                            (
                                k,
                                format::Debug(|f| {
                                    f.debug_map()
                                        .entries(v.iter().map(|(k, v)| (k, v)))
                                        .finish()
                                }),
                            )
                        }))
                        .finish()
                }),
            )
            .field("symbols", &symbols)
            .finish()
    }
}

impl Schematic {
    pub fn new(input: &str) -> Self {
        let mut numbers = BTreeMap::<_, Vec<(_, _)>>::new();
        let mut symbols = Vec::new();
        let mut height = Line::new(0);
        let mut width = None;
        for (line_num, line) in input.lines().enumerate().map(|(i, c)| (Line::new(i), c)) {
            height = line_num;

            let line_width = Column::new(line.chars().count()); // FIXME: Oof, this ain't no rendered width
                                                                // beyond ASCII. 😬 getouttahere.jpg
            let old_width = width.replace(line_width);
            if let Some(old_width) = old_width {
                if old_width != line_width {
                    panic!(
                        concat!(
                            "line {} is {} characters long, ",
                            "but previous line(s) were {} characters long"
                        ),
                        line_num, line_width, old_width
                    )
                }
            }

            let mut line_chars = line.char_indices().peekable();
            while let Some((idx, c)) = line_chars.next() {
                let col = Column::new;
                match c {
                    c if c.is_ascii_digit() => {
                        line_chars
                            .take_while_ref(|(_i, c)| c.is_ascii_digit())
                            .last();
                        let number_end_idx =
                            line_chars.peek().map(|(i, _c)| *i).unwrap_or(line.len());
                        let number_start_idx = idx;
                        numbers.entry(line_num).or_default().push((
                            ColumnRange::new(col(number_start_idx)..col(number_end_idx)),
                            line[number_start_idx..number_end_idx]
                                .parse::<u16>()
                                .unwrap(),
                        ));
                    }
                    '.' => (),
                    '*' | '#' | '$' | '+' | '-' | '@' | '=' | '%' | '&' | '/' => {
                        symbols.push((line_num, col(idx)));
                    }
                    invalid => panic!(
                        "invalid schematic at {}:{}; \
                    expected digit, '.', or symbol character, found {:?}",
                        line_num,
                        col(idx),
                        invalid
                    ),
                }
            }
        }
        Self {
            height,
            width: width.unwrap_or(Column::new(0)),
            numbers,
            symbols,
        }
    }

    fn part_numbers(&self) -> BTreeSet<(Line, ColumnRange, u16)> {
        let Self {
            numbers,
            symbols,
            width,
            height,
        } = self;
        let mut found_symbols = BTreeSet::new();
        for (sym_line, sym_col) in symbols {
            for line in sym_line
                .saturating_sub(1)
                .range_inclusive(sym_line.saturating_add(1).min(*height))
            {
                for column in sym_col
                    .saturating_sub(1)
                    .range_inclusive(sym_col.saturating_add(1).min(*width))
                {
                    if let Some((col_range, num)) = numbers.get(&line).and_then(|line_nums| {
                        let search_idx = line_nums.binary_search_by(|(range, _val)| {
                            let Range { start, end } = range.clone().into_inner();
                            match start.cmp(&column) {
                                Ordering::Less if end <= column => Ordering::Less,
                                Ordering::Less | Ordering::Equal => Ordering::Equal,
                                Ordering::Greater => Ordering::Greater,
                            }
                        });
                        search_idx.ok().map(|idx| line_nums[idx].clone())
                    }) {
                        found_symbols.insert((line, col_range, num));
                    }
                }
            }
        }
        found_symbols
    }
}

#[nutype(derive(Clone, Copy, Eq, FromStr, Ord, PartialEq, PartialOrd))]
pub struct Line(usize);

impl Line {
    pub fn range_inclusive(self, upper_bound: Self) -> impl Iterator<Item = Self> {
        (self.into_inner()..=upper_bound.into_inner()).map(Self::new)
    }

    pub fn saturating_add(self, rhs: usize) -> Self {
        let lhs = self.into_inner();
        Self::new(lhs.saturating_add(rhs))
    }

    pub fn saturating_sub(self, rhs: usize) -> Self {
        let lhs = self.into_inner();
        Self::new(lhs.saturating_sub(rhs))
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.into_inner().checked_add(1).unwrap(), f)
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.into_inner().checked_add(1).unwrap(), f)
    }
}

#[nutype(derive(Clone, Copy, Eq, FromStr, Ord, PartialEq, PartialOrd))]
pub struct Column(usize);

impl Column {
    pub fn range_inclusive(self, upper_bound: Self) -> impl Iterator<Item = Self> {
        (self.into_inner()..=upper_bound.into_inner()).map(Self::new)
    }

    pub fn saturating_add(self, rhs: usize) -> Self {
        let lhs = self.into_inner();
        Self::new(lhs.saturating_add(rhs))
    }

    pub fn saturating_sub(self, rhs: usize) -> Self {
        let lhs = self.into_inner();
        Self::new(lhs.saturating_sub(rhs))
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.into_inner().checked_add(1).unwrap(), f)
    }
}

impl Debug for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.into_inner().checked_add(1).unwrap(), f)
    }
}

#[nutype(derive(Clone, Eq, PartialEq))]
pub struct ColumnRange(Range<Column>);

impl Debug for ColumnRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.clone().into_inner(), f)
    }
}

impl PartialOrd for ColumnRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ColumnRange {
    fn cmp(&self, other: &Self) -> Ordering {
        let Range {
            start: start_lhs,
            end: end_lhs,
        } = self.clone().into_inner();
        let Range {
            start: start_rhs,
            end: end_rhs,
        } = other.clone().into_inner();
        start_lhs.cmp(&start_rhs).then(end_lhs.cmp(&end_rhs))
    }
}

const EXAMPLE_ENGINE_SCHEMATIC: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

#[test]
fn part_1_example() {
    let schematic = Schematic::new(EXAMPLE_ENGINE_SCHEMATIC);
    assert_debug_snapshot!("parsed_example_schematic", schematic);
    let part_numbers = schematic.part_numbers();
    assert_debug_snapshot!("part_1_part_numbers", part_numbers);
    assert_eq!(
        part_numbers
            .iter()
            .map(|(_line, _col_range, val)| u64::from(*val))
            .sum::<u64>(),
        4361
    );
}

const PUZZLE_INPUT: &str = include_str!("d3.txt");

#[test]
fn part_1() {
    assert_eq!(
        Schematic::new(PUZZLE_INPUT)
            .part_numbers()
            .iter()
            .map(|(_line, _col_range, val)| u64::from(*val))
            .sum::<u64>(),
        507214
    );
}
