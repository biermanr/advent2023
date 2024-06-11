use std::fs;
use itertools::Itertools;
use std::collections::{HashSet,HashMap};

pub fn run_part1(data: &str) {
    let mut total_winnings = 0;
    for line in fs::read_to_string(data).unwrap().lines() {
        let l = line.splitn(2, ": ").last().unwrap();
        let (nums,win_nums) = l.splitn(2, " | ").collect_tuple().unwrap();

        let nums: HashSet<usize> = nums
            .trim()
            .split_whitespace()
            .filter_map(|c| Some(c.parse::<usize>().unwrap()))
            .collect();

        let win_nums: HashSet<usize> = win_nums
            .trim()
            .split_whitespace()
            .filter_map(|c| Some(c.parse::<usize>().unwrap()))
            .collect();

        let num_wins = nums.intersection(&win_nums).count();
    
        //this is ugly, struglling with raising to a power
        if num_wins > 0 {
            let mut card_winnings = 1;
            for _ in 1..num_wins {
                card_winnings *= 2;
            }
            total_winnings += card_winnings;
        }
    }
    println!("{:?}",total_winnings);
}

pub fn run_part2(data: &str) {
    let mut wins_per_card: HashMap<usize, usize> = HashMap::new();

    for line in fs::read_to_string(data).unwrap().lines() {
        let (card_num, l) = line
            .splitn(2, ": ")
            .collect_tuple()
            .unwrap();

        let card_num: usize = card_num
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        *wins_per_card.entry(card_num).or_insert(0) += 1;

        let (nums,win_nums) = l.splitn(2, " | ").collect_tuple().unwrap();

        let nums: HashSet<u32> = nums
            .trim()
            .split_whitespace()
            .filter_map(|c| Some(c.parse::<u32>().unwrap()))
            .collect();

        let win_nums: HashSet<u32> = win_nums
            .trim()
            .split_whitespace()
            .filter_map(|c| Some(c.parse::<u32>().unwrap()))
            .collect();

        let num_wins = nums.intersection(&win_nums).count();

        for i in 1..(num_wins+1) {
            *wins_per_card.entry(card_num+i).or_insert(0) += wins_per_card[&card_num];
        }
    }

    let total_wins: usize = wins_per_card.values().sum();
    println!("{}",total_wins);
}
