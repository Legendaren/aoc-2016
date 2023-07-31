use regex::Regex;
use std::fs;

fn is_abba(input: &str) -> bool {
    let first_char = input.chars().nth(0).unwrap();
    let second_char = input.chars().nth(1).unwrap();
    let first_two = &input[..2];
    let first_two_rev: String = first_two.chars().rev().collect();
    let last_two = &input[2..];
    first_char != second_char && first_two_rev == last_two
}

fn to_4_char_window_seqs(strings: Vec<String>) -> Vec<String> {
    strings
        .iter()
        .map(|ip| ip.chars().collect())
        .map(|char_vec: Vec<char>| {
            char_vec
                .windows(4)
                .map(|window| window.to_vec())
                .collect::<Vec<_>>()
        })
        .flatten()
        .map(|v| v.iter().collect::<String>())
        .collect()
}

fn supports_tls(ip: &str) -> bool {
    let re = Regex::new(r"(?P<Supernet>[a-z]+)*(?P<Hypernet>\[[a-z]+\])*").unwrap();
    let hypernet_caps: Vec<String> = re
        .captures_iter(ip)
        .filter_map(|caps| {
            caps.name("Hypernet")
                .map(|supernet| supernet.as_str())
                .map(|supernet| supernet.replace("[", ""))
                .map(|supernet| supernet.replace("]", ""))
        })
        .collect();
    let supernet_caps: Vec<String> = re
        .captures_iter(ip)
        .filter_map(|caps| {
            caps.name("Supernet")
                .map(|supernet| supernet.as_str().to_string())
        })
        .collect();

    let any_abba_in_hypernet: bool = to_4_char_window_seqs(hypernet_caps)
        .iter()
        .any(|hypernet| is_abba(&hypernet));

    let any_abba_in_supernet = to_4_char_window_seqs(supernet_caps)
        .iter()
        .any(|hypernet| is_abba(&hypernet));

    //println!("{:?}", hypernet_caps);
    //println!("{:?}", supernet_caps);
    any_abba_in_supernet && !any_abba_in_hypernet
}

fn part1() -> i32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|ip| supports_tls(ip))
        .filter(|&tls| tls)
        .count() as i32
}

fn main() {
    let part1_answer = part1();
    println!("{:?}", part1_answer);
}
