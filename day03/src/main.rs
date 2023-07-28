use std::fs;
use std::env;

fn line_to_triangle(line: &str) -> Vec<i32> {
    line
    .split(" ")
    .filter(|x| !x.is_empty())
    .map(String::from)
    .map(|x| x.parse().unwrap())
    .collect::<Vec<i32>>()
}

fn is_triangle(triangle: &[i32]) -> bool {
    let mut sorted: Vec<i32> = vec![0; 3];
    sorted.clone_from_slice(triangle);
    sorted.sort();
    sorted[0] + sorted[1] > sorted[2]
}

fn transpose(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let mut new_vec: Vec<Vec<i32>> = Vec::new();
    for i in 0..matrix[0].len() {
        let new_row = matrix.iter().map(|row| row[i]).collect();
        new_vec.push(new_row);
    }
    new_vec
}

fn part1() -> i32 {
    let input = fs::read_to_string("input.txt").expect("Should read file");
    let triangles: Vec<Vec<i32>> = input.lines().map(line_to_triangle).collect();
    let triangle_count = triangles.iter().map(|triangle| is_triangle(&triangle)).filter(|x| *x).count();
    triangle_count as i32
}

fn part2() -> i32 {
    let input = fs::read_to_string("input.txt").expect("Should read file");
    let triangles_rows: Vec<Vec<i32>> = input.lines().map(line_to_triangle).collect();
    let triangles_columns = transpose(&triangles_rows);
    let triangles_nums: Vec<i32> = triangles_columns.into_iter().flatten().collect();
    let triangles: Vec<&[i32]> = triangles_nums.chunks(3).collect();
    let triangle_count = triangles.iter().map(|triangle| is_triangle(triangle)).filter(|x| *x).count();
    triangle_count as i32
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let part1_answer = part1();
    let part2_answer = part2();
    println!("part1 answer: {}", part1_answer);
    println!("part2 answer: {}", part2_answer);
}
