use arrayvec::ArrayVec;
use insta::assert_debug_snapshot;

const EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CamelCard {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand([CamelCard; Self::HAND_SIZE]);

impl Hand {
    const HAND_SIZE: usize = 5;

    fn trick(&self) -> Trick {
        let Self(inner) = self;

        let count_by_card = {
            let mut sorted = inner.clone();
            sorted.sort();
            let mut counts =
                sorted
                    .into_iter()
                    .fold(ArrayVec::<(CamelCard, u8), 5>::new(), |mut acc, card| {
                        if let Some((_card, count)) = acc
                            .iter_mut()
                            .filter(|(this_card, _)| &card == this_card)
                            .next()
                        {
                            *count += 1;
                        } else {
                            acc.push((card, 1));
                        }
                        acc
                    });
            counts.sort_by(|(card_1, count_1), (card_2, count_2)| {
                count_1.cmp(count_2).then(card_1.cmp(card_2))
            });
            counts
        };

        match count_by_card.as_slice() {
            &[(_, 5)] => Trick::FiveOfAKind,
            &[.., (_, 4)] => Trick::FourOfAKind,
            &[(_, 2), (_, 3)] => Trick::FullHouse,
            &[.., (_, 3)] => Trick::ThreeOfAKind,
            &[.., (_, 2), (_, 2)] => Trick::TwoPair,
            &[.., (_, 2)] => Trick::OnePair,
            &[.., (_, 1)] => Trick::HighCard,
            _ => panic!("lolwat"),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.trick().cmp(&other.trick()).then({
            let Self(this) = self;
            let Self(other) = other;
            this.cmp(other)
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Trick {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Play {
    hand: Hand,
    bid: u32,
}

impl Play {
    fn part_1(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();

                let hand = hand
                    .chars()
                    .map(|card| match card {
                        'A' => CamelCard::A,
                        'K' => CamelCard::K,
                        'Q' => CamelCard::Q,
                        'J' => CamelCard::J,
                        'T' => CamelCard::T,
                        '9' => CamelCard::N9,
                        '8' => CamelCard::N8,
                        '7' => CamelCard::N7,
                        '6' => CamelCard::N6,
                        '5' => CamelCard::N5,
                        '4' => CamelCard::N4,
                        '3' => CamelCard::N3,
                        '2' => CamelCard::N2,
                        _ => panic!("ohpoop bad card: found character {card:?}"),
                    })
                    .collect::<ArrayVec<CamelCard, 5>>()
                    .into_inner()
                    .map(Hand)
                    .unwrap();

                let bid = bid.parse::<u32>().unwrap();

                Play { hand, bid }
            })
            .collect::<Vec<_>>()
    }
}

fn part_1_total_winnings(plays: &mut [Play]) -> u64 {
    plays.sort_by_key(|p| p.hand.clone());
    plays
        .iter()
        .zip(1u64..)
        .fold(0, |acc, (play, rank)| acc + u64::from(play.bid) * rank)
}

#[test]
fn part_1_example() {
    let mut plays = Play::part_1(EXAMPLE);
    let winnings = part_1_total_winnings(&mut plays);
    assert_debug_snapshot!(plays);
    assert_eq!(winnings, 6440);
}

const PUZZLE_INPUT: &str = include_str!("d7.txt");

#[test]
fn part_1() {
    assert_eq!(
        part_1_total_winnings(&mut Play::part_1(PUZZLE_INPUT)),
        248105065
    );
}
