use std::fs;

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Maze {
    grid: Vec<Vec<char>>,
    nrow: usize,
    ncol: usize,
    start: Pos,
}

impl Maze {
    fn new<'a>(lines: impl Iterator<Item = &'a str>) -> Maze {
        let grid: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
        let nrow = grid.len();
        let ncol = grid[0].len();

        let start_ind = grid
            .iter()
            .map(|r| r
            .iter())
            .flatten()
            .position(|&c| c == 'S')
            .unwrap();

        let start_x = start_ind % ncol;
        let start_y = start_ind / ncol;

        Maze {
            grid: grid,
            nrow: nrow,
            ncol: ncol,
            start: Pos{ x: start_x, y: start_y},
        }
    }

    fn next_step(self: &Self, p: &Pos, prev: &Pos) -> Pos {
        let prev_x: isize = (p.x-prev.x) as isize;
        let prev_y: isize = (p.y-prev.y) as isize;
        let prev_dir = (prev_x, prev_y);

        //( 0, -1) means previous position was BELOW
        //( 0,  1) means previous position was ABOVE
        //(-1,  0) means previous position was RIGHT
        //( 1,  0) means previous position was LEFT
        match (self.grid[p.y][p.x], prev_dir) {
            ('|',( 0,-1)) => Pos { x: p.x  , y: p.y-1},
            ('|',( 0, 1)) => Pos { x: p.x  , y: p.y+1},
            ('-',(-1, 0)) => Pos { x: p.x-1, y: p.y  },
            ('-',( 1, 0)) => Pos { x: p.x+1, y: p.y  },
            ('L',( 0, 1)) => Pos { x: p.x+1, y: p.y  },
            ('L',(-1, 0)) => Pos { x: p.x  , y: p.y-1},
            ('J',( 0, 1)) => Pos { x: p.x-1, y: p.y  },
            ('J',( 1, 0)) => Pos { x: p.x  , y: p.y-1},
            ('7',( 1, 0)) => Pos { x: p.x  , y: p.y+1},
            ('7',( 0,-1)) => Pos { x: p.x-1, y: p.y  },
            ('F',(-1, 0)) => Pos { x: p.x  , y: p.y+1},
            ('F',( 0,-1)) => Pos { x: p.x+1, y: p.y  },
            _ => panic!("Fell out of maze somehow!"),
        }
    }

    fn tile(self: &Self, x: isize, y: isize) -> char {
        if x < 0 || x >= self.ncol.try_into().unwrap() || y < 0 || y >= self.nrow.try_into().unwrap() {
            '.'
        } else {
            self.grid[y as usize][x as usize]
        }
    }

    fn get_start_adjacent_squares(self: &Self) -> (Pos, Pos) {

        //find the 2 tiles out of all 4 surrounding S that connect
        let mut connection_squares = Vec::new();

        //TODO this type conversion is terrible
        let x: isize = (self.start.x) as isize;
        let y: isize = (self.start.y) as isize;
        let l: isize = (self.start.x-1) as isize;
        let r: isize = (self.start.x+1) as isize;
        let u: isize = (self.start.y-1) as isize;
        let d: isize = (self.start.y+1) as isize;

        //check square above
        if "|7F".contains(self.tile(x,u)){
            connection_squares.push( Pos{ x: self.start.x, y: self.start.y-1 });
        }
        //check square below
        if "|JL".contains(self.tile(x,d)){
            connection_squares.push( Pos{ x: self.start.x, y: self.start.y+1 });
        }
        //check square left
        if "-LF".contains(self.tile(l,y)){
            connection_squares.push( Pos{ x: self.start.x-1, y: self.start.y });
        }
        //check square right
        if "-7J".contains(self.tile(r,y)){
            connection_squares.push( Pos{ x: self.start.x+1, y: self.start.y });
        }

        //edge case where there aren't any connecting squares
        if connection_squares.len() == 0 {
            panic!("No connecting tiles to S");
        }

        let c1 = connection_squares.pop().unwrap();
        let c2 = connection_squares.pop().unwrap();

        return (c1,c2);
    }

    fn furthest_end(self: &Self) -> usize {

        let (mut c1, mut c2) = self.get_start_adjacent_squares();

        let mut prev_c1 = Pos { x: self.start.x, y: self.start.y};
        let mut prev_c2 = Pos { x: self.start.x, y: self.start.y};

        //walk both branches until they collide
        let mut steps = 1;
        loop {
            if c1.x == c2.x && c1.y == c2.y { break; }

            steps += 1;

            let new_c1 = self.next_step(&c1, &prev_c1);
            prev_c1 = c1;
            c1 = new_c1;

            let new_c2 = self.next_step(&c2, &prev_c2);
            prev_c2 = c2;
            c2 = new_c2;
        }

        return steps;
    }

    fn mark_loop(self: &Self) -> Maze {
        let (mut c1, mut c2) = self.get_start_adjacent_squares();

        let mut prev_c1 = Pos { x: self.start.x, y: self.start.y};
        let mut prev_c2 = Pos { x: self.start.x, y: self.start.y};

        let mut marked_grid = vec![vec!['0'; self.ncol]; self.nrow];

        marked_grid[self.start.y][self.start.x] = '1';

        //walk both branches until they collide
        loop {
            marked_grid[c1.y][c1.x] = '1';
            marked_grid[c2.y][c2.x] = '1';

            if c1.x == c2.x && c1.y == c2.y { break; }

            let new_c1 = self.next_step(&c1, &prev_c1);
            prev_c1 = c1;
            c1 = new_c1;

            let new_c2 = self.next_step(&c2, &prev_c2);
            prev_c2 = c2;
            c2 = new_c2;
        }

        Maze {
            grid: marked_grid,
            nrow: self.nrow,
            ncol: self.ncol,
            start: Pos{ x: self.start.x, y: self.start.y},
        }
    }

    fn internal_tiles(self: &Self) -> usize {
        let mut num_internal = 0;

        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == '1' { continue };

                //count how many loop tiles are between here
                //and the right boundary of the grid
                let mut num_intersections_right = 0;
                for c in x..self.grid[y].len() {
                    if self.grid[y][c] == '1' {
                        num_intersections_right += 1;
                    }
                }

                //count how many loop tiles are between here
                //and the left boundary of the grid
                let mut num_intersections_left = 0;
                for c in 0..x {
                    if self.grid[y][c] == '1' {
                        num_intersections_left += 1;
                    }
                }

                if (num_intersections_right % 2 == 1) && (num_intersections_left % 2 == 1) {
                    println!("{x},{y}");
                    num_internal += 1;
                }
            }
        }

        num_internal
    }

}

/// Day 10 problem 1
pub fn run_part1(data: &str) {

    let binding = fs::read_to_string(data).unwrap();
    let lines = binding.lines();

    let maze = Maze::new(lines);
    let steps = maze.furthest_end();
    println!("{:?}",steps);
}

/// Day 10 problem 2
pub fn run_part2(data: &str) {

    let binding = fs::read_to_string(data).unwrap();
    let lines = binding.lines();

    //create the maze and then
    //create a new maze board where
    //all tiles that are part of the loop
    //are 1, otherwise 0.
    let maze = Maze::new(lines);
    let maze = maze.mark_loop();

    //count internal tiles
    let num_internal = maze.internal_tiles();
    println!("{:?}",num_internal);
}


