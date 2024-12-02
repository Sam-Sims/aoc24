fn part_one(input: &str) {
    let mut safe = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        // check they are in ascending or descending order
        if nums.windows(2).all(|w| w[0] < w[1]) || nums.windows(2).all(|w| w[0] > w[1]) {
            // if they are check the difference
            if nums
                .windows(2)
                .all(|w| w[0].abs_diff(w[1]) <= 3 && w[0].abs_diff(w[1]) >= 1)
            {
                // if all rules pass, we are safe
                safe += 1;
            }
        }
    }
    println!("Part1: {}", safe);
}

fn part_two(input: &str) {
    let mut safe = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        for i in 0..nums.len() {
            let mut nums = nums.clone();
            nums.remove(i);
            if nums.windows(2).all(|w| w[0] < w[1]) || nums.windows(2).all(|w| w[0] > w[1]) {
                if nums
                    .windows(2)
                    .all(|w| w[0].abs_diff(w[1]) <= 3 && w[0].abs_diff(w[1]) >= 1)
                {
                    // if the clone does pass, we are safe and no need to process rest of the indexes
                    safe += 1;
                    break;
                }
            }
        }
    }
    println!("Part2: {}", safe);
}

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
}
