use std::fs;
use std::collections::HashMap;


/// Day 8 problem 1
pub fn run_part1(data: &str) {

    let binding = fs::read_to_string(data).unwrap();
    let mut lines = binding.lines();
    let directions = lines.next().unwrap();
    lines.next(); //skip blank line

    let nodes: HashMap<String, (String, String)> = lines
        .map(|l| l.split_once(" = (").unwrap())
        .map(|(n,lr)| (n,lr.split_once(", ").unwrap()))
        .map(|(n,(l,r))| (n.to_owned(),(l.to_owned(),r.replace(")",""))))
        .collect();

    let mut node = "AAA";
    let mut num_steps = 0;
    'outer: loop {
        for d in directions.chars() {
            num_steps += 1;
            if d == 'L' {
                node = &nodes[node].0;
            } else {
                node = &nodes[node].1;
            }
            
            if node == "ZZZ" {
                break 'outer;
            }
        }
    }
    println!("{:?}",num_steps);

}


/// Day 8 problem 2
pub fn run_part2(data: &str) {

    let binding = fs::read_to_string(data).unwrap();
    let mut lines = binding.lines();
    let directions = lines.next().unwrap();
    lines.next(); //skip blank line

    let nodes: HashMap<&str, (&str, &str)> = lines
        .map(|l| l.split_once(" = (").unwrap())
        .map(|(n,lr)| (n,lr.split_once(", ").unwrap()))
        .map(|(n,(l,r))| (n,(l,&r[0..r.len()-1])))
        .collect();

    //start at all nodes that end with **A
    let mut curr_nodes: Vec<&str> = nodes
        .keys()
        .filter(|n| n.chars().last() == Some('A'))
        .map(|&n| n) //ugly, not sure how to avoid this
        .collect();

    let mut num_steps = 0;

    let mut finished_cycles = 0;
    let mut cycle_lens: HashMap<usize, usize> = HashMap::new();

    'outer: loop {
        for d in directions.chars() {
            num_steps += 1;
            curr_nodes = curr_nodes
                .iter()
                .map(|n| if d == 'L'  { nodes[n].0 } else { nodes[n].1 })
                .collect();

            for (i,n) in curr_nodes.iter().enumerate() {
                if n.chars().last() == Some('Z') {
                    cycle_lens.entry(i).or_insert(num_steps);
                    finished_cycles += 1;
                    println!("Cycle {:?} took {:?} steps to finish",i,num_steps);
                }
            }

            if finished_cycles == curr_nodes.len() {
                break 'outer;
            }
        }
    }

    let answer = lcm(cycle_lens.into_values().collect());
    println!("{:?}",answer);

}

// STOLEN FROM https://www.andyloree.com/blog/2022/12/11/least-common-multiple-vect-rust/
/// Least common multiple vec of numbers
fn lcm(numbers: Vec<usize>) -> usize {
    let mut temp = numbers.clone();
    
    // check all the same
    loop {
        let mut same = true;

        for idx in 1..temp.len() {
            if temp[0] != temp[idx] {
                same = false;
                break;
            }
        }

        if same {
            return temp[0];
        }

        // Find lowest index
        match temp.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b)).map(|(index, _)| index) {
            Some(idx) => {
                temp[idx] = temp[idx] + numbers[idx];
            },
            None => panic!("Not possible")
        }
    }
}

