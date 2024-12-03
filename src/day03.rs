use regex::Regex;

fn parse_instruction(instruction: &str) -> i32 {
    let re = Regex::new(r"(\d{1,3}).(\d{1,3})").unwrap();
    let captures = re.captures(instruction).unwrap();
    let first = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let second = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    first * second
}

fn parse_instruction_string(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    // came across really nice function to iterate over matches
    // https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_iter
    re.find_iter(input)
        .map(|x| parse_instruction(x.as_str()))
        .sum()
}
fn part_one(input: &str) {
    let sum = parse_instruction_string(input);
    println!("Part1: {}", sum);
}

fn part_two(input: &str) {
    // split by do() which gets blocks where the start of the string until dont() contain instructions to multiply
    // so then we can just split by dont() and then take the first element
    let instruction_string: String = input
        .split("do()")
        .flat_map(|x| x.split("don't()").next())
        .collect();
    let sum = parse_instruction_string(&instruction_string);
    println!("Part2: {}", sum);
}

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
}
