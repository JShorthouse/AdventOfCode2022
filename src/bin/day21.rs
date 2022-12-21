use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use std::collections::HashMap;

#[derive(Debug)]
enum OpType {
    Add,
    Minus,
    Divide,
    Multiply,
}

#[derive(Debug)]
struct MonkeyMaths {
    first: String,
    second: String,
    operator: OpType,
}

#[derive(Debug)]
enum Monkey {
    Value(i64),
    Operation(MonkeyMaths),
}

fn calc_monkey(name: &str, monkeys: &HashMap<String, Monkey>, cache: &mut HashMap<String, i64>) -> i64 {
    let monkey = monkeys.get(name).unwrap();

    match monkey {
        Monkey::Value(v) => { return *v }
        Monkey::Operation(op) => {
            let first = match cache.get(&op.first) {
                Some(v) => *v,
                None => {
                    let v = calc_monkey(&op.first, monkeys, cache);
                    cache.insert(op.first.clone(), v);
                    v
                }
            };
            let second = match cache.get(&op.second) {
                Some(v) => *v,
                None => {
                    let v = calc_monkey(&op.second, monkeys, cache);
                    cache.insert(op.second.clone(), v);
                    v
                }
            };

            return match op.operator {
                OpType::Add => first + second,
                OpType::Minus => first - second,
                OpType::Divide => first / second,
                OpType::Multiply => first * second,
            }
        }
    }
}


fn main() {
    let file = File::open("input/21.txt").unwrap();
    let reader = BufReader::new(file);

    let mut monkeys = HashMap::<String, Monkey>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let split = line.split(" ").collect::<Vec<&str>>();

        let name = split[0][0..split[0].len()-1].to_string();

        if split.len() == 2 {
           monkeys.insert(name, Monkey::Value( split[1].parse().unwrap() )); 
        } else {
            let operator = match split[2].chars().next().unwrap() {
                '+' => OpType::Add,
                '-' => OpType::Minus,
                '/' => OpType::Divide,
                '*' => OpType::Multiply,
                _ => unreachable!(),
            };

            monkeys.insert(name, Monkey::Operation( MonkeyMaths{
                first: split[1].parse().unwrap(),
                second: split[3].parse().unwrap(),
                operator,
            }));
        }
    }

    let mut cache = HashMap::<String, i64>::new();

    println!("Part 1: {}", calc_monkey("root", &monkeys, &mut cache));
}
