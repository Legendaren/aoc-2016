use counter::Counter;
use std::fs;

fn part1() -> String {
    let file_content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file_content.lines().collect();
    let mut message = String::new();
    let column_count = lines[0].len();

    for i in 0..column_count {
        let chars_in_column: Vec<char> = lines
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .collect();
        let char_counts = chars_in_column.iter().collect::<Counter<_>>();
        message.push(*char_counts.most_common().first().unwrap().0);
    }
    message
}

fn main() {
    println!("{}", part1());
}
