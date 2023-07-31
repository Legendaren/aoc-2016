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

fn is_aba(input: &str) -> bool {
    let first_char = input.chars().nth(0).unwrap();
    let second_char = input.chars().nth(1).unwrap();
    let third_char = input.chars().nth(2).unwrap();
    first_char == third_char && first_char != second_char
}

fn to_bab(aba: &str) -> String {
    let first_char = aba.chars().nth(0).unwrap();
    let second_char = aba.chars().nth(1).unwrap();
    let bab = vec![second_char, first_char, second_char];
    bab.iter().collect()
}

fn to_char_window_seqs(strings: &Vec<String>, window_size: usize) -> Vec<String> {
    strings
        .iter()
        .map(|ip| ip.chars().collect())
        .map(|char_vec: Vec<char>| {
            char_vec
                .windows(window_size)
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

    let any_abba_in_hypernet: bool = to_char_window_seqs(&hypernet_caps, 4)
        .iter()
        .any(|hypernet| is_abba(&hypernet));
    let any_abba_in_supernet = to_char_window_seqs(&supernet_caps, 4)
        .iter()
        .any(|hypernet| is_abba(&hypernet));
    any_abba_in_supernet && !any_abba_in_hypernet
}

fn supports_ssl(ip: &str) -> bool {
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

    let abas: Vec<String> = to_char_window_seqs(&supernet_caps, 3)
        .into_iter()
        .filter(|supernet| is_aba(&supernet))
        .collect();

    let hypernet_seqs: Vec<String> = to_char_window_seqs(&hypernet_caps, 3);

    let any_bab_in_supernet = abas
        .iter()
        .map(|aba| to_bab(aba))
        .any(|bab| hypernet_seqs.iter().any(|seq| &bab == seq));

    any_bab_in_supernet
}

fn part1() -> i32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter(|ip| supports_tls(ip))
        .count() as i32
}

fn part2() -> i32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter(|ip| supports_ssl(ip))
        .count() as i32
}

fn main() {
    let part1_answer = part1();
    let part2_answer = part2();
    println!("{:?}", part1_answer);
    println!("{:?}", part2_answer);
}
