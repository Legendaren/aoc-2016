use std::fs;
use std::collections::HashSet;

enum Direction {
    North, West, East, South,
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_x(&mut self, steps: i32) {
        self.x += steps
    }

    fn move_y(&mut self, steps: i32) {
        self.y += steps
    }
}

struct Walker {
    position: Position,
    direction: Direction,
}

impl Walker {
    fn blocks_away(&self) -> i32 {
        self.position.x + self.position.y
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }

    fn move_pos(&mut self, steps: i32) {
        match self.direction {
            Direction::North => self.position.move_y(steps),
            Direction::West => self.position.move_x(-steps),
            Direction::South => self.position.move_y(-steps),
            Direction::East => self.position.move_x(steps),
        }
    }

    fn path(&self, steps: i32) -> Vec<Position> {
        match self.direction {
            Direction::North => (0..steps).map(|i: i32| Position{x: self.position.x, y: self.position.y + i}).collect(),
            Direction::West => (0..steps).map(|i: i32| Position{x: self.position.x - i, y: self.position.y}).collect(),
            Direction::South => (0..steps).map(|i: i32| Position{x: self.position.x, y: self.position.y - i}).collect(),
            Direction::East => (0..steps).map(|i: i32| Position{x: self.position.x + i, y: self.position.y}).collect(),
        }
    }
}


fn part1() -> i32 {
    let input = fs::read_to_string("input.txt").expect("Should read file");
    let coords: Vec<&str> = input.split(",").map(|s| s.trim()).collect();

    let mut walker = Walker {
        position: Position {x: 0, y: 0},
        direction: Direction::North
    };
    for coord in coords {
        let dir_input = coord.chars().next().unwrap();
        match dir_input {
            'L' => walker.turn_left(),
            'R' => walker.turn_right(),
            _ => panic!("invalid dir")
        };

        let steps: i32 = coord[1..].parse().unwrap();
        walker.move_pos(steps);
    }
    walker.blocks_away()
}

fn part2() -> i32 {
    let input = fs::read_to_string("input.txt").expect("Should read file");
    let coords: Vec<&str> = input.split(",").map(|s| s.trim()).collect();

    let mut visited: HashSet<Position> = HashSet::new();
    let mut walker = Walker {
        position: Position {x: 0, y: 0},
        direction: Direction::North
    };
    for coord in coords {

        let dir_input = coord.chars().next().unwrap();
        match dir_input {
            'L' => walker.turn_left(),
            'R' => walker.turn_right(),
            _ => panic!("invalid dir")
        };

        let steps: i32 = coord[1..].parse().unwrap();
        let path = walker.path(steps);
        for pos in path {
            if visited.contains(&pos) {
                return pos.x + pos.y;
            }
            visited.insert(pos);
        }
        walker.move_pos(steps);
    }
    walker.blocks_away()
}



fn main() {
    let part1_answer = part1();
    let part2_answer = part2();
    println!("part1 answer: {}", part1_answer);
    println!("part2 answer: {}", part2_answer);
}
