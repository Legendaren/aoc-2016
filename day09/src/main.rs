use regex::Regex;
use std::fs;

fn repeat(input: &str, n: i32) -> String {
    let mut result = String::new();
    for _ in 0..n {
        result.push_str(input);
    }
    result
}

fn decompress(input: &str) -> String {
    let re = Regex::new(r"(?P<prefix>.*?)\((?P<letter_count>\d+)x(?P<reps>\d+)\)(?P<suffix>.*)")
        .unwrap();
    let mut result = String::new();
    let mut haystack = input.to_string();
    loop {
        if let Some(caps) = re.captures(haystack.as_str()) {
            let prefix = &caps["prefix"];
            let letter_count = caps["letter_count"].parse::<usize>().unwrap();
            let reps = caps["reps"].parse().unwrap();
            let suffix = &caps["suffix"];
            let suffix_repeated = &suffix[0..letter_count];
            let suffix_remaining = &suffix[letter_count..];
            let repeated_letters = repeat(suffix_repeated, reps);
            let part_result = format!("{}{}", prefix, repeated_letters);
            result.push_str(&part_result);
            haystack = suffix_remaining.to_string();
        } else {
            return format!("{}{}", result, haystack);
        }
    }
}

fn part1() -> String {
    let input = fs::read_to_string("input.txt").unwrap();
    decompress(&input)
}

fn main() {
    let part1_answer = part1();
    println!("part 1 answer: {}", part1_answer.len());
}
