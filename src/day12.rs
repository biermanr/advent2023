use std::fs;


/// Day 12 problem 1
pub fn run_part1(data: &str) {

    let binding = fs::read_to_string(data).unwrap();

    //this is very ugly, probably a better way
    let layouts: Vec<(&str, Vec<u32>)> = binding
        .lines()
        .filter_map(|l| l.split_once(' '))
        .filter_map(|(g,n)| 
            Some((g,n.split(',').map(
                    |n| n.parse::<u32>().unwrap()
                    ).collect::<Vec<u32>>())))
        .collect();

    let mut tot_num_ways = 0;
    for (l,c) in layouts {
        let num_ways = count_ways(l,&c);
        println!("{} {:?} {}",l,c,num_ways);
        tot_num_ways += num_ways;
        println!("");
    }

    println!("{:?}", tot_num_ways);
}

/// Day 12 problem 2
pub fn run_part2(data: &str) {

    let binding = fs::read_to_string(data).unwrap();

    //this is very ugly, probably a better way
    let layouts: Vec<(&str, Vec<u32>)> = binding
        .lines()
        .filter_map(|l| l.split_once(' '))
        .filter_map(|(g,n)| 
            Some((g,n.split(',').map(
                    |n| n.parse::<u32>().unwrap()
                    ).collect::<Vec<u32>>())))
        .collect();

    let mut tot_num_ways = 0;
    for (l,c) in layouts {
        //X5 both l and c
        let l = format!("{}?{}?{}?{}?{}", l, l, l, l, l);
        let c = c.iter().cycle().take(c.len() * 5).map(|&c| c).collect();

        let num_ways = count_ways(&l,&c);
        println!("{} {:?} {}",l,c,num_ways);
        tot_num_ways += num_ways;
        println!("");
    }

    println!("{:?}", tot_num_ways);
}

fn count_ways(layout: &str, contigs: &Vec<u32>) -> u32 {
    count_ways_recur(layout, contigs, 0, false, "")
}

///NEED TO FIGURE OUT A FASTER WAY, TOO SLOW FOR PART 2
fn count_ways_recur(layout: &str, contigs: &[u32], chain: u32, started: bool, build: &str) -> u32 {
    //if we've iterated through the whole layout
    //then return 1 if we've satisfied all contigs, otherwise 0
    let Some(spring) = layout.chars().next() else {
        if contigs.len() == 0 || (contigs.len() == 1 && chain == contigs[0]) {
            //println!("{build}");
            return 1;
        } else {
            return 0;
        }
    };

    //remove the current spring from the layout
    let layout = layout.chars().skip(1).collect::<String>();

    //if spring is '?' try both '.' and '#'
    if spring == '?' {
        let missing_layout = format!("{}{}", '.', layout);
        let missing_branch = count_ways_recur(&missing_layout, contigs, chain, started, build);

        let present_layout = format!("{}{}", '#', layout);
        let present_branch = count_ways_recur(&present_layout, contigs, chain, started, build);
        return missing_branch+present_branch;
    }

    let build = format!("{}{}", build, spring);

    if spring == '.' {
        //in-between contigs
        if !started {
            return count_ways_recur(&layout, contigs, chain, false, &build);
        }
        //no more contigs to compare against
        else if contigs.len() == 0 {
            return count_ways_recur(&layout, contigs, chain, false, &build);
        }
        //just "closed" a prior contig with correct number of springs
        else if chain == contigs[0] {
            return count_ways_recur(&layout, &contigs[1..], 0, false, &build);
        }
        //just "closed" a prior contig with incorrect number
        else {
            return 0;
        }
    }

    else if spring == '#' {
        //out of contigs or too many springs in a row
        if contigs.len() == 0 || chain > contigs[0] {
            return 0;
        }
        else {
            return count_ways_recur(&layout, contigs, chain+1, true, &build);
        }
    }

    //shouldn't get here
    else {
        return 0;
    }
}
