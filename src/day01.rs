use std::collections::HashMap;
use std::fs;

/// Day 1 problem 1
///
/// Description:
/// Given an input file path as a &str (TODO change to use Path)
/// read the entire file into memory (TODO change to read line by line)
/// loop through the lines and filter out any non-numeric characters
/// then take the first and last numeric character and convert to an
/// int of two digits. Return the sum of all lines
pub fn run_part1(data: &str) {
    let mut s: u32 = 0;
    for line in fs::read_to_string(data).unwrap().lines() {
        let l: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        let str_num = format!("{}{}", l.first().unwrap(), l.last().unwrap());
        let num = str_num.parse::<u32>().unwrap();
        s += num;
    }
    println!("{}", s);
}

/// Day 1 problem 2
///
/// Description:
/// Same problem as before, except there are numbers spelled out
/// that we want to parse. The current approach is to replace all
/// instances of a spelled out letter with itself[#]itself because
/// the start and end of some spellings can overlap like eightwo. Then
/// after this replacement, the method is the same.
pub fn run_part2(data: &str) {
    let letter_to_num = HashMap::from([
        ("zero", "zero0zero"),
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ]);
    let mut s: u32 = 0;
    for line in fs::read_to_string(data).unwrap().lines() {
        let mut line = line.to_owned();
        for (k, v) in &letter_to_num {
            line = line.replace(k, v);
        }
        let l: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        let str_num = format!("{}{}", l.first().unwrap(), l.last().unwrap());
        let num = 10 * str_num;
        s += num;
    }
    println!("{}", s);
    println!("{}", data);
}
