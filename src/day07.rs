fn calc(numbers: &Vec<i64>, index: i32, target: i64, total: i64, concat: bool) -> bool {
    if total > target {
        return false;
    }
    if index == numbers.len() as i32 {
        return total == target;
    }

    let current_num = numbers[index as usize];

    if calc(numbers, index + 1, target, current_num + total, concat) {
        return true;
    }

    if calc(numbers, index + 1, target, current_num * total, concat) {
        return true;
    }

    if concat {
        let concat_total: i64 = format!("{}{}", total.to_string(), current_num.to_string())
            .parse()
            .unwrap();
        if calc(numbers, index + 1, target, concat_total, concat) {
            return true;
        }
    }

    false
}

fn part1(input: &str) -> i64 {
    let mut total = 0;
    for line in input.lines() {
        let mut parts = line.split(":");
        let target: i64 = parts.next().unwrap().parse().unwrap();
        let numbers: Vec<i64> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if calc(&numbers, 0, target, 0, false) {
            total += target;
        }
    }
    total
}

fn part2(input: &str) -> i64 {
    let mut total = 0;
    for line in input.lines() {
        let mut parts = line.split(":");
        let target: i64 = parts.next().unwrap().parse().unwrap();
        let numbers: Vec<i64> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if calc(&numbers, 0, target, 0, true) {
            total += target;
        }
    }
    total
}

pub fn run(input: &str) {
    println!("Part 1 {}", part1(input));
    println!("Part 2 {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_pass() {
        let numbers = vec![81, 40, 27];
        let target = 3267;
        assert_eq!(calc(&numbers, 0, target, 0, false), true);
    }

    #[test]
    fn test_calc_fail() {
        let numbers = vec![6, 8, 6, 15];
        let target = 7290;
        assert_eq!(calc(&numbers, 0, target, 0, false), false);
    }

    #[test]
    fn test_concat() {
        let numbers = vec![6, 8, 6, 15];
        let target = 7290;
        assert_eq!(calc(&numbers, 0, target, 0, true), true);
    }
}
