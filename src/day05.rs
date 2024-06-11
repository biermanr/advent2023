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

fn apply_map_range(source_values: &Vec<(usize,usize)>, m: &Vec<(usize,usize,usize)>) -> Vec<(usize,usize)> {
    let mut dest_ranges:Vec<(usize,usize)> = Vec::new();
    for (s_start,s_span) in source_values {
        let mut s_start = *s_start;
        let s_end = s_start+s_span;

        for (m_start,d,r) in m {
            let m_start = *m_start;
            let m_end = m_start+r;
           
            //if the start of the next region is too large
            //then no point in checking further regions
            //SOURCE:       ----------
            //   MAP:                   -------- 
            if s_end < m_start {
                dest_ranges.push((s_start,*s_span));
                s_start = s_end;
                break;
            }

            //if the source start is greater than the map end
            //then move on to the next map
            //SOURCE:                ----------
            //   MAP:      -------- 
            if s_start > m_end {
                continue;
            }

            //case when overlap with "dangling" left-side source
            //might continue past the end of the map region
            //SOURCE:      ---------????
            //   MAP:          -------- 
            if s_start < m_start {
                dest_ranges.push((s_start,m_start-s_start));
                s_start = m_start;
            }

            //case when source end is within the map end
            //after doing the mapping don't have to check
            //the next mapping region since it will be too large
            //SOURCE:          ----
            //   MAP:          -------- 
            if s_end < m_end {
                dest_ranges.push((d+(s_start-m_start),s_end-s_start));
                s_start = s_end;
                break;
            }
            
            //only way to end up here is if source continues
            //past the end of the map
            //SOURCE:          ???--------
            //   MAP:          --------
            dest_ranges.push((d+(s_start-m_start),m_end-s_start));
            s_start = m_end;
        }

        //have to check final case if SOURCE is larger than all maps
        //which I'm denoting with s_start < s_end
        if s_start < s_end {
            dest_ranges.push((s_start,s_end-s_start));
        }
    }
    dest_ranges
}

///Day 5 problem 2
pub fn run_part2(data: &str) {
    //I don't know why I need this binding, something about borrowing
    let binding = fs::read_to_string(data).unwrap();
    let mut lines = binding.lines();

    //First line is now pairs of ranges of seeds
    let seeds_line = lines.next().unwrap();
    let mut seeds: Vec<(usize,usize)> = seeds_line
        .split(' ')
        .filter_map(|t| t.parse::<usize>().ok())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|x| (x[0],x[1]))
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
        m.sort_by(|a,b| (a.0).cmp(&b.0));
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
        seeds = apply_map_range(&seeds, &maps[&(*s,*d)]);
    }
    println!("{:?}",seeds.iter().min().unwrap());
}

