use std::fs;
use std::str;
use std::collections::{HashSet, HashMap};

const DIGITS: &[u8; 10] = b"0123456789";


struct Engine{
    diagram_height: usize,
    diagram_width: usize,
    diagram: Vec<String>,
}

impl Engine {
    fn new(lines: Vec<String>) -> Engine {
        Engine { 
            diagram_height: (&lines).len(),
            diagram_width: (&lines[0]).len(),
            diagram: lines,
        }
    }

    fn is_symbol(self: &Self, x: usize, y: usize) -> bool {
        if x >= self.diagram_width {
            false
        } else if y >= self.diagram_height {
            false
        } else {
            let c = self.diagram[y].bytes().nth(x).unwrap();
            if c == b'.' {
                false
            } else if DIGITS.contains(&c) {
                false
            } else {
                true
            }
        }
    }

    fn is_gear(self: &Self, x: usize, y: usize) -> bool {
        if x >= self.diagram_width {
            false
        } else if y >= self.diagram_height {
            false
        } else {
            let c = self.diagram[y].bytes().nth(x).unwrap();
            if c == b'*' {
                true
            } else {
                false
            }
        }
    }

    fn adj_symbol(self: &Self, x: usize, y: usize) -> bool {
        [
            if x > 0 { self.is_symbol(x-1, y  ) } else { false },   //check left
            self.is_symbol(x+1, y  ),                               //check right
            if y > 0 { self.is_symbol(  x, y-1) } else { false },   //check up
            self.is_symbol(  x, y+1),                               //check down
            if x*y > 0 { self.is_symbol(x-1, y-1) } else { false }, //check up-left
            if x > 0 { self.is_symbol(x-1, y+1) } else { false },   //check down-left 
            if y > 0 { self.is_symbol(x+1, y-1) } else { false },   //check up-right
            self.is_symbol(x+1, y+1),                               //check up-right
        ].into_iter().filter(|b| *b).count() > 0
    }

    fn get_adj_gears(self: &Self, x: usize, y: usize) -> HashSet<(usize,usize)> {
        let mut gears = HashSet::new();

        if x > 0 && self.is_gear(x-1, y  ) { 
            gears.insert((x-1,y));
        }
        if self.is_gear(x+1, y  ) {
            gears.insert((x+1, y));
        }
        if y > 0 && self.is_gear(  x, y-1) {
            gears.insert((x,y-1));
        }
        if self.is_gear(  x, y+1) {
            gears.insert((x,y+1));
        }
        if x*y > 0 && self.is_gear(x-1, y-1) {
            gears.insert((x-1,y-1));
        }
        if x > 0 && self.is_gear(x-1, y+1) {
            gears.insert((x-1,y+1));
        }
        if y > 0 && self.is_gear(x+1, y-1) {
            gears.insert((x+1,y-1));
        }
        if self.is_gear(x+1, y+1) {
            gears.insert((x+1, y+1));
        }

        gears
    }

    fn part_numbers(self: &Self) -> Vec<usize> {
        let mut part_numbers = vec![];
        for (y,line) in self.diagram.iter().enumerate() {
            let mut digits = vec![];
            let mut symbol_adj = false;

            for (x,c) in line.bytes().enumerate() {
                if DIGITS.contains(&c) {
                    digits.push(c);
                    symbol_adj = symbol_adj || self.adj_symbol(x,y);
                } else {
                    if digits.len() > 0 && symbol_adj {
                        let s = str::from_utf8(&digits).unwrap().parse().unwrap();
                        part_numbers.push(s);
                    }
                    digits = vec![];
                    symbol_adj = false;
                }
            }
            if digits.len() > 0 && symbol_adj {
                let s = str::from_utf8(&digits).unwrap().parse().unwrap();
                part_numbers.push(s);
            }
        }
        part_numbers
    }

    fn gear_prod_sums(self: &Self) -> usize {
        let mut gears: HashMap<(usize,usize),Vec<usize>> = HashMap::new();

        for (y,line) in self.diagram.iter().enumerate() {
            let mut digits = vec![];
            let mut adj_gears = HashSet::new();

            for (x,c) in line.bytes().enumerate() {
                if DIGITS.contains(&c) {
                    digits.push(c);
                    adj_gears = adj_gears.union(
                        &self.get_adj_gears(x,y)
                    ).copied().collect::<HashSet<_>>();
                } else {
                    if digits.len() > 0 {
                        let s = str::from_utf8(&digits).unwrap().parse().unwrap();
                        for gear in adj_gears {
                            if gears.contains_key(&gear) {
                                gears.get_mut(&gear).unwrap().push(s);
                            } else {
                                gears.insert(gear,vec![s]);
                            }
                        }
                    }
                    digits = vec![];
                    adj_gears = HashSet::new();
                }
            }

            //need to check once more at the end
            if digits.len() > 0 {
                let s = str::from_utf8(&digits).unwrap().parse().unwrap();
                for gear in adj_gears {
                    if gears.contains_key(&gear) {
                        gears.get_mut(&gear).unwrap().push(s);
                    } else {
                        gears.insert(gear,vec![s]);
                    }
                }
            }
        }

        let mut ret = 0;
        for (_gear,parts) in gears.into_iter() {
            if parts.len() == 2 {
                ret += parts[0]*parts[1];
            }
        }
        ret
    }
}

pub fn run_part1(data: &str){
    let lines: Vec<String> = fs::read_to_string(data)
        .unwrap()
        .lines()
        .map(|l| l.to_owned())
        .collect();

    let engine = Engine::new(lines);
    let part_numbers = engine.part_numbers();
    let part_sum: usize = part_numbers.iter().sum();
    println!("{}", part_sum);
}

pub fn run_part2(data: &str){
    let lines: Vec<String> = fs::read_to_string(data)
        .unwrap()
        .lines()
        .map(|l| l.to_owned())
        .collect();

    let engine = Engine::new(lines);
    let gear_sum = engine.gear_prod_sums();
    println!("{}", gear_sum);
}

