// inspired by a reddit comment using all() to check each index agaisnt MAS
const MAS: [char; 3] = ['M', 'A', 'S'];

fn part_one(grid: &[Vec<char>]) -> i32 {
    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 'X' {
                continue;
            }
            // check backwards for SAMX
            if x >= 3 && grid[y][x - 3..x] == ['S', 'A', 'M'] {
                total += 1;
            }
            // check forwards for MAS
            if x + 3 < grid[0].len() && grid[y][x + 1..x + 4] == MAS {
                total += 1;
            }
            // check up for MAS
            if y >= 3 && (1..4).all(|i| grid[y - i][x] == MAS[i - 1]) {
                total += 1;
            }
            // check down for MAS
            if y + 3 < grid.len() && (1..4).all(|i| grid[y + i][x] == MAS[i - 1]) {
                total += 1;
            }
            // check diag up left
            if x >= 3 && y >= 3 && (1..4).all(|i| grid[y - i][x - i] == MAS[i - 1]) {
                total += 1;
            }
            // check diag down left
            if x >= 3 && y + 3 < grid.len() && (1..4).all(|i| grid[y + i][x - i] == MAS[i - 1]) {
                total += 1;
            }
            // check diag up right
            if x + 3 < grid[0].len() && y >= 3 && (1..4).all(|i| grid[y - i][x + i] == MAS[i - 1]) {
                total += 1;
            }
            // check diag down right
            if x + 3 < grid[0].len()
                && y + 3 < grid.len()
                && (1..4).all(|i| grid[y + i][x + i] == MAS[i - 1])
            {
                total += 1;
            }
        }
    }
    total
}

fn part_two(grid: &[Vec<char>]) -> i32 {
    let mut total = 0;
    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if grid[y][x] != 'A' {
                continue;
            }

            if (grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S'
                || grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M')
                && (grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S'
                    || grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M')
            {
                total += 1;
            }
        }
    }
    total
}

pub fn run(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let part1 = part_one(&grid);
    println!("Part one: {}", part1);
    let part2 = part_two(&grid);
    println!("Part two: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "S  S  S
 A A A 
  MMM  
SAMXMAS
  MMM  
 A A A 
S  S  S
";

        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let total = part_one(&grid);
        assert_eq!(total, 8)
    }
}
