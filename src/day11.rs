use std::fs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

/// Day 11 problem 1
pub fn run_part1(data: &str) {

    //Create a grid: Vec<Vec<char>> from the input file
    let binding = fs::read_to_string(data).unwrap();
    let lines = binding.lines();
    let grid: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();

    println!("INITIAL GRID");
    print_grid(&grid);

    //Expand the grid rows, transpose, expand rows, transpose back
    let grid = expand_rows(grid);
    let grid = transpose_grid(grid);
    let grid = expand_rows(grid);
    let grid = transpose_grid(grid);

    println!("EXPANDED GRID");
    print_grid(&grid);

    //Get the locations of all galaxies
    let galaxies = find_galaxies(&grid);
    println!("{:?}",galaxies);

    //Sum the manhattan distance for all pairs of galaxies
    let mut dist_sum = 0;
    for i in 0..galaxies.len()-1 {
        for j in i+1..galaxies.len() {
            let x_dist = ((galaxies[i].x-galaxies[j].x) as isize).abs();
            let y_dist = ((galaxies[i].y-galaxies[j].y) as isize).abs();
            dist_sum += x_dist+y_dist;
        }
    }
    println!("{:?}",dist_sum);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}",grid[y][x]);
        }
        println!("");
    }
}

fn transpose_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let nrows = grid.len();
    let ncols = grid[0].len();

    //Transpose the grid
    let mut grid_t:Vec<Vec<char>> = Vec::new();

    for c in 0..ncols {
        let mut col: Vec<char> = Vec::new();
        for r in 0..nrows {
            col.push(grid[r][c]);
        }
        grid_t.push(col);
    }

    grid_t
}

fn expand_rows(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let nrows = grid.len();
    let ncols = grid[0].len();

    //Expand the empty rows
    let mut row_expand_grid:Vec<Vec<char>> = Vec::new();
    for r in 0..nrows {
        let mut all_empty = true;
        for c in 0..ncols {
            if grid[r][c] != '.' {
                all_empty = false;
                break;
            }
        }

        row_expand_grid.push(grid[r].clone());

        if all_empty {
            row_expand_grid.push(grid[r].clone());
        }
    }

    row_expand_grid
}

fn find_galaxies(grid: &Vec<Vec<char>>) -> Vec<Point> {
    let mut galaxies:Vec<Point> = Vec::new();

    let nrows = grid.len();
    let ncols = grid[0].len();
    for x in 0..ncols {
        for y in 0..nrows {
            if grid[y][x] == '#' {
                galaxies.push(Point{ x:x, y:y });
            }
        }
    }
    galaxies
}
