use std::collections::HashSet;

enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn next_pos(&self, pos: (i32, i32)) -> (i32, i32) {
        let (y, x) = pos;
        match self {
            Direction::N => (y - 1, x),
            Direction::E => (y, x + 1),
            Direction::S => (y + 1, x),
            Direction::W => (y, x - 1),
        }
    }
}

fn part1(grid: &[Vec<char>]) -> HashSet<(i32, i32)> {
    let start_y = grid.iter().position(|p| p.contains(&'^')).unwrap();
    let start_x = grid[start_y].iter().position(|p| *p == '^').unwrap();
    let mut current_pos = (start_y as i32, start_x as i32);
    let mut direction = Direction::N;
    let mut visited = HashSet::new();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    loop {
        visited.insert(current_pos);

        let (new_y, new_x) = direction.next_pos(current_pos);

        // check bounds
        if new_y < 0 || new_y >= height || new_x < 0 || new_x >= width {
            break;
        }

        if grid[new_y as usize][new_x as usize] == '#' {
            direction = direction.turn();
        } else {
            current_pos = (new_y, new_x);
        }
    }
    visited
}

fn part2(visited_locations: HashSet<(i32, i32)>, grid: Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    for (y, x) in visited_locations.iter() {
        let mut obstacle_grid = grid.clone();
        let start_y = obstacle_grid.iter().position(|p| p.contains(&'^')).unwrap();
        let start_x = obstacle_grid[start_y]
            .iter()
            .position(|p| *p == '^')
            .unwrap();
        // add a obstacle in the current position and see if the guard can exit
        obstacle_grid[*y as usize][*x as usize] = '#';
        let mut current_pos = (start_y as i32, start_x as i32);
        let mut direction = Direction::N;
        let height = obstacle_grid.len() as i32;
        let width = obstacle_grid[0].len() as i32;
        let mut count = 0;
        let mut escaped = false;

        // try 10000 times - if the guard cant escape we probably are in a loop so add 1 to total
        while count < 10000 {
            let (new_y, new_x) = direction.next_pos(current_pos);

            // check bounds - if gaurd goes out of bounds we have "escaped" and dont add any to total
            if new_y < 0 || new_y >= height || new_x < 0 || new_x >= width {
                escaped = true;
                break;
            }

            if obstacle_grid[new_y as usize][new_x as usize] == '#' {
                direction = direction.turn();
            } else {
                current_pos = (new_y, new_x);
            }
            count += 1;
        }
        if !escaped {
            total += 1;
        }
    }
    total
}

pub fn run(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // return hashset of all visited locations to use as bruteforce input in part2 by only inserting an obstacle in the places the guard can actually visit
    let part1_locations = part1(&grid);
    println!("Part one: {}", part1_locations.len());

    let part2_total = part2(part1_locations, grid);
    println!("Part two: {}", part2_total);
}
