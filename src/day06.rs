use std::fs;


/// Day 6 problem 1
pub fn run_part1(data: &str) {

    let binding = fs::read_to_string(data).unwrap();
    let mut lines = binding.lines();

    let times: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|t| t.parse::<usize>().ok())
        .collect();

    let distances: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|t| t.parse::<usize>().ok())
        .collect();

    let mut ways_to_win = 1;
    for (t,d) in times.iter().zip(distances.iter()) {
        let r_min = ((*t as f64)-((t*t-4*d) as f64).sqrt())/2.0;
        let r_max = ((*t as f64)+((t*t-4*d) as f64).sqrt())/2.0;

        let mut r_min = (r_min.ceil()) as usize;
        if r_min*(t-r_min) == *d {
            r_min += 1;
        }

        let mut r_max = (r_max.floor()) as usize;
        if r_max*(t-r_max) == *d {
            r_max -= 1;
        }

        ways_to_win *= r_max-r_min+1;
    }
    println!("{:?}",ways_to_win);
}


/// Day 6 problem 2
pub fn run_part2(data: &str) {

    let binding = fs::read_to_string(data).unwrap();
    let mut lines = binding.lines();

    let t: usize = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let d: usize = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let r_min = ((t as f64)-((t*t-4*d) as f64).sqrt())/2.0;
    let r_max = ((t as f64)+((t*t-4*d) as f64).sqrt())/2.0;

    let mut r_min = (r_min.ceil()) as usize;
    if r_min*(t-r_min) == d {
        r_min += 1;
    }

    let mut r_max = (r_max.floor()) as usize;
    if r_max*(t-r_max) == d {
        r_max -= 1;
    }

    let ways_to_win = r_max-r_min+1;
    println!("{:?}",ways_to_win);
}
