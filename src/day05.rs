use std::collections::HashMap;
use itertools::Itertools;
use std::fs;

fn apply_map(source_values: &Vec<usize>, m: &Vec<(usize,usize,usize)>) -> Vec<usize> {
    let mut dest_values = Vec::new();
    for v in source_values {
        let mut dest_value:usize = *v;
        for (s,d,r) in m {
            if v >= s && *v < s+r {
                dest_value = d+v-s;
            }
        }
        dest_values.push(dest_value);
    }
    dest_values
}

/// Day 5 problem 1
pub fn run_part1(data: &str) {
    //I don't know why I need this binding, something about borrowing
    let binding = fs::read_to_string(data).unwrap();
    let mut lines = binding.lines();

    //First line is always the seeds that need to be planted
    let seeds_line = lines.next().unwrap();
    let mut seeds: Vec<usize> = seeds_line
        .split(' ')
        .filter_map(|t| t.parse::<usize>().ok())
        .collect();

    //Next line will be empty, skip it
    lines.next().unwrap();

    let mut maps: HashMap<(&str,&str), Vec<(usize,usize,usize)>> = HashMap::new();

    //parse the maps
    while let Some(header_line) = lines.next() {

        let (dest,source): (&str,&str) = header_line
            .split(' ')
            .next()
            .unwrap()
            .split("-to-")
            .collect_tuple()
            .unwrap();

        let mut m: Vec<(usize,usize,usize)> = Vec::new();

        while let Some(map_line) = lines.next() {
            if map_line.len() == 0 { break }

            let (d,s,r): (usize,usize,usize) = map_line
                .split(' ')
                .map(|t| t.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            m.push((s,d,r));
        }
        maps.insert((dest,source), m);
    }

    //find route from seed_number to location_number
    //NOTE this works, but it's not very general
    //assumes that there aren't any loops in source --> dest map
    let mut map_order = Vec::new();
    let mut source = "seed";
    while source != "location" {
        for (s,d) in maps.keys() {
            if *s == source {
                map_order.push((s,d));
                source = d;
            }
        }
    }

    //Apply all the maps in the correct order
    for (s,d) in map_order {
        seeds = apply_map(&seeds, &maps[&(*s,*d)]);
    }
    println!("{:?}",seeds.iter().min().unwrap());
}

