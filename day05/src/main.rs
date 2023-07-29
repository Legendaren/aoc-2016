use std::collections::HashMap;

use itertools::Itertools;

fn part1() -> String {
    let input = "reyedfim";
    let prefix = String::from(input);
    let mut password = String::new();
    let mut i = 0;
    while password.len() < 8 {
        let hash_input = format!("{}{}", prefix, i);
        let hash = md5::compute(hash_input);
        let hash_str = format!("{:x}", hash);
        if hash_str.chars().take(5).collect::<String>() == "00000" {
            password.push(hash_str.chars().nth(5).unwrap());
        }
        i += 1;
    }
    password
}

fn part2() -> String {
    let input = "reyedfim";
    let prefix = String::from(input);
    let mut password = HashMap::new();
    let mut i = 0;
    while password.len() < 8 {
        let hash_input = format!("{}{}", prefix, i);
        let hash = md5::compute(hash_input);
        let hash_str = format!("{:x}", hash);
        if hash_str.chars().take(5).collect::<String>() == "00000" {
            let position = hash_str.chars().nth(5).unwrap();
            let is_valid_hash = position.is_digit(10)
                && (0..8).contains(&position.to_digit(10).unwrap())
                && !password.contains_key(&position);
            if is_valid_hash {
                let char_to_add = hash_str.chars().nth(6).unwrap();
                password.insert(position, char_to_add);
            }
        }
        i += 1;
    }
    password
        .keys()
        .sorted()
        .map(|key| password.get(key).unwrap())
        .collect::<String>()
}

fn main() {
    //let part1_answer = part1();
    let part2_answer = part2();
    //println!("{}", part1_answer);
    println!("{}", part2_answer);
}
