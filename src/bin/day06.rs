use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

struct Circular<T>{
    vec: Vec<T>,
    sorted: Vec<T>,
    idx: usize
}
 
// Circular list with a fixed size
// Overwrites oldest elements once size reached
impl<T: std::clone::Clone + std::cmp::PartialEq + std::cmp::Ord> Circular<T> {
    fn new(size: usize, initial_val: T) -> Circular<T> {
        let vec = vec![initial_val; size];
        return Circular{
            sorted: vec.clone(),
            vec: vec,
            idx: 0,
        }
    }
 
    fn push(&mut self, item: T) {
        self.vec[self.idx] = item;
        self.idx += 1;
        if self.idx >= self.vec.len() {
            self.idx = 0;
        }
 
        self.sorted.clone_from(&self.vec);
        self.sorted.sort();
    }
 
    fn contains(&self, val: &T) -> bool {
        return self.vec.contains(val);
    }
 
    fn all_unique(&self) -> bool {
        for i in 1..self.vec.len() {
            if self.sorted[i-1] == self.sorted[i] {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let file = File::open("input/06.txt").unwrap();
    let reader = BufReader::new(file);

    let chars: Vec<char> = reader.lines().next().unwrap().unwrap().chars().collect();

    let mut p1_buffer = Circular::new(4, ' ');
    let mut p2_buffer = Circular::new(14, ' ');

    let mut p1_answer = None;
    let mut p2_answer = None;

    for (idx, c) in chars.iter().enumerate() {
        p1_buffer.push(*c);
        p2_buffer.push(*c);

        if idx >= 3 && p1_answer.is_none() && p1_buffer.all_unique() {
             p1_answer = Some(idx + 1);
        }
        if idx >= 14 && p2_buffer.all_unique() {
            p2_answer = Some(idx + 1);
            break;
        }
    }

    println!("Part 1: {}", p1_answer.unwrap());
    println!("Part 2: {}", p2_answer.unwrap());
}
