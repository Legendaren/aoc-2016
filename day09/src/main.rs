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

fn decompress_v2(input: &str) -> usize {
    let re = Regex::new(r"\((?P<letter_count>\d+)x(?P<reps>\d+)\)").unwrap();
    if let Some(caps) = re.captures(input) {
        let marker_start_index = input.chars().position(|x| x == '(').unwrap();
        let marker_stop_index = input.chars().position(|x| x == ')').unwrap();
        let letter_count = caps["letter_count"].parse::<usize>().unwrap();
        let reps = caps["reps"].parse::<usize>().unwrap();
        let suffix_start = marker_stop_index + 1;
        let suffix_end = suffix_start + letter_count;
        let res = marker_start_index
            + decompress_v2(&input[suffix_start..suffix_end]) * reps
            + decompress_v2(&input[suffix_end..]);
        return res;
    }
    return input.len();
}

fn part1() -> String {
    let input = fs::read_to_string("input.txt").unwrap();
    decompress(&input)
}

fn part2() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    decompress_v2(&input)
}

fn main() {
    let part1_answer = part1();
    println!("part 1 answer: {}", part1_answer.len());

    let part2_answer = part2();
    println!("part 2 answer: {}", part2_answer);
}
