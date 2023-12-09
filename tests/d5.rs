use std::{cmp::Ordering, fmt::Display, num::NonZeroUsize};

use chumsky::{
    error::Rich,
    extra,
    primitive::{end, group, just},
    text::{ascii::ident, digits, inline_whitespace, newline},
    IterParser, Parser,
};
use format::lazy_format;
use insta::assert_debug_snapshot;
use nutype::nutype;

mod spaces {
    use std::fmt::{self, Debug, Display, Formatter};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) struct Seed;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) struct Location;

    #[derive(Clone, Eq, PartialEq)]
    pub(crate) struct Dynamic<'a> {
        pub name: &'a str,
    }

    impl Debug for Dynamic<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let Self { name } = self;
            Debug::fmt(name, f)
        }
    }

    impl Display for Dynamic<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let Self { name } = self;
            Display::fmt(name, f)
        }
    }

    impl From<Seed> for Dynamic<'static> {
        fn from(value: Seed) -> Self {
            let Seed = value;
            Dynamic { name: "seed" }
        }
    }

    impl TryFrom<Dynamic<'_>> for Seed {
        type Error = ();

        fn try_from(value: Dynamic<'_>) -> Result<Self, Self::Error> {
            if let Dynamic { name: "seed" } = value {
                Ok(Self)
            } else {
                Err(())
            }
        }
    }

    impl From<Location> for Dynamic<'static> {
        fn from(value: Location) -> Self {
            let Location = value;
            Dynamic { name: "location" }
        }
    }

    impl TryFrom<Dynamic<'_>> for Location {
        type Error = ();

        fn try_from(value: Dynamic<'_>) -> Result<Self, Self::Error> {
            if let Dynamic { name: "location" } = value {
                Ok(Self)
            } else {
                Err(())
            }
        }
    }
}

#[nutype(derive(Clone, Copy, Debug, Display, Eq, Ord, PartialEq, PartialOrd))]
struct RawId(u64);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Id<Space> {
    value: RawId,
    space: Space,
}

#[derive(Debug)]
struct AlmanacConfig<'a> {
    seeds: SeedProvider,
    maps: Vec<Map<spaces::Dynamic<'a>, spaces::Dynamic<'a>>>,
}

#[derive(Debug)]
enum SeedProvider {
    Day1(Vec<Id<spaces::Seed>>),
    Day2(Vec<(Id<spaces::Seed>, u64)>),
}

impl SeedProvider {
    pub fn iter(&self) -> impl Iterator<Item = Id<spaces::Seed>> + '_ {
        let iter: Box<dyn Iterator<Item = Id<spaces::Seed>>> = match &self {
            SeedProvider::Day1(seeds) => Box::new(seeds.iter().cloned()),
            SeedProvider::Day2(seed_ranges) => {
                Box::new(seed_ranges.iter().cloned().flat_map(|(start, size)| {
                    let start = start.value.into_inner();
                    (start..(start + size)).map(|value| Id {
                        space: spaces::Seed,
                        value: RawId::new(value),
                    })
                }))
            }
        };
        iter
    }
}

impl<'a> AlmanacConfig<'a> {
    fn parse_u64<'b>() -> impl Parser<'b, &'b str, u64, extra::Err<Rich<'b, char>>> + Clone {
        digits(10).to_slice().from_str::<u64>().unwrapped()
    }

    fn parser<S>(seeds_marshaller: S) -> impl Parser<'a, &'a str, Self, extra::Err<Rich<'a, char>>>
    where
        S: Parser<'a, &'a str, SeedProvider, extra::Err<Rich<'a, char>>>,
    {
        let parse_u64 = digits(10).to_slice().from_str::<u64>().unwrapped();

        let raw_id = parse_u64.map(RawId::new);

        let seeds = just("seeds:")
            .ignore_then(seeds_marshaller)
            .then_ignore(newline().repeated().exactly(2).or(end()));

        let map = ident()
            .then_ignore(just("-to-"))
            .then(ident())
            .then_ignore(group((just(" map:"), newline())))
            .then(
                group((
                    inline_whitespace(),
                    raw_id.clone(),
                    inline_whitespace(),
                    raw_id,
                    inline_whitespace(),
                    digits(10).to_slice().from_str::<NonZeroUsize>().unwrapped(),
                    inline_whitespace(),
                    newline().or(end()),
                ))
                .map(
                    |((), start_id_to, (), start_id_from, (), size, (), ())| MapEntry {
                        start_id_from,
                        start_id_to,
                        size,
                    },
                )
                .repeated()
                .collect(),
            )
            .then_ignore(newline().or(end()))
            .map(|((from_space, to_space), entries)| Map {
                from_space: spaces::Dynamic { name: from_space },
                to_space: spaces::Dynamic { name: to_space },
                entries,
            });

        seeds
            .then(map.repeated().collect::<Vec<_>>())
            .map(|(seeds, maps)| AlmanacConfig { seeds, maps })
    }

    fn new<S>(input: &'a str, seeds_parser: S) -> Self
    where
        S: Parser<'a, &'a str, SeedProvider, extra::Err<Rich<'a, char>>>,
    {
        let Self { seeds, mut maps } = Self::parser(seeds_parser)
            .parse(input)
            .into_result()
            .unwrap();

        {
            let mut maps = maps.iter();
            let Map {
                from_space,
                to_space,
                entries: _,
            } = maps.next().unwrap();
            let spaces::Seed = from_space
                .clone()
                .try_into()
                .expect("first map does not map from `seed`");

            let mut to_space = to_space;
            loop {
                match to_space.clone().try_into() {
                    Ok(spaces::Location) => break,
                    Err(()) => {}
                }
                let map = maps.next().expect("last map does not map to `location`");
                let Map {
                    from_space: next_from_space,
                    to_space: next_to_space,
                    entries: _,
                } = map;
                assert_eq!(
                        to_space,
                        next_from_space,
                        "`{}` map is not continous with previous map; expected to map from `{to_space}`",
                        map.display_type(),
                        );
                to_space = next_to_space;
            }
        }

        maps.iter_mut()
            .for_each(|map| map.entries.sort_by_key(|entry| entry.start_id_from.clone()));

        Self { seeds, maps }
    }

    pub fn new_part_1(input: &'a str) -> Self {
        Self::new(
            input,
            Self::parse_u64()
                .map(RawId::new)
                .map(|value| Id {
                    value,
                    space: spaces::Seed,
                })
                .padded_by(inline_whitespace())
                .repeated()
                .at_least(1)
                .collect()
                .map(SeedProvider::Day1),
        )
    }

    pub fn new_part_2(input: &'a str) -> Self {
        let seed_range = Self::parse_u64()
            .map(RawId::new)
            .map(|value| Id {
                value,
                space: spaces::Seed,
            })
            .then_ignore(inline_whitespace())
            .then(Self::parse_u64());
        Self::new(
            input,
            seed_range
                .padded_by(inline_whitespace())
                .repeated()
                .at_least(1)
                .collect()
                .map(SeedProvider::Day2),
        )
    }

    pub fn lowest_translated_seed_location(&self) -> Id<spaces::Location> {
        let Self { seeds, maps } = self;

        let lowest_location_from_seed_seen = seeds
            .iter()
            .map(|seed| {
                let Id {
                    value: idx,
                    space: spaces::Seed,
                } = seed;

                maps.iter().fold(idx.clone(), |translated_idx, map| {
                    let Map {
                        from_space: _,
                        to_space: _,
                        entries,
                    } = map;
                    entries
                        .binary_search_by(|entry| {
                            fn compare(entry: &MapEntry, translated_idx: RawId) -> Ordering {
                                let MapEntry {
                                    start_id_from,
                                    start_id_to: _,
                                    size,
                                } = entry;
                                match start_id_from.cmp(&translated_idx) {
                                    Ordering::Less
                                        if start_id_from
                                            .into_inner()
                                            .checked_add(size.get().try_into().unwrap())
                                            .unwrap()
                                            > translated_idx.into_inner() =>
                                    {
                                        Ordering::Equal
                                    }
                                    ord => ord,
                                }
                            }
                            compare(entry, translated_idx)
                        })
                        .ok()
                        .map(|idx| {
                            let MapEntry {
                                start_id_from,
                                start_id_to,
                                size: _,
                            } = &entries[idx];

                            RawId::new(
                                start_id_to
                                    .into_inner()
                                    .checked_add(
                                        translated_idx
                                            .into_inner()
                                            .checked_sub(start_id_from.into_inner())
                                            .unwrap(),
                                    )
                                    .unwrap(),
                            )
                        })
                        .unwrap_or(translated_idx.clone())
                })
            })
            .min()
            .expect("no seeds specified");

        Id {
            value: lowest_location_from_seed_seen,
            space: spaces::Location,
        }
    }
}

#[derive(Debug)]
struct Map<S1, S2> {
    from_space: S1,
    to_space: S2,
    entries: Vec<MapEntry>,
}

impl<S1, S2> Map<S1, S2> {
    pub fn display_type(&self) -> impl Display + '_
    where
        S1: Display,
        S2: Display,
    {
        let Self {
            from_space,
            to_space,
            entries: _,
        } = self;
        lazy_format!("{from_space}-to-{to_space}")
    }
}

#[derive(Debug)]
struct MapEntry {
    start_id_from: RawId,
    start_id_to: RawId,
    size: NonZeroUsize,
}

const EXAMPLE: &str = include_str!("d5-example.txt");

#[test]
fn part_1_example() {
    let example_almanac_config = AlmanacConfig::new_part_1(EXAMPLE);
    assert_debug_snapshot!("part_1_parsed_example", example_almanac_config);

    assert_eq!(
        example_almanac_config.lowest_translated_seed_location(),
        Id {
            value: RawId::new(35),
            space: spaces::Location
        }
    );
}

const PUZZLE_INPUT: &str = include_str!("d5.txt");

#[test]
fn part_1() {
    assert_eq!(
        AlmanacConfig::new_part_1(PUZZLE_INPUT).lowest_translated_seed_location(),
        Id {
            value: RawId::new(836040384),
            space: spaces::Location,
        }
    )
}

#[test]
fn part_2_example() {
    let example_almanac_config = AlmanacConfig::new_part_2(EXAMPLE);
    assert_debug_snapshot!("part_2_parsed_example", example_almanac_config);

    assert_eq!(
        example_almanac_config.lowest_translated_seed_location(),
        Id {
            value: RawId::new(46),
            space: spaces::Location
        }
    );
}

#[test]
#[ignore]
fn part_2() {
    assert_eq!(
        AlmanacConfig::new_part_2(PUZZLE_INPUT).lowest_translated_seed_location(),
        Id {
            value: RawId::new(10834440),
            space: spaces::Location,
        }
    )
}
