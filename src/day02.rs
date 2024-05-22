use std::fs;
use regex::Regex;

const MAX_RED: u16 = 12;
const MAX_GREEN: u16 = 13;
const MAX_BLUE: u16 = 14;

/// Day 2 problem 1
///
/// Description:
/// This problem is really just string parsing of multiple "games"
/// we can use enumerate() to avoid parsing the game number
/// then we use take_while() to iterate over the separate games
/// similarly we need to iterate over the colors in each game.
///
/// Ugliest code in the world. I tried to avoid using .split()
/// ended up struggling with iterator next and take_while/skip_while
/// consuming the last position I needed. Ended up making it peekable
/// which doesn't feel like a good solution either
pub fn run_part1(data: &str) {
    let mut ret = 0;
    for (game_num, line) in fs::read_to_string(data).unwrap().lines().enumerate() {
        let mut passing = true;
        let mut s = line.chars().skip_while(|&c| c != ':'); //skip past "Game *:"
        while s.next().is_some() {
            let mut draw = s.by_ref().take_while(|&c| c != ';').peekable();

            while draw.peek().is_some() {
                if let Some(c) = draw.peek() {
                    if c == &' ' {
                        draw.next();
                    }
                }
                let num_color: String = draw.by_ref().take_while(|&c| c != ' ').collect();

                let num_color: u16 = num_color.parse().unwrap();

                //skip to the next ball color
                let color: String = draw.by_ref().take_while(|&c| c != ',').collect();

                match color.as_ref() {
                    "red" => {
                        if num_color > MAX_RED {
                            passing = false
                        }
                    }
                    "green" => {
                        if num_color > MAX_GREEN {
                            passing = false
                        }
                    }
                    "blue" => {
                        if num_color > MAX_BLUE {
                            passing = false
                        }
                    }
                    &_ => println!("UNKNOWN COLOR!"),
                }
            }
        }
        if passing {
            ret += game_num + 1;
        }
    }
    println!("{}", ret);
}

/// Day 2 problem 2
///
/// Description:
/// Same problem, but using regex instead of trying to only
/// go through the line once and it is SO MUCH EASIER!
/// The goal is to determine the minimum number of balls of each
/// color needed for each game, which is just the max.
pub fn run_part2(data: &str) {
    let mut ret = 0;

    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();

    for line in fs::read_to_string(data).unwrap().lines(){
        let max_red:usize = re_red
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_,[v])| v.parse().unwrap())
            .max().unwrap();
        
        let max_green:usize = re_green
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_,[v])| v.parse().unwrap())
            .max().unwrap();

        let max_blue:usize = re_blue
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_,[v])| v.parse().unwrap())
            .max().unwrap();

        ret += max_red*max_green*max_blue;
    }
    println!("{}",ret);
}
