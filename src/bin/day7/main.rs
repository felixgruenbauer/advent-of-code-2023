use std::{cmp::Ordering, collections::HashMap, ops::Deref, str::FromStr};

fn main() {
    let input = std::include_str!("input");
    let mut hands = parse_input(input);
    hands.iter_mut().for_each(|hand| hand.calc_htype2());
    hands.sort();
    for (idx, hand) in hands.iter_mut().enumerate() {
        hand.rank = Some(idx + 1);
    }
    let sum: usize = hands.iter().enumerate().map(|(idx, hand)| (idx + 1) * hand.bid).sum();

    for hand in hands.iter_mut() {
        println!("{:?}", hand);
    }
    
    println!("sum is {sum}");

}

#[derive(Debug, Eq)]
struct Hand {
    hand: Vec<Card>,
    rank: Option<usize>,
    bid: usize,
    htype: Htype,
    htype2: Option<Htype>
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.htype2.cmp(&other.htype2) {
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                let mut ord = Ordering::Equal;
                for (idx, card) in self.hand.iter().enumerate() {
                    let other_card = other.hand[idx] as usize;
                    if *card as usize > other_card {
                        ord = Ordering::Greater;
                        break;
                    };
                    if other_card > *card as usize {
                        ord = Ordering::Less;
                        break;
                    };
                }
                ord
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.htype == other.htype && self.hand == other.hand
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Htype {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    Pair = 2,
    HighCard = 1,

}

impl Hand {
    fn calc_htype2(&mut self) {
        let mut cards = self.hand.clone();
        cards.sort_by(| a, b| {
            let val_a = *a as usize;
            let val_b = *b as usize;
            val_a.cmp(&val_b)
        });

        let mut col = HashMap::new();
        for val in cards.iter() {
            col.entry(val).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut res = self.htype.clone();
        if !cards.contains(&Card::J) {self.htype2 = Some(res); return}
        //if *col.get(&Card::J).unwrap() > 1 {self.htype2 = self.htype; return}
        match self.htype {
            Htype::HighCard => {res = Htype::Pair},
            Htype::Pair => {res = Htype::ThreeOfAKind}
            Htype::TwoPair => {
                if *col.get(&Card::J).unwrap() == 2 {res = Htype::FourOfAKind}
                else {res = Htype::FullHouse}
            },
            Htype::ThreeOfAKind => {res = Htype::FourOfAKind},
            Htype::FullHouse => {res = Htype::FiveOfAKind},
            Htype::FourOfAKind => {res = Htype::FiveOfAKind},
            Htype::FiveOfAKind => {res = Htype::FiveOfAKind}
        }
        self.htype2 = Some(res);

    }
}

impl From<Vec<Card>> for Htype {
    fn from(value: Vec<Card>) -> Self {
        let mut cards = value;
        cards.sort_by(| a, b| {
            let val_a = *a as usize;
            let val_b = *b as usize;
            val_a.cmp(&val_b)
        });
        
        //if value.iter().reduce(|acc, v| v == acc) { return Htype::FiveOfAKind}
        let mut col = HashMap::new();
        for val in cards.iter() {
            col.entry(val).and_modify(|e| *e += 1).or_insert(1);
        }
        
        if col.iter().any(|(k, v)| *v == 4) { return Htype::FourOfAKind }
        if col.len() == 5 { return Htype::HighCard }
        if col.len() == 4 { return Htype::Pair }
        if col.len() == 2 { return Htype::FullHouse }
        if col.len() == 1 { return Htype::FiveOfAKind }
        if col.iter().any(|(k, v)| *v == 3) { return Htype::ThreeOfAKind }
        return Htype::TwoPair
        
        
    }
    
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 1,
    T = 10,
    N9 = 9,
    N8 = 8,
    N7 = 7,
    N6 = 6,  
    N5 = 5,
    N4 = 4,
    N3 = 3,
    N2 = 2,
}


#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;
impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            "9" => Ok(Card::N9),
            "8" => Ok(Card::N8),
            "7" => Ok(Card::N7),
            "6" => Ok(Card::N6),
            "5" => Ok(Card::N5),
            "4" => Ok(Card::N4),
            "3" => Ok(Card::N3),
            "2" => Ok(Card::N2),
            _ => Err(ParseCardError)
        }
        
    }
    
}


fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(|line| {
        let mut iter = line.split_ascii_whitespace();
        let hand = iter.next().unwrap().chars().map(|c| c.to_string().parse::<Card>().unwrap()).collect::<Vec<_>>();
        let htype = Htype::from(hand.clone());
        let bid = iter.next().unwrap().parse::<usize>().unwrap();
        Hand {
            hand,
            rank: None,
            bid,
            htype,
            htype2: None
        }
    }).collect()
}