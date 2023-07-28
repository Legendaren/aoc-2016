use std::fs;

fn line_to_triangle(line: &str) -> Vec<i32> {
    line
    .split(" ")
    .filter(|x| !x.is_empty())
    .map(String::from)
    .map(|x| x.parse().unwrap())
    .collect::<Vec<i32>>()
}

fn is_triangle(triangle: &Vec<i32>) -> bool {
    let mut sorted: Vec<i32> = triangle.clone();
    sorted.sort();
    sorted[0] + sorted[1] > sorted[2]
}

fn part1() -> i32 {
    let input = fs::read_to_string("input.txt").expect("Should read file");
    let triangles: Vec<Vec<i32>> = input.lines().map(line_to_triangle).collect();
    let triangle_count = triangles.iter().map(is_triangle).filter(|x| *x).count();
    triangle_count as i32
}

fn main() {
    let part1_answer = part1();
    println!("{}", part1_answer)
}
