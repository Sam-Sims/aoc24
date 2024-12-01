use hashbrown::HashMap;
fn part1(col1: &Vec<i32>, col2: &Vec<i32>) {
    let mut total_diff = 0;
    for (i, num) in col1.iter().copied().enumerate() {
        if col2[i] > num {
            total_diff += col2[i] - num;
        } else {
            total_diff += num - col2[i];
        }
    }
    println!("Part1: {}", total_diff);
}

fn part2(col1: &[i32], col2: &[i32]) {
    let mut col2_counts: HashMap<i32, i32> = HashMap::new();
    for num in col2 {
        *col2_counts.entry(*num).or_insert(0) += 1;
    }

    let mut total_diff = 0;
    for num in col1 {
        // since get() returns a reference, unwrap_or() also needs to return a reference (in this case to 0)
        let count = col2_counts.get(num).unwrap_or(&0);
        total_diff += *num * count;
    };

    println!("Part2: {}", total_diff);
}

pub fn run(input: &str) {
    let (mut col1, mut col2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .unzip();

    col1.sort();
    col2.sort();

    part1(&col1, &col2);
    part2(&col1, &col2);
}
