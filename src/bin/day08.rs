use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

fn main() {
    let file = File::open("input/08.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid = Vec::<Vec<u32>>::new();

    const OFFSETS: [[i32; 2]; 4] = [[0, 1], [1, 0], [-1, 0], [0, -1]];

    for line in reader.lines() {
        let mut cur_row = Vec::new();
        for ch in line.unwrap().chars() {
            cur_row.push( ch as u32 - '0' as u32 );
        }
        grid.push(cur_row);
    }

    let mut visibility = vec![vec![false; grid[0].len()]; grid.len()];
    let mut scores = vec![vec![0_u32; grid[0].len()]; grid.len()];

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut score = 1;

            for offset in OFFSETS {
                let mut cur_x = x as i32;
                let mut cur_y = y as i32;
                let height = grid[y][x];

                let mut visible = true;

                cur_x += offset[0];
                cur_y += offset[1];

                let mut this_score = 0;

                while cur_x >= 0 && cur_x < grid[0].len() as i32 && cur_y >= 0 && cur_y < grid.len() as i32 {
                    this_score += 1;

                    if grid[cur_y as usize][cur_x as usize] >= height {
                        visible = false;
                        break;
                    }

                    cur_x += offset[0];
                    cur_y += offset[1];
                }

                if visible {
                    visibility[y][x] = true;
                }

                score *= this_score;
            }
            scores[y][x] = score;
        }
    }

    let mut num_visible = 0;
    let mut highest_score = 0;

    for row in &visibility {
        for visible in row {
            if *visible { num_visible += 1; }
        }
    }

    for row in &scores {
        for score in row {
            if *score > highest_score { highest_score = *score }
        }
    }

    println!("Part 1: {}", num_visible);
    println!("Part 2: {}", highest_score);
}
