use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use std::collections::VecDeque;
use regex::Regex;

struct Instruction {
    amount: u32,
    from: u32,
    to: u32,
}

fn main() {
    let file = File::open("input/05.txt").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut reading_crates = true;
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut instructions = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with(" 1") || line.len() == 0 {
            reading_crates = false;
            continue;
        }
       
        if reading_crates {
            let chars: Vec<char> = line.chars().collect();

            for (idx, chunk) in chars.chunks(4).enumerate() {
                // Split crates as '[A] ' and extract name
                let crate_name = chunk[1];

                if crate_name != ' ' {
                    while stacks.len() < idx+1 {
                        stacks.push(VecDeque::new());
                    }
                    stacks[idx].push_back(crate_name);
                }
            }
        } else {
            let captures = re.captures(&line).unwrap();
            let amount = captures[1].parse::<u32>().unwrap();
            let from = captures[2].parse::<u32>().unwrap();
            let to = captures[3].parse::<u32>().unwrap();

            instructions.push( Instruction{ amount, from, to } );
        }
    }

    let mut p1_stacks = stacks;
    let mut p2_stacks = p1_stacks.clone();

    for ins in &instructions {
        // Part 1
        for _ in 0..ins.amount {
            let c = p1_stacks[(ins.from-1) as usize].pop_front().unwrap();
            p1_stacks[(ins.to-1) as usize].push_front(c);
        }

        // Part 2
        let mut removed = VecDeque::new();

        for _ in 0..ins.amount {
            removed.push_front( p2_stacks[(ins.from-1) as usize].pop_front().unwrap() );
        }
        for c in removed {
            p2_stacks[(ins.to-1) as usize].push_front(c);
        }
    }

    let mut p1_ans = String::new();
    let mut p2_ans = String::new();

    for i in 0..p1_stacks.len() {
        p1_ans.push( p1_stacks[i][0] );
        p2_ans.push( p2_stacks[i][0] );
    }

    println!("Part 1: {}", p1_ans);
    println!("Part 2: {}", p2_ans);
}
