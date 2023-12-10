use insta::assert_debug_snapshot;

#[derive(Debug)]
pub(crate) struct ScratchCard {
    winning_nums: Vec<u8>,
    received_nums: Vec<u8>,
}

impl ScratchCard {
    pub fn new(line: &str) -> Self {
        let mut line = line;
        line = line.strip_prefix("Card").unwrap();
        line = line.trim_start_matches(' ');
        line = line.trim_start_matches(|c: char| c.is_ascii_digit());
        line = line.strip_prefix(": ").unwrap();
        let (winning_nums, received_nums) = line.split_once(" | ").unwrap();
        let winning_nums = winning_nums
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        let received_nums = received_nums
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        Self {
            winning_nums,
            received_nums,
        }
    }

    pub fn points(&self) -> u64 {
        if let Some(exponent) = u32::from(self.num_winning_nums_received()).checked_sub(1) {
            2u64.pow(exponent)
        } else {
            0
        }
    }

    pub fn num_winning_nums_received(&self) -> u16 {
        let Self {
            winning_nums,
            received_nums,
        } = self;
        received_nums
            .iter()
            .copied()
            .filter(|n| winning_nums.contains(n))
            .count()
            .try_into()
            .unwrap()
    }
}

const EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

#[test]
fn part_1_example() {
    let scratch_cards = EXAMPLE.lines().map(ScratchCard::new).collect::<Vec<_>>();
    assert_debug_snapshot!("parsed_example_part_1", scratch_cards);

    let points = scratch_cards
        .iter()
        .fold(0u64, |points, card| points + card.points());
    assert_eq!(points, 13);
}

const PUZZLE_INPUT: &str = include_str!("d4.txt");

#[test]
fn part_1() {
    assert_eq!(
        PUZZLE_INPUT
            .lines()
            .map(ScratchCard::new)
            .fold(0u64, |points, card| points + card.points()),
        21919
    )
}

fn calculate_scratch_cards_won(input: &str) -> u64 {
    let mut card_multipliers = Vec::<u64>::new();
    input
        .lines()
        .map(ScratchCard::new)
        .collect::<Vec<_>>()
        .iter()
        .fold(0u64, |cards_won, card| {
            let card_multiplier = (!card_multipliers.is_empty())
                .then(|| card_multipliers.remove(0))
                .unwrap_or(1);
            let new_cards_won = cards_won + card_multiplier;
            let num_winning_nums_received = usize::from(card.num_winning_nums_received());
            card_multipliers.resize(num_winning_nums_received.max(card_multipliers.len()), 1);
            card_multipliers
                .iter_mut()
                .take(num_winning_nums_received)
                .for_each(|mult| *mult += card_multiplier);
            new_cards_won
        })
}

#[test]
fn part_2_example() {
    assert_eq!(calculate_scratch_cards_won(EXAMPLE), 30);
}

#[test]
fn part_2() {
    assert_eq!(calculate_scratch_cards_won(PUZZLE_INPUT), 9881048);
}
