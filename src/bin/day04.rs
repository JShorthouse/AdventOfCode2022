use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use regex::Regex;

fn main() {
    let file = File::open("input/04.txt").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut p1_score = 0;
    let mut p2_score = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let captures = re.captures(&line).unwrap();

        let r1_start = captures[1].parse::<i32>().unwrap();
        let r1_end = captures[2].parse::<i32>().unwrap();
        
        let r2_start = captures[3].parse::<i32>().unwrap();
        let r2_end = captures[4].parse::<i32>().unwrap();

        if ( r1_start >= r2_start && r1_end <= r2_end ) || ( r2_start >= r1_start && r2_end <= r1_end ) {
            p1_score += 1;
        }

        if ( r1_start >= r2_start && r1_start <= r2_end ) || ( r1_end >= r2_start && r1_end <= r2_end ) ||
           ( r2_start >= r1_start && r2_start <= r1_end ) || ( r2_end >= r1_start && r2_end <= r1_end ) 
        {
            p2_score += 1;
        }

    }

    println!("Part 1: {}", p1_score);
    println!("Part 2: {}", p2_score);
}
