use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use regex::Regex;
 
#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}
 
impl Position {
    fn distance(&self, other: &Position) -> i64 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}
 
#[derive(Debug)]
struct Sensor {
    pos: Position,
    closest_beacon: Position,
    radius: i64,
}

impl Sensor {
    fn get_sensor_edges_at_y(&self, y: i64) -> Option<[Position; 2]> {
        let vertical_distance = (self.pos.y - y).abs();
        if vertical_distance > self.radius {
            return None;
        }

        let x_distance = self.radius - vertical_distance;

        return Some([
            Position{ x: self.pos.x - x_distance - 1, y },
            Position{ x: self.pos.x + x_distance + 1, y }
        ]);
    }
}

fn valid_beacon_pos(sensors: &Vec<Sensor>, pos: &Position) -> bool {
    for sensor in sensors {
        if sensor.closest_beacon == *pos {
            return true;
        }

        let distance = pos.distance(&sensor.pos);
        if distance <= sensor.radius {
            return false;
        }
    }
    return true;
}
 
fn main() {
    let file = File::open("input/15.txt").unwrap();
    let reader = BufReader::new(file);
 
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
 
    let mut sensors = Vec::new();
 
    for line in reader.lines() {
        let line = line.unwrap();
        let captures = re.captures(&line).unwrap();
 
        let sensor_pos = Position {
            x: captures[1].parse::<i64>().unwrap(),
            y: captures[2].parse::<i64>().unwrap(),
        };
 
        let beacon_pos = Position {
            x: captures[3].parse::<i64>().unwrap(),
            y: captures[4].parse::<i64>().unwrap(),
        };
 
        sensors.push( Sensor {
            radius: sensor_pos.distance(&beacon_pos),
            pos: sensor_pos,
            closest_beacon: beacon_pos,
        } );
    }

    let mut max_x = 0;
    let mut min_x = i64::MAX;
 
    for sensor in &sensors {
        let max = sensor.pos.x + sensor.radius;
        let min = sensor.pos.x - sensor.radius;

        if max > max_x { max_x = max };
        if min < min_x { min_x = min };
    }
 
    const TARGET_Y: i64 = 2000000;
 
    let mut p1_count = 0;
 
    for x in min_x..=max_x {
        let target_pos = Position{ x, y: TARGET_Y };

        if !valid_beacon_pos( &sensors, &target_pos ) {
            p1_count += 1;
        }
    }
 
    println!("Part 1: {}", p1_count);

    const SEARCH_MIN: i64 = 0;
    const SEARCH_MAX: i64 = 4000000;

    let mut found_pos = None;

'p2loop:
    for y in SEARCH_MIN..=SEARCH_MAX {
        for sensor in &sensors {
            let edges = sensor.get_sensor_edges_at_y(y);
            if let Some(edges) = edges {
                for pos in &edges {
                    if pos.x >= SEARCH_MIN && pos.x <= SEARCH_MAX && valid_beacon_pos( &sensors, &pos ) {
                        found_pos = Some(pos.clone());
                        break 'p2loop;
                    } 
                }
            }
         }
    }

    let found_pos = found_pos.unwrap();
    let p2_ans = found_pos.x * SEARCH_MAX + found_pos.y;
    println!("Part 2: {}", p2_ans);
}
