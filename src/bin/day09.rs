use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn main() {
    let file = File::open("input/09.txt").unwrap();
    let reader = BufReader::new(file);

    let mut instructions = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(" ").collect();

        let direction = match split[0].chars().next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unreachable!(),
        };
        let distance = (split[1]).parse::<u32>().unwrap();

        for _ in 0..distance {
            instructions.push(direction.clone());
        }
    }

    let mut p1_visited = HashMap::<Position, bool>::new();
    let mut p2_visited = HashMap::<Position, bool>::new();

    let mut knot_positions = vec![ Position{ x: 0, y: 0 }; 10];

    for ins in &instructions {
        // Move head
        match ins {
            Direction::Left  => { knot_positions[0].x -= 1; },
            Direction::Right => { knot_positions[0].x += 1; },
            Direction::Down  => { knot_positions[0].y += 1; },
            Direction::Up    => { knot_positions[0].y -= 1; },
        }

        // Move tail pieces
        for i in 1..10 {
            let x_diff = knot_positions[i-1].x - knot_positions[i].x;
            let y_diff = knot_positions[i-1].y - knot_positions[i].y;

            let mut desired_pos = knot_positions[i].clone();

            if x_diff.abs() >= 1 {
                if x_diff > 0 {
                    desired_pos.x += 1;
                } else {
                    desired_pos.x -= 1;
                }
            }
            if y_diff.abs() >= 1 {
                if y_diff > 0 {
                    desired_pos.y += 1;
                } else {
                    desired_pos.y -= 1;
                }
            }

            // Move as close to desired position as possible
            if desired_pos.x != knot_positions[i-1].x || desired_pos.y != knot_positions[i-1].y {
                knot_positions[i].x = desired_pos.x;
                knot_positions[i].y = desired_pos.y;
            } else if desired_pos.x != knot_positions[i-1].x {
                knot_positions[i].x = desired_pos.x;
            } else if desired_pos.y != knot_positions[i-1].y {
                knot_positions[i].y = desired_pos.y;
            }

            if i == 1 {
                p1_visited.insert(knot_positions[i].clone(), true);
            }

            if i == 9 {
                p2_visited.insert(knot_positions[i].clone(), true);
            }
        }
    }

    println!("Part 1: {}", p1_visited.len());
    println!("Part 2: {}", p2_visited.len());
}
