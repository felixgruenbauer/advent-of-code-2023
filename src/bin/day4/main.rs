use std::fmt::Display;

fn main() {
    let input = std::include_str!("input");
    let cards = parse_input(input);
    for card in cards.iter() { println!("{card}") };
    let sum1 = cards.iter().fold(0, |acc, card| acc + card.score);
    println!("part one: sum is {sum1}");
    let sum2 = count_cards(cards);
    println!("part two: sum is {sum2}");
}

#[derive(Debug, Default)]
struct Card {
    id: usize,
    win_num: Vec<usize>,
    my_num: Vec<usize>,
    score: usize,
    matches: usize
}

fn count_cards(cards: Vec<Card>) -> usize {

    let mut res = vec![1usize; 189];
    
    for (card_idx, card) in cards.iter().enumerate() {
        let instances = res[card_idx];
        for i in card_idx+1..=card_idx+card.matches {
            if i > 188 { break; }
            res[i] += instances;
        }
    }
    
    res.iter().sum()
    
}

impl Card {
    fn new(id: usize, win_num: Vec<usize>, my_num: Vec<usize>) -> Self {
        let mut score = 0usize;
        let count = my_num.iter().filter(|n| win_num.contains(n)).count();
        if count > 0 {
            score = usize::pow(2, (count - 1) as u32);
        }
        Card {id, win_num, my_num, score, matches: count}
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {}: {:?} || {:?} => {}", self.id, self.win_num, self.my_num, self.score)
    }
    
}

fn parse_input(s: &str) -> Vec<Card> {
    let mut res: Vec<Card> = vec![];
    
    for (line_id, line) in s.lines().enumerate() {

        let lists: Vec<_> = line.split(":").last().unwrap().split("|").map(|list| list.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect();
        res.push(Card::new(line_id+1, lists[0].to_owned(), lists[1].to_owned()));
    }
    
    res
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
