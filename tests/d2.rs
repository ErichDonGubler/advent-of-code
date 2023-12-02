#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Game {
    pub(crate) id: u32,
    pub(crate) rounds: Vec<Round>,
}

impl Game {
    pub fn new(s: &str) -> Self {
        let rest = s;
        let (id, rest) = rest
            .strip_prefix("Game ")
            .unwrap()
            .split_once(": ")
            .unwrap();
        let id = id.parse().unwrap();
        let rounds = rest
            .split("; ")
            .map(|round| {
                let mut red = None;
                let mut green = None;
                let mut blue = None;
                for dice in round.split(", ") {
                    let (count, color) = dice.split_once(' ').unwrap();
                    let count = count.parse::<u8>().unwrap();
                    let prop = match color {
                        "red" => &mut red,
                        "blue" => &mut blue,
                        "green" => &mut green,
                        _ => panic!("bad color {color:?}"),
                    };
                    let None = prop.replace(count) else {
                        panic!("duplicate value for {color:?}");
                    };
                }
                let red = red.unwrap_or_default();
                let blue = blue.unwrap_or_default();
                let green = green.unwrap_or_default();
                Round { red, blue, green }
            })
            .collect();
        Game { id, rounds }
    }

    pub fn is_possible(&self, possible_reds: u8, possible_blues: u8, possible_greens: u8) -> bool {
        let Self { id: _, rounds } = self;
        for round in rounds {
            let &Round { red, blue, green } = round;
            if red > possible_reds || blue > possible_blues || green > possible_greens {
                return false;
            }
        }
        true
    }

    pub fn minimum_cubes(&self) -> (u8, u8, u8) {
        let Self { id: _, rounds } = self;

        let mut needed_reds = 0;
        let mut needed_blues = 0;
        let mut needed_greens = 0;

        for round in rounds {
            let Round { red, blue, green } = round;
            needed_reds = needed_reds.max(*red);
            needed_blues = needed_blues.max(*blue);
            needed_greens = needed_greens.max(*green);
        }

        (needed_reds, needed_blues, needed_greens)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Round {
    pub(crate) red: u8,
    pub(crate) blue: u8,
    pub(crate) green: u8,
}

const EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

#[test]
fn part_1_example() {
    let games = EXAMPLE.lines().map(Game::new).collect::<Vec<_>>();

    assert_eq!(
        games,
        vec![
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        red: 4,
                        blue: 3,
                        green: 0
                    },
                    Round {
                        red: 1,
                        blue: 6,
                        green: 2
                    },
                    Round {
                        red: 0,
                        blue: 0,
                        green: 2
                    }
                ]
            },
            Game {
                id: 2,
                rounds: vec![
                    Round {
                        red: 0,
                        blue: 1,
                        green: 2
                    },
                    Round {
                        red: 1,
                        blue: 4,
                        green: 3
                    },
                    Round {
                        red: 0,
                        blue: 1,
                        green: 1
                    }
                ]
            },
            Game {
                id: 3,
                rounds: vec![
                    Round {
                        red: 20,
                        blue: 6,
                        green: 8
                    },
                    Round {
                        red: 4,
                        blue: 5,
                        green: 13
                    },
                    Round {
                        red: 1,
                        blue: 0,
                        green: 5
                    }
                ]
            },
            Game {
                id: 4,
                rounds: vec![
                    Round {
                        red: 3,
                        blue: 6,
                        green: 1
                    },
                    Round {
                        red: 6,
                        blue: 0,
                        green: 3
                    },
                    Round {
                        red: 14,
                        blue: 15,
                        green: 3
                    }
                ]
            },
            Game {
                id: 5,
                rounds: vec![
                    Round {
                        red: 6,
                        blue: 1,
                        green: 3
                    },
                    Round {
                        red: 1,
                        blue: 2,
                        green: 2
                    }
                ]
            }
        ]
    );

    let possible_reds = 12;
    let possible_blues = 14;
    let possible_greens = 13;
    let possible_rounds = games
        .iter()
        .filter(|game| game.is_possible(possible_reds, possible_blues, possible_greens))
        .map(|game| game.id)
        .collect::<Vec<_>>();

    assert_eq!(possible_rounds, vec![1, 2, 5]);

    assert_eq!(possible_rounds.iter().sum::<u32>(), 8);
}

const INPUT: &str = include_str!("./d2.txt");

#[test]
fn part_1() {
    let games = INPUT.lines().map(Game::new).collect::<Vec<_>>();

    let possible_reds = 12;
    let possible_blues = 14;
    let possible_greens = 13;
    let possible_rounds = games
        .iter()
        .filter(|game| game.is_possible(possible_reds, possible_blues, possible_greens))
        .map(|game| game.id);

    assert_eq!(possible_rounds.sum::<u32>(), 2331);
}

#[test]
fn part_2_example() {
    let games = EXAMPLE.lines().map(Game::new).collect::<Vec<_>>();

    let minimum_cubes_per_game = games.iter().map(Game::minimum_cubes).collect::<Vec<_>>();
    assert_eq!(
        minimum_cubes_per_game,
        vec![(4, 6, 2), (1, 4, 3), (20, 6, 13), (14, 15, 3), (6, 2, 3)]
    );

    assert_eq!(
        minimum_cubes_per_game
            .iter()
            .cloned()
            .map(|(r, b, g)| u64::from(r) * u64::from(b) * u64::from(g))
            .sum::<u64>(),
        2286
    )
}

#[test]
fn part_2() {
    let games = INPUT.lines().map(Game::new).collect::<Vec<_>>();

    assert_eq!(
        games
            .iter()
            .map(Game::minimum_cubes)
            .map(|(r, b, g)| u64::from(r) * u64::from(b) * u64::from(g))
            .sum::<u64>(),
        71585
    )
}
