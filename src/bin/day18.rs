use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

const OFFSETS: &[Point] = &[
    Point{ x:  1, y:  0, z:  0 },
    Point{ x: -1, y:  0, z:  0 },
    Point{ x:  0, y:  1, z:  0 },
    Point{ x:  0, y: -1, z:  0 },
    Point{ x:  0, y:  0, z:  1 },
    Point{ x:  0, y:  0, z: -1 },
];

fn main() {
    let file = File::open("input/18.txt").unwrap();
    let reader = BufReader::new(file);

    let mut cubes = HashMap::<Point, bool>::new();

    let mut min_bound = Point{ x: i32::MAX, y: i32::MAX, z: i32::MAX };
    let mut max_bound = Point{ x: -i32::MAX, y: -i32::MAX, z: -i32::MAX };

    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(",").collect();
        let point = Point {
            x: split[0].parse().unwrap(),
            y: split[1].parse().unwrap(),
            z: split[2].parse().unwrap(),
        };

        if point.x <= min_bound.x { min_bound.x = point.x - 1 }
        if point.y <= min_bound.y { min_bound.y = point.y - 1 }
        if point.z <= min_bound.z { min_bound.z = point.z - 1 }
        if point.x >= max_bound.x { max_bound.x = point.x + 1 }
        if point.y >= max_bound.y { max_bound.y = point.y + 1 }
        if point.z >= max_bound.z { max_bound.z = point.z + 1 }

        cubes.insert(point, true);
    }

    let mut num_faces = 0;

    for cube in cubes.keys() {
        for offset in OFFSETS {
            let mut test_pos = cube.clone();
            test_pos.x += offset.x;
            test_pos.y += offset.y;
            test_pos.z += offset.z;

            if !cubes.contains_key(&test_pos) {
                num_faces += 1;
            }
        }
    }

    println!("Part 1: {}", num_faces);

    let mut num_outer_faces = 0;

    let mut visited = HashMap::<Point, bool>::new();
    let mut queue = Vec::new();

    queue.push( min_bound );

    while queue.len() > 0 {
        let mut new_queue = Vec::new();

        for pos in queue {
            for offset in OFFSETS {
                let mut test_pos = pos.clone();
                test_pos.x += offset.x;
                test_pos.y += offset.y;
                test_pos.z += offset.z;

                if test_pos.x < min_bound.x || test_pos.y < min_bound.y || test_pos.z < min_bound.z ||
                   test_pos.x > max_bound.x || test_pos.y > max_bound.y || test_pos.z > max_bound.z
                {
                    continue;
                }

                if cubes.contains_key(&test_pos) {
                    num_outer_faces += 1;
                } else {
                    if !visited.contains_key(&test_pos) {
                        new_queue.push( test_pos );
                        visited.insert( test_pos, true);
                    }
                }
            }
        }

        queue = new_queue;
    }

    println!("Part 2: {}", num_outer_faces);
}
