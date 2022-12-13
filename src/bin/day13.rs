use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
 
#[derive(Debug, Clone, PartialEq)]
enum Element {
    Num(i32),
    List(Vec<Element>),
}
 
use std::cmp::Ordering;
 
fn parse_list(list: &str) -> Element {
    let mut split_list: Vec<&str> = Vec::new();
 
    let mut start_pos = None;
    let mut level = 0;
    for (idx, c) in list.chars().enumerate().skip(1) {
        if start_pos.is_none() {
            start_pos = Some(idx);
        }
 
        if c == '[' { level += 1; }
        if c == ']' { level -= 1; }
 
        if (level == 0 && c == ',') || idx == list.len()-1  {
            let split = &list[start_pos.unwrap()..idx];
            if split.len() > 0 {
                split_list.push(split);
            }
            start_pos = None;
        }
    }
 
    let mut elements = Vec::new();
 
    for list in &split_list {
        if list.starts_with('[') {
            elements.push( parse_list(list) );
        } else {
            elements.push( Element::Num( list.parse::<i32>().unwrap() ));
        }
    }
 
    return Element::List(elements);
}
 
fn compare_elements(left: &Element, right: &Element) -> Ordering {
    if let (Element::Num(left), Element::Num(right)) = (&left, &right) {
        return left.cmp(right);
    }
 
    // Convert numbers to lists
    let left_list = match left {
        Element::List(l) => l.clone(),
        Element::Num(n) => vec![ Element::Num(*n) ],
    };
    let right_list = match right {
        Element::List(l) => l.clone(),
        Element::Num(n) => vec![ Element::Num(*n) ],
    };
 
    let min_length = std::cmp::min( left_list.len(), right_list.len() );
 
    for idx in 0..min_length {
        match compare_elements(&left_list[idx], &right_list[idx]) {
            Ordering::Equal => {},
            ordering => { return ordering },
        }
    }
 
    // If reached here then all values up to min_length are the same, decide based on length
    return left_list.len().cmp(&right_list.len());
}
 
fn main() {
    let file = File::open("input/13.txt").unwrap();
    let reader = BufReader::new(file);
 
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
 
    let mut p1_ans = 0;
 
    let mut packet_list = Vec::new();
 
    for (idx, line) in lines.chunks(3).enumerate() {
        let left = parse_list( &line[0] );
        let right = parse_list( &line[1] );
 
        match compare_elements(&left, &right) {
            Ordering::Less => { p1_ans += idx+1; }
            _ => {}
        }
 
        packet_list.push( left );
        packet_list.push( right );
    }
 
    let divider_a = Element::List( vec![ Element::Num( 2 ) ]);
    let divider_b = Element::List( vec![ Element::Num( 6 ) ]);
 
    packet_list.push(divider_a.clone());
    packet_list.push(divider_b.clone());
 
    packet_list.sort_by(|a, b| compare_elements(&a, &b) );
 
    let a_idx = packet_list.iter().position(|e| e == &divider_a ).unwrap() + 1;
    let b_idx = packet_list.iter().position(|e| e == &divider_b ).unwrap() + 1;
 
    println!("Part 1: {}", p1_ans);
    println!("Part 2: {}", a_idx * b_idx);
}
