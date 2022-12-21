use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

const DECRYPTION_KEY: i64 = 811589153;

#[derive(Debug, Copy, Clone)]
struct Element {
    id: usize,
    num: i64,
}

fn solve(elems: &Vec<Element>, key: i64, rounds: usize) -> i64 {
    let mut elements = elems.clone();

    for elem in &mut elements {
        elem.num *= key;
    }

    let mut working_list = elements.clone();

    for _ in 0..rounds {
        for elem in &elements {
            let cur_idx = working_list.iter().position(|e| e.id == elem.id ).unwrap();
            let new_idx = (cur_idx as i64 + elem.num).rem_euclid(elements.len() as i64 - 1) as usize;

            working_list.remove(cur_idx);
            working_list.insert(new_idx, *elem);
        }
    }

    let zero_idx = working_list.iter().position(|e| e.num == 0).unwrap();

    let mut result = working_list[(zero_idx + 1000).rem_euclid(elements.len())].num;
    result += working_list[(zero_idx + 2000).rem_euclid(elements.len())].num;
    result += working_list[(zero_idx + 3000).rem_euclid(elements.len())].num;

    return result;
}

fn main() {
    let file = File::open("input/20.txt").unwrap();
    let reader = BufReader::new(file);

    let mut elements = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        elements.push( Element{
            id: idx,
            num: line.parse().unwrap(),
        });
    }

    println!("Part 1: {}", solve(&elements, 1, 1));
    println!("Part 2: {}", solve(&elements, DECRYPTION_KEY, 10));
}
