use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

const AIR: u8 = 0;
const ROCK: u8 = 1;
const SAND: u8 = 2;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct OffsetGrid {
    data: Vec<u8>,
    width: usize,
    x_offset: usize,
    y_offset: usize,
}

impl OffsetGrid {
    fn new(width: usize, height: usize, x_offset: usize, y_offset: usize) -> OffsetGrid {
        return OffsetGrid {
            data: vec![ AIR; width * height ],
            width,
            x_offset,
            y_offset,
        }
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        let x = x - self.x_offset;
        let y = y - self.y_offset;
        self.data[y*self.width + x] = val;
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        let x = x - self.x_offset;
        let y = y - self.y_offset;
        self.data[y*self.width + x]
    }

    //fn print(&self) {
    //    for row in self.data.chunks(self.width) {
    //        for val in row {
    //            print!("{}", val);
    //        }
    //        print!("\n");
    //    }
    //}
}

fn main() {
    let file = File::open("input/14.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rock_paths = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let points = line.split(" -> ");

        let mut path = Vec::new();

        for p in points {
            let split: Vec<&str> = p.split(",").collect();

            path.push( Point{ x: split[0].parse().unwrap(), y: split[1].parse().unwrap() } );
        }

        rock_paths.push( path );
    }

    // Find grid bounds
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for path in &rock_paths {
        for point in path {
            if point.x > max_x { max_x = point.x; }
            if point.x < min_x { min_x = point.x; }
            if point.y > max_y { max_y = point.y; }
            if point.y < min_y { min_y = point.y; }
        }
    }

    let mut grid;
    {
        let width = (max_x - min_x + 300) as usize;
        let height = (max_y - min_y + 20) as usize;
        let grid_min_x = std::cmp::max(min_x as i32 - 150, 0 as i32) as usize;
        grid = OffsetGrid::new( width, height, grid_min_x, 0 );
    }

    // Draw lines
    for path in &rock_paths {
        for line in path.windows(2) {
            let mut start = &line[0];
            let mut end = &line[1];

            if start.x == end.x {
                if start.y > end.y {
                    start = &line[1];
                    end = &line[0];
                }
                for y in start.y..=end.y {
                    grid.set(start.x, y, ROCK);
                }
            } else {
                if start.x > end.x {
                    start = &line[1];
                    end = &line[0];
                }
                for x in start.x..=end.x {
                    grid.set(x, start.y, ROCK);
                }
            }
        }
    }

    let floor_level = max_y + 2;
    let mut blocked = false;
    let mut sand_count = 0;

    let mut p1_ans = 0;

    while !blocked {
        let mut sand_pos = Point{ x: 500, y: 0 };
        let mut sand_settled = false;

        while !sand_settled {
            let mut moved = false;
            if grid.get(sand_pos.x, sand_pos.y + 1) == 0 {
                sand_pos.y += 1;
                moved = true;
            } else if grid.get(sand_pos.x - 1, sand_pos.y + 1) == 0 {
                sand_pos.y += 1;
                sand_pos.x -= 1;
                moved = true;
            } else if grid.get(sand_pos.x + 1, sand_pos.y + 1) == 0 {
                sand_pos.y += 1;
                sand_pos.x += 1;
                moved = true;
            }

            if !moved || sand_pos.y + 1 == floor_level {
                grid.set(sand_pos.x, sand_pos.y, SAND);
                sand_settled = true;
                sand_count += 1;

                if p1_ans == 0 && sand_pos.y + 1 == floor_level {
                    p1_ans = sand_count;
                }
            }
        }

        if sand_pos.y == 0 {
            blocked = true;
        }
    }

    println!("Part 1: {}", p1_ans);
    println!("Part 2: {}", sand_count);
}
