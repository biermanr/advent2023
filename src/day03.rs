use std::fs;
use std::str;

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
        }
        part_numbers
    }
}

pub fn run_part1(data: &str){
    println!("{}",data);
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

