use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash)]
enum Label {
    A,
    K,
    Q,
    J,
    T,
    Num(u16),
    None,
}

impl Label {
    fn to_u16(&self) -> u16 {
        match self {
            Self::A => 15,
            Self::K => 14,
            Self::Q => 13,
            Self::T => 10,
            Self::J => 1,
            Self::Num(x) => *x,
            Self::None => 0,
        }
    }
}

impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        self.to_u16() == other.to_u16() 
    }
}

impl Eq for Label {}

impl Ord for Label {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_u16().cmp(&other.to_u16())
    }
}

impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
enum Type {
    Five(Label),
    Four(Label),
    FH(Label, Label), // (2, 3)
    Three(Label),
    TwoPair(Label, Label),
    OnePair(Label),
    High(Label)
}

impl Type {
    fn to_u16(&self) -> u16 {
        match self {
            Self::Five(_) => 6,
            Self::Four(_) => 5,
            Self::FH(_, _) => 4,
            Self::Three(_) => 3,
            Self::TwoPair(_, _) => 2,
            Self::OnePair(_) => 1,
            Self::High(_) => 0,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.to_u16() == other.to_u16()
    }
}

impl Eq for Type {}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Label; 5],
    _type: Type,
}

struct HandBuilder {
    cards: [Label; 5],
    _n: u16,
}

impl Hand {
    fn new() -> HandBuilder {
        HandBuilder{cards: [Label::None; 5], _n: 0}
    }

    fn get_type(&self) -> Type {
        self._type
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_type() == other.get_type() {
            for (c_self, c_other) in self.cards.iter().zip(other.cards) {
                if *c_self != c_other {
                    return c_self.cmp(&c_other);
                }
            }
        }
        self.get_type().to_u16().cmp(&other.get_type().to_u16())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_card(card: &Label, cards: &HashMap<Label, u16>) -> Option<Type> {
    let jokers = match card {
        Label::J => 0,
        _ => match cards.get(&Label::J) {
            Some(x) => *x,
            None => 0,
        },
    };

    if cards[card] + jokers >= 5 {
        return Some(Type::Five(*card));
    } 
    if cards[card] + jokers >= 4 {
        return Some(Type::Four(*card));
    }
    if cards[card] + jokers >= 3 {
        for other in cards.keys() {
            if *other == *card {
                continue;
            }

            if *other == Label::J {
                if jokers + cards[card] - 3 >= 2 {
                    return Some(Type::FH(*other, *card));
                }
            } else if cards[other] + jokers + cards[card] - 3 >= 2 {
                return Some(Type::FH(*other, *card));
            }
        }
        return Some(Type::Three(*card));
    }
    if cards[card] + jokers >= 2 {
        for other in cards.keys() {
            if *other == *card {
                continue;
            }

            if *other == Label::J {
                if jokers + cards[card] - 2 >= 3 {
                    return Some(Type::FH(*card, *other));
                }
            } else if cards[other] + jokers + cards[card] - 2 >= 3 {
                return Some(Type::FH(*card, *other));
            }
        }
    }
    if cards[card] + jokers >= 2 {
        for other in cards.keys() {
            if *other == *card {
                continue;
            }

            if *other == Label::J {
                if jokers + cards[card] - 2 >= 2 {
                    return Some(Type::TwoPair(*other, *card));
                }
            } else if cards[other] + jokers + cards[card] - 2 == 2 {
                return Some(Type::TwoPair(*other, *card));
            }
        }
        return Some(Type::OnePair(*card));
    }

    None
}

impl Type {
    fn from(cards: &[Label]) -> Self {
        let mut map: HashMap<Label, u16> = HashMap::new();

        for card in cards {
            if map.contains_key(card) {
                map.insert(*card, map[card] + 1);
            } else {
                map.insert(*card, 1);
            }
        }

        let mut best: Option<Type> = None;

        for card in map.keys() {
            if let Some(c) = parse_card(card, &map) {
                match best {
                    Some(_type) => {
                        if c.to_u16() > _type.to_u16() {
                            best = Some(c);
                        }
                    },
                    None => {best = Some(c);}
                }
            }
        }

        match best {
            Some(_type) => _type,
            None => Type::High(map.keys()
                    .fold(Label::None, |a, b| Label::max(a, *b)))
        }
    }
}


impl HandBuilder {
    fn add_card(mut self, card: Label) -> Self {
        self.cards[self._n as usize] = card;
        self._n += 1;
        self
    }

    fn build(self) -> Result<Hand, String> {
        if self._n != 5 {
            return Err(format!("A hand should have 5 cards. Found {}", self._n));
        }
        let _type = Type::from(&self.cards);
        Ok(Hand{cards: self.cards, _type: _type})
    }
}

impl Label {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            x => {
                if let Some(n) = x.to_digit(10) {
                    Self::Num(n as u16)
                } else {
                    Self::None
                }
            },
        } 
    }
}

pub fn part2(prompt: String) -> u64 {
    let mut hands = prompt.lines()
        .map(|line| line.split_whitespace())
        .map(|split| (split.clone().nth(0).unwrap(), split.clone().nth(1).unwrap()))
        .map(|(cards, bet)| (cards
                .chars()
                .map(|c| Label::from(c))
                .fold(Hand::new(), |hand, card| hand.add_card(card))
                .build().unwrap()
            , bet.parse::<u64>().unwrap()))
        .collect::<Vec<(Hand, u64)>>();

    hands.sort_by(|(a, _), (b, _)| a.cmp(&b));

    hands.iter()
        .enumerate()
        .map(|(i, (_, bet))| (i+1) as u64 * bet)
        .sum()
}
