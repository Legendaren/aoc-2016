use std::fs;

enum Direction {
    North, West, East, South,
}


fn turn_left(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North
    }
}

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::West => Direction::North,
        Direction::South => Direction::West,
        Direction::East => Direction::South
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").expect("Should read file");
    let coords: Vec<&str> = input.split(",").map(|s| s.trim()).collect();

    let mut x = 0;
    let mut y = 0;
    let mut dir = Direction::North;
    for coord in coords {
        let dir_input = coord.chars().next().unwrap();
        dir = match dir_input {
            'L' => turn_left(&dir),
            'R' => turn_right(&dir),
            _ => panic!("invalid dir")
        };

        let steps: i32 = coord[1..].parse().unwrap();
        match dir {
            Direction::North => y += steps,
            Direction::South => y -= steps,
            Direction::West => x -= steps,
            Direction::East => x += steps,
        }
    }

    let blocks = x + y;

    println!("{}", blocks);
}
