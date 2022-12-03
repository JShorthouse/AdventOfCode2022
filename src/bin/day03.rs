use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

fn char_score(input: char) -> i32 {
    if input >= 'a' && input <= 'z' {
        return 1 + input as i32 - 'a' as i32;
    } else if input >= 'A' && input <= 'Z' {
        return 27 + input as i32 - 'A' as i32;
    } else {
        unreachable!();
    }
}

fn main() {
    let file = File::open("input/03.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rucksacks = Vec::<Vec<char>>::new();

    for line in reader.lines() {
        rucksacks.push(line.unwrap().chars().collect());
    }

    let mut p1_score = 0;

    for rucksack in &rucksacks {
        let first = &rucksack[0 .. rucksack.len()/2];
        let second = &rucksack[rucksack.len()/2 .. rucksack.len()];

        for item in first {
            if second.contains(item) {
                p1_score += char_score(*item);
                break;
            }
        }
    }

    let mut p2_score = 0;

    for grouped_sacks in rucksacks.chunks(3) {
        for item in &grouped_sacks[0] {
            if grouped_sacks[1].contains(item) && grouped_sacks[2].contains(item) {
                p2_score += char_score(*item);
                break;
            }
        }
    }

    println!("Part 1: {}", p1_score);
    println!("Part 2: {}", p2_score);
}
