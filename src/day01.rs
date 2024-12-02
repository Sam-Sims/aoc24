use fxhash::FxHashMap;

// fn part1(col1: &Vec<i32>, col2: &Vec<i32>) {
//     let mut total_diff = 0;
//     for (i, num) in col1.iter().copied().enumerate() {
//         if col2[i] > num {
//             total_diff += col2[i] - num;
//         } else {
//             total_diff += num - col2[i];
//         }
//     }
//
//     println!("Part1: {}", total_diff);
// }
//
// fn part2(col1: &[i32], col2: &[i32]) {
//     let mut col2_counts: HashMap<i32, i32> = HashMap::new();
//     for num in col2 {
//         *col2_counts.entry(*num).or_insert(0) += 1;
//     }
//
//     let mut total_diff = 0;
//     for num in col1 {
//         // since get() returns a reference, unwrap_or() also needs to return a reference (in this case to 0)
//         let count = col2_counts.get(num).unwrap_or(&0);
//         total_diff += *num * count;
//     }
//
//     println!("Part2: {}", total_diff);
// }

fn part1_improved(col1: &[i32], col2: &[i32]) {
    // should make use of iterator methods instead of for loop
    let total_diff: u32 = col1
        .iter()
        .zip(col2.iter())
        // turns out there is an abs_diff function, which avoids the IF statement
        .map(|(col1, col2)| (col1.abs_diff(*col2)))
        .sum();
    println!("Part1: {}", total_diff);
}

fn part2_improved(col1: &[i32], col2: &[i32]) {
    // the hashmap will always be of a fixed size so we can preallocate the memory
    let mut col2_counts: FxHashMap<i32, i32> =
        FxHashMap::with_capacity_and_hasher(col2.len(), Default::default());
    for num in col2 {
        *col2_counts.entry(*num).or_insert(0) += 1;
    }

    // again making better use of iterator methods
    let total_diff: i32 = col1
        .iter()
        .map(|num| num * col2_counts.get(num).unwrap_or(&0))
        .sum();

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

    // we can use sort_unstable as the order of equal elements doesnt matter, and apparently its faster
    col1.sort_unstable();
    col2.sort_unstable();

    part1_improved(&col1, &col2);
    part2_improved(&col1, &col2);
}
