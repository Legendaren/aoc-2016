use std::fs;
use std::env;

fn input_to_move(c: char) -> (i32, i32) {
    match c {
        'U' => (-1, 0),
        'R' => (0, 1),
        'L' => (0, -1),
        'D' => (1, 0),
        _ => panic!("Invalid character")
    }
}

fn out_of_bounds(keypad: &Vec<Vec<&str>>, (row, col): (i32, i32)) -> bool{
    let row_out_of_bounds = row < 0 || row >= keypad.len() as i32;
    let col_out_of_bounds = col < 0 || col >= keypad.len() as i32;
    row_out_of_bounds || col_out_of_bounds
}

fn part1() -> String {
    let input = fs::read_to_string("input.txt").expect("Should read file");
    let lines: Vec<String> = input.split("\n").map(|x| x.trim()).map(str::to_string).collect();
    let keypad = vec![
        ["1", "2", "3"],
        ["4", "5", "6"],
        ["7", "8", "9"]
    ];
    let (mut row, mut col) = (1, 1);
    let mut registered_keys: Vec<&str> = Vec::new();

    for line in lines {
        let moves: Vec<(i32, i32)> = line.chars().into_iter().map(|c| input_to_move(c)).collect();
        for (row_move, col_move) in moves {
            let (new_row, new_col) = (row + row_move, col + col_move);
            if let Some(_) = keypad
            .get(new_row as usize)
            .and_then(|row| row.get(new_col as usize)) {
                row = new_row;
                col = new_col;
            }
        }
        registered_keys.push(keypad[row as usize][col as usize]);
    }

    println!("{:?}", registered_keys);
    registered_keys.join("")
}

fn part2() -> String {
    let input = fs::read_to_string("input.txt").expect("Should read file");
    let lines: Vec<String> = input.split("\n").map(|x| x.trim()).map(str::to_string).collect();
    let keypad = vec![
        vec!["", "", "1", "", ""],
        vec!["", "2", "3", "4", ""],
        vec!["5", "6", "7", "8", "9"],
        vec!["", "A", "B", "C", ""],
        vec!["", "", "D", "", ""],
    ];
    let (mut row, mut col) = (2, 0);
    let mut registered_keys: Vec<&str> = Vec::new();

    for line in lines {
        let moves: Vec<(i32, i32)> = line.chars().into_iter().map(input_to_move).collect();
        for (row_move, col_move) in moves {
            let (new_row, new_col) = (row + row_move, col + col_move);
            if !out_of_bounds(&keypad, (new_row, new_col)) && keypad[new_row as usize][new_col as usize] != ""{
                row = new_row;
                col = new_col;
            }
        }
        registered_keys.push(keypad[row as usize][col as usize]);
    }

    println!("{:?}", registered_keys);
    registered_keys.join("")
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let part1_answer = part1();
    let part2_answer = part2();
    println!("part1 answer: {}", part1_answer);
    println!("part2 answer: {}", part2_answer);
}
