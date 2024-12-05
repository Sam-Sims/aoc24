use std::collections::HashMap;

// fn part1(rules_map: &HashMap<i32, Vec<i32>>, instructions: &str) -> (i32, Vec<Vec<i32>>) {
//     let mut passed_instruction_sets: Vec<Vec<i32>> = Vec::new();
//     let mut failed_instruction_sets: Vec<Vec<i32>> = Vec::new();
//
//     // loop through every instruction
//     for instruction in instructions.lines() {
//         let instructions_vec: Vec<i32> =
//             instruction.split(",").map(|i| i.parse().unwrap()).collect();
//
//         let mut pass = true;
//         // for each number in the instructions vector look up its set of rules in the hashmap
//         for i in 0..instructions_vec.len() {
//             let Some(rules) = rules_map.get(&instructions_vec[i]) else {
//                 continue;
//             };
//             // check each rule returned from the lookup
//             // the current index must be behind the index of the identifed rules value
//             // if it isnt then the instruction doesnt pass
//             for rule in rules.iter() {
//                 if let Some(rule_index) = instructions_vec.iter().position(|&i| i == *rule) {
//                     if rule_index < i {
//                         pass = false;
//                     }
//                 }
//             }
//         }
//         if pass {
//             passed_instruction_sets.push(instructions_vec);
//         } else {
//             failed_instruction_sets.push(instructions_vec);
//         }
//     }
//
//     (
//         passed_instruction_sets.iter().map(|i| i[i.len() / 2]).sum(),
//         failed_instruction_sets,
//     )
// }

// an improved part1 after someone on reddit suggested the is_sorted_by() function
fn part1(rules_map: &HashMap<i32, Vec<i32>>, instructions: &str) -> (i32, Vec<Vec<i32>>) {
    let mut passed_instruction_sets: Vec<Vec<i32>> = Vec::new();
    let mut failed_instruction_sets: Vec<Vec<i32>> = Vec::new();
    for instruction in instructions.lines() {
        let instructions_vec: Vec<i32> =
            instruction.split(",").map(|i| i.parse().unwrap()).collect();
        // nice function that checks each pair of values in the vector, if the rules map for A contains its adjacent value (B) then A is allowed to be before B
        // this will be then compare each pair in the vec and return true if they all pass
        if instructions_vec.is_sorted_by(|a, b| rules_map[a].contains(b)) {
            passed_instruction_sets.push(instructions_vec);
        } else {
            failed_instruction_sets.push(instructions_vec);
        }
    }

    (
        passed_instruction_sets.iter().map(|i| i[i.len() / 2]).sum(),
        failed_instruction_sets,
    )
}

fn part2(failed_instructions: &mut Vec<Vec<i32>>, rules_map: &HashMap<i32, Vec<i32>>) -> i32 {
    for instructions in failed_instructions.iter_mut() {
        // same parsing as part 1
        for i in 0..instructions.len() {
            let Some(rules) = rules_map.get(&instructions[i]) else {
                continue;
            };
            for rule in rules.iter() {
                if instructions.contains(rule) {
                    let rule_index = instructions.iter().position(|&i| i == *rule).unwrap();
                    if rule_index < i {
                        // if we find an instruction failing the rule, swap them
                        // we brute force it by running this 50 times lol, so no "pass" condition
                        // wait is this bubble sort??
                        instructions.swap(i, rule_index);
                    }
                }
            }
        }
    }
    failed_instructions.iter().map(|i| i[i.len() / 2]).sum()
}

pub fn run(input: &str) {
    let (rules, instructions) = input.split_once("\n\n").unwrap();
    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    // parse each rule into a hashmap to store multiple occurrences of each key in a vector
    // this results in a mapping between keys, and a vector of numbers that the key must be behind
    for rule in rules.lines() {
        let mut rule_parts = rule.split("|");
        let a = rule_parts.next().unwrap();
        let b = rule_parts.next().unwrap();
        rules_map
            .entry(a.parse().unwrap())
            .or_default()
            .push(b.parse().unwrap());
    }

    let (part1, mut failed_instructions) = part1(&rules_map, instructions);
    println!("Part 1 {}", part1);
    let mut i = 0;
    let mut part2_total = 0;
    // cant work out a pass condition so sorting it 50 times should be enough
    while i < 50 {
        part2_total = part2(&mut failed_instructions, &rules_map);
        i += 1;
    }
    println!("Part 2 {}", part2_total)
}
