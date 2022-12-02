use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn calculate_score(our_move: Move, their_move: Move) -> u32 {
    let score = (our_move as u32) + 1;

    if their_move == our_move {
        // Draw
        return score + 3;
    } else {
        if our_move == Move::Rock     && their_move == Move::Scissors ||
           our_move == Move::Paper    && their_move == Move::Rock ||
           our_move == Move::Scissors && their_move == Move::Paper
        {
            // Win
            return score + 6;
        }
    }
    return score;
}

fn main() {
    let file = File::open("input/02.txt").unwrap();
    let reader = BufReader::new(file);

    let mut p1_score = 0;
    let mut p2_score = 0;

    for line in reader.lines() {
        let chars: Vec<char> = line.unwrap().chars().collect();

        let their_move = match chars[0] {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => unreachable!()
        };
        let our_move = match chars[2] {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => unreachable!()
        };

        p1_score += calculate_score(our_move, their_move);


        let p2_move;
        if chars[2] == 'Z' {
            // Win
            p2_move = match their_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            }
        } else if chars[2] == 'Y' {
            // Draw
            p2_move = their_move.clone();
        } else {
            // Lose
            p2_move = match their_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            }
        }

        p2_score += calculate_score(p2_move, their_move);
    }

    println!("Part 1: {}", p1_score);
    println!("Part 2: {}", p2_score);
}
