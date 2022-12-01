use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

fn main() {
    let file = File::open("input/01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut calories = Vec::<u32>::new();
    let mut current = 0;

    for line in reader.lines() {
       let line = line.unwrap();
       if line == "" {
           calories.push(current);
           current = 0;
       } else {
            current += line.parse::<u32>().unwrap();
       }
    }
    calories.push(current);

    calories.sort();
    calories.reverse();

    println!("Part 1: {}", calories[0]);
    println!("Part 2: {}", calories[0] + calories[1] + calories[2]);
}
