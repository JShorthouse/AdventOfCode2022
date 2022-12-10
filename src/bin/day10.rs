use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

enum Instruction {
    Add(i32),
    NoOp,
}

fn main() {
    let file = File::open("input/10.txt").unwrap();
    let reader = BufReader::new(file);

    let mut cycle = 1;
    let mut register = 1;

    let mut instructions = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(" ").collect();

        match split[0] {
            "noop" => instructions.push( Instruction::NoOp ),
            "addx" => {
                let num = split[1].parse::<i32>().unwrap();
                // addx takes two cycles before value is updated,
                // simulate by adding fake noop
                instructions.push( Instruction::NoOp );
                instructions.push( Instruction::Add(num) );
            },
            _ => unreachable!(),
        }
    }

    let mut p1_ans = 0;
    let mut pixels = Vec::<bool>::new();

    for ins in &instructions {
        let scan_pos = (cycle-1) % 40;
        if scan_pos >= register-1 && scan_pos <= register+1 {
            pixels.push(true);
        } else {
            pixels.push(false);
        }

        match ins {
            Instruction::NoOp => {
            }
            Instruction::Add(num) => {
                register += num;
            }
        }
        cycle += 1;

        if (cycle + 20) % 40 == 0 {
            p1_ans += (cycle) * register;
        }

        if cycle >= 240 { break; }
    }

    println!("Part 1: {}", p1_ans);

    print!("Part 2:");
    for (idx, pixel) in pixels.iter().enumerate() {
        if idx % 40 == 0 { print!("\n"); }

        if *pixel { print!("#") } else { print!(" ") }
    }
}
