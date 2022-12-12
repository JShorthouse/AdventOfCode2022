use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}
use std::collections::HashMap;

const OFFSETS: [Position; 4] = [ Position{ x: 0, y: 1}, Position{ x: 1, y: 0 }, 
                                 Position{ x: -1, y: 0}, Position{ x: 0, y: -1} ];

fn main() {
    let file = File::open("input/12.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<i32>> = Vec::new();

    let mut start_pos = Position{ x: 0, y: 0 };
    let mut end_pos = Position{ x: 0, y: 0 };

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut row = Vec::new();

        for (x, char) in line.chars().enumerate() {
            match char {
                'S' => {
                    row.push(0);
                    start_pos = Position{ x: x as i32, y: y as i32 };
                },
                'E' => { 
                    row.push(25);
                    end_pos = Position{ x: x as i32, y: y as i32 };
                },
                c => row.push( c as i32 - 'a' as i32 ),
            }
        }

        grid.push(row);
    }

    let mut edges = vec![ end_pos ];
    let mut visited = HashMap::<Position, bool>::new();
    let mut start_reached = false;
    let mut distances = vec![ vec![ i32::MAX; grid[0].len() ]; grid.len() ];

    let mut distance = 0;

    while !start_reached {
        let mut new_edges = Vec::new();

        for pos in edges {
            distances[pos.y as usize][pos.x as usize] = distance;

            if pos == start_pos {
                start_reached = true;
                break;
            }

            for offset in OFFSETS {
                let next_pos = Position{ x: pos.x + offset.x, y: pos.y + offset.y };
                if next_pos.x >= 0 && next_pos.x < grid[0].len() as i32 && 
                   next_pos.y >= 0 && next_pos.y < grid.len() as i32
                {
                    if visited.contains_key(&next_pos) {
                        continue; 
                    }

                    let cur_height = grid[pos.y as usize][pos.x as usize];
                    let next_height = grid[next_pos.y as usize][next_pos.x as usize];

                    if cur_height - next_height > 1 { 
                        continue; 
                    }

                    if !new_edges.contains(&next_pos) {
                        new_edges.push(next_pos);
                    }
                }
            }
            visited.insert(pos, true);
        }

        edges = new_edges;

        distance += 1;
    }

    println!("Part 1: {}", distances[start_pos.y as usize][start_pos.x as usize]);

    let mut min_distance = i32::MAX;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                if distances[y][x] < min_distance {
                    min_distance = distances[y][x];
                }
            }
        }
    }

    println!("Part 2: {}", min_distance);
}
