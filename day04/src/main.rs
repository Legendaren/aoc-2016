use std::fs;
use counter::Counter;
use regex::Regex;

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    sector_id: i32,
    checksum: String,
}

fn convert_to_room(input: &str) -> Room {
    let re: Regex = Regex::new(r"([a-z-]+)-(\d+)\[([a-z]{5})\]").unwrap();
    for (_, [m1, m2, m3]) in re.captures_iter(input).map(|caps| caps.extract()) {
        let room = Room {
            encrypted_name: m1.replace("-", ""),
            sector_id: m2.parse().unwrap(),
            checksum: format!("{}", m3)
        };
        return room;
    }
    panic!("Room not found");
}

fn five_most_common_letters(room: &Room) -> String {
    room.encrypted_name
    .chars()
    .collect::<Counter<_>>()
    .most_common_tiebreaker(|&a, &b| a.cmp(&b))
    .iter()
    .map(|(key, _)| format!("{}", key))
    .take(5)
    .collect::<Vec<String>>()
    .join("")
}

fn is_real_room(room: &Room) -> bool {
    five_most_common_letters(&room) == room.checksum
}

fn part1() -> i32 {
    fs::read_to_string("input.txt")
        .expect("Should read file")
        .lines()
        .map(|line| convert_to_room(line))
        .filter(|room| is_real_room(room))
        .map(|room| room.sector_id)
        .sum()
}

fn main() {
    let part1_answer = part1();
    println!("part1 answer: {}", part1_answer);
}
