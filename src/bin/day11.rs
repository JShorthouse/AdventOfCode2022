use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

#[derive(Debug, Clone)]
enum NumType {
    Value(i64),
    Old,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(NumType),
    Multiply(NumType)
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    test_operation: Operation,
    test_value: i64,
    true_monkey: usize,
    false_monkey: usize,
    times_inspected: u32,
}

fn simulate_monkeys(monkeys: &mut Vec<Monkey>, rounds: usize, part2: bool) {
    let mut common_multiple = 1; 

    if part2 {
        for monkey in monkeys.iter() {
            common_multiple *= monkey.test_value;
        }
    }

    for _ in 0..rounds {
        for id in 0..monkeys.len() {
            let mut cur_monkey = monkeys[id].clone();

            for m_item in &cur_monkey.items {
                let mut item = m_item.clone();

                cur_monkey.times_inspected += 1;

                match &cur_monkey.test_operation {
                    Operation::Add(num_type) => {
                        match num_type {
                            NumType::Old => item += item,
                            NumType::Value(n) => item += n,
                        }
                    }
                    Operation::Multiply(num_type) => {
                        match num_type {
                            NumType::Old => item *= item,
                            NumType::Value(n) => item *= n,
                        }
                    }
                }

                if part2 {
                    item %= common_multiple;
                } else {
                    item /= 3;
                }

                if item % cur_monkey.test_value == 0 {
                    monkeys[cur_monkey.true_monkey].items.push(item);
                } else {
                    monkeys[cur_monkey.false_monkey].items.push(item);
                }
            }
            cur_monkey.items.clear();

            monkeys[id] = cur_monkey;
        }
    }
}

fn calculate_score(monkeys: &Vec<Monkey>) -> u64 {
    let mut times_inspected: Vec<u32> = monkeys.iter().map(|m| m.times_inspected).collect();
    times_inspected.sort();

    return times_inspected[monkeys.len()-1] as u64 * times_inspected[monkeys.len()-2] as u64;
}

fn main() {
    let file = File::open("input/11.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut monkeys = Vec::new();

    for line_group in lines.chunks(7) {
        let starting_line = &line_group[1];
        let operation_line = &line_group[2];
        let test_line = &line_group[3];
        let true_line = &line_group[4];
        let false_line = &line_group[5];

        let items: Vec<i64> = starting_line.split(": ").nth(1).unwrap().split(", ")
                                           .map(|s| s.parse().unwrap() ).collect();

        let op: Vec<&str> = operation_line.split("old ").nth(1).unwrap().split(" ").collect();

        let op_numtype = match op[1] {
            "old" => NumType::Old,
            n => NumType::Value(n.parse().unwrap()),
        };

        let test_operation = match op[0] {
            "+" => Operation::Add(op_numtype),
            "*" => Operation::Multiply(op_numtype),
            _ => unreachable!(),
        };

        let test_value = test_line.split("by ").nth(1).unwrap().parse().unwrap();
        let true_monkey = true_line.split("monkey ").nth(1).unwrap().parse().unwrap();
        let false_monkey = false_line.split("monkey ").nth(1).unwrap().parse().unwrap();

        monkeys.push( Monkey{
            items,
            test_operation,
            test_value,
            true_monkey,
            false_monkey,
            times_inspected: 0,
        });
    }

    let mut p1_monkeys = monkeys.clone();
    let mut p2_monkeys = monkeys;

    simulate_monkeys(&mut p1_monkeys, 20, false);
    println!("Part 1: {}", calculate_score(&p1_monkeys));

    simulate_monkeys(&mut p2_monkeys, 10000, true);
    println!("Part 2: {}", calculate_score(&p2_monkeys));
}
