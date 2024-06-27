use std::fs;
use std::collections::HashMap;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
enum HandKind {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct Hand {
    numeric_cards: [u8; 5],
    bid: usize,
    kind: HandKind,
}


impl Hand{
    fn new(cards: &str, bid: &str, jokers: bool) -> Hand {

        let numeric_cards: [u8; 5] = cards.chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => if jokers { 1 } else { 11 },
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                _ => 0,
            })
            .collect::<Vec<u8>>()
            .try_into()
            .expect("Number of cards must be 5");

        let bid = bid.parse::<usize>().expect("Bid non-numeric");

        let hand_kind = Self::classify_cards(cards, jokers);

        Hand{ 
            numeric_cards: numeric_cards,
            bid: bid,
            kind: hand_kind,
        }
    }

    fn classify_cards(cards: &str, jokers: bool) -> HandKind {
        let mut hm: HashMap::<char, u8> = HashMap::new();
        let mut num_jokers = 0;

        for c in cards.chars() {
            if jokers && c == 'J' {
                num_jokers += 1;
            } else {
                *hm.entry(c).or_insert(0) += 1;
            }
        }

        //There must be a better way to do this 
        let mut most_freq = 0;
        let mut second_most_freq = 0;
        for v in hm.values() {
            if *v > most_freq {
                second_most_freq = most_freq;
                most_freq = *v;
            } else if *v > second_most_freq {
                second_most_freq = *v;
            }
        }

        most_freq += num_jokers;

        let hand_kind = match (most_freq,second_most_freq) {
            (5,0) => Some(HandKind::FiveOfAKind),
            (4,1) => Some(HandKind::FourOfAKind),
            (3,2) => Some(HandKind::FullHouse),
            (3,1) => Some(HandKind::ThreeOfAKind),
            (2,2) => Some(HandKind::TwoPair),
            (2,1) => Some(HandKind::OnePair),
            (1,1) => Some(HandKind::HighCard),
            _ => None
        };

        hand_kind.unwrap()
    }
}


/// Day 7 problem 1
pub fn run_part1(data: &str) {

    let binding = fs::read_to_string(data).unwrap();
    let mut hands: Vec<Hand> = binding.lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(c,b)| Hand::new(c,b,false))
        .collect();


    hands.sort_by(|i,j| 
        i.kind.cmp(&j.kind)
        .then(i.numeric_cards.cmp(&j.numeric_cards))
    );

    let q: usize = hands.iter().enumerate().map(|(i,h)| (i+1)*h.bid).sum();
    println!("{:?}",q);

}

/// Day 7 problem 2
pub fn run_part2(data: &str) {

    let binding = fs::read_to_string(data).unwrap();
    let mut hands: Vec<Hand> = binding.lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(c,b)| Hand::new(c,b,true))
        .collect();


    hands.sort_by(|i,j| 
        i.kind.cmp(&j.kind)
        .then(i.numeric_cards.cmp(&j.numeric_cards))
    );

    let q: usize = hands.iter().enumerate().map(|(i,h)| (i+1)*h.bid).sum();
    println!("{:?}",q);

}
