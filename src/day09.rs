use std::fs;


/// Day 9 problem 1
pub fn run_part1(data: &str) {

    //Read in the series of measurements
    let binding = fs::read_to_string(data).unwrap();
    let lines = binding.lines();

    let mut series = Vec::new();
    for line in lines {
        let s: Vec<isize> = line
            .split(" ")
            .filter_map(|c| Some(c.parse::<isize>().unwrap()))
            .collect();

        series.push(s);
    }

    let answer: isize = series.iter().map(|s| recur_forward(s)).sum();
    println!("{:?}",answer);
}

fn recur_forward(s: &Vec<isize>) -> isize {
    match s.iter().sum() {
        0 => 0,
        _ => {
            let n = s.last().unwrap();
            let s: Vec<isize> = s[..s.len()-1].iter()
                .zip(s[1..].iter())
                .map(|(l,r)| r-l)
                .collect();

            n+recur_forward(&s)
        },
    }
}

/// Day 9 problem 2
pub fn run_part2(data: &str) {

    //Read in the series of measurements
    let binding = fs::read_to_string(data).unwrap();
    let lines = binding.lines();

    let mut series = Vec::new();
    for line in lines {
        let s: Vec<isize> = line
            .split(" ")
            .filter_map(|c| Some(c.parse::<isize>().unwrap()))
            .collect();

        series.push(s);
    }

    let answer: isize = series.iter().map(|s| recur_backward(s)).sum();
    println!("{:?}",answer);
}

fn recur_backward(s: &Vec<isize>) -> isize {
    match s.iter().sum() {
        0 => 0,
        _ => {
            let n = s.first().unwrap();
            let s: Vec<isize> = s[..s.len()-1].iter()
                .zip(s[1..].iter())
                .map(|(l,r)| r-l)
                .collect();

            n-recur_backward(&s)
        },
    }
}
