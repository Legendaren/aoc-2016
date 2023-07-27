use std::fs;
use std::env;


fn generate_keypad(size: i32) -> Vec<Vec<String>> {
    (1..=size)
    .map(|i| ((i-1)*size+1..i*size+1).collect())
    .map(|x: Vec<i32>| x.iter().map(|y| y.to_string()).collect())
    .collect()
}

fn input_to_move(c: char) -> (i32, i32) {
    match c {
        'U' => (-1, 0),
        'R' => (0, 1),
        'L' => (0, -1),
        'D' => (1, 0),
        _ => panic!("Invalid character")
    }
}

fn center_coords(keypad: &Vec<Vec<String>>) -> (i32, i32) {
    let rows = keypad.len();
    let columns = keypad[0].len();
    ((rows / 2) as i32, (columns / 2) as i32)
}

fn out_of_bounds(keypad: &Vec<Vec<String>>, (row, col): (i32, i32)) -> bool {
    let row_out_of_bounds = row < 0 || row >= keypad.len() as i32;
    let col_out_of_bounds = col < 0 || col >= keypad[0].len() as i32;
    row_out_of_bounds || col_out_of_bounds
}

fn part1() -> String {
    let input = fs::read_to_string("input.txt").expect("Should read file");
    let lines: Vec<String> = input.split("\n").map(|x| x.trim()).map(str::to_string).collect();
    let size = 3;
    let keypad = generate_keypad(size);
    let (mut row, mut col) = center_coords(&keypad);
    let mut registered_keys: Vec<&str> = Vec::new();

    for line in lines {
        let moves: Vec<(i32, i32)> = line.chars().into_iter().map(|c| input_to_move(c)).collect();
        for (row_move, col_move) in moves {
            let (new_row, new_col) = (row + row_move, col + col_move);
            if !out_of_bounds(&keypad, (new_row, new_col)) {
                row = new_row;
                col = new_col;
            }
        }
        let number = &keypad[row as usize][col as usize];
        registered_keys.push(number);
    }

    println!("{:?}", registered_keys);
    registered_keys.join("")
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let part1_answer = part1();
    println!("part1 answer: {}", part1_answer);
}
