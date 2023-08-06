use regex::Regex;
use std::collections::VecDeque;
use std::fs;

type Matrix = VecDeque<VecDeque<i32>>;

fn print_matrix(matrix: &Matrix) {
    let mut s = String::new();
    for row in matrix {
        for col in row {
            match col {
                0 => s.push('.'),
                1 => s.push('#'),
                _ => s.push('E'),
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn pixels_turned_on(matrix: &Matrix) -> i32 {
    matrix
        .iter()
        .map(|row| row.iter().filter(|&&x| x == 1).count())
        .sum::<usize>() as i32
}

fn create_matrix(width: i32, height: i32) -> Matrix {
    (0..height)
        .map(|_| (0..width).map(|_| 0).collect::<VecDeque<i32>>())
        .collect::<Matrix>()
}

fn turn_on_rect(matrix: Matrix, width: i32, height: i32) -> Matrix {
    let mut new_matrix = matrix.clone();
    for i in 0..height {
        for j in 0..width {
            new_matrix[i as usize][j as usize] = 1
        }
    }
    new_matrix
}

fn rotate_column(matrix: Matrix, column: i32, rotations: i32) -> Matrix {
    let mut new_matrix = matrix.clone();

    let mut column_elems: Vec<_> = matrix.into_iter().map(|row| row[column as usize]).collect();
    column_elems.rotate_right(rotations as usize);

    for i in 0..new_matrix.len() {
        new_matrix[i][column as usize] = column_elems[i];
    }
    new_matrix
}

fn rotate_row(matrix: Matrix, row: i32, rotations: i32) -> Matrix {
    let mut new_matrix = matrix.clone();
    new_matrix[row as usize].rotate_right(rotations as usize);
    new_matrix
}

fn process(matrix: Matrix, line: &str) -> Matrix {
    let rect_re = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let rotate_column_re = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
    let rotate_row_re = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();

    if let Some(captures) = rect_re.captures(line) {
        let width: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let height: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        return turn_on_rect(matrix, width, height);
    } else if let Some(captures) = rotate_column_re.captures(line) {
        let column: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let rotations: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        return rotate_column(matrix, column, rotations);
    } else if let Some(captures) = rotate_row_re.captures(line) {
        let row: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let rotations: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        println!("row: {}", row);
        return rotate_row(matrix, row, rotations);
    }
    matrix
}

fn part1() -> i32 {
    let mut matrix = create_matrix(50, 6);
    let input = fs::read_to_string("input.txt").unwrap();
    for line in input.lines() {
        matrix = process(matrix, line);
        print_matrix(&matrix);
    }
    pixels_turned_on(&matrix)
}

fn main() {
    let part1_answer = part1();
    println!("part1 answer: {}", part1_answer);
}
