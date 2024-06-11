use std::fs;
use itertools::Itertools;
use std::collections::HashSet;

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

