use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Left,
    Right,
    Move(u32),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn change_dir(cur_dir: Direction, ins: Instruction) -> Direction {
   return match ins {
       Instruction::Move(_) => cur_dir,
       Instruction::Left => {
            match cur_dir {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            }
       }
       Instruction::Right => {
           match cur_dir {
               Direction::Up => Direction::Right,
               Direction::Right => Direction::Down,
               Direction::Down => Direction::Left,
               Direction::Left => Direction::Up,
           }
       }
   }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum GridElem {
    Void,
    Valid,
    Wall,
}

struct Grid {
    data: Vec<Vec<GridElem>>,
}

impl Grid {
    fn get(self: &mut Grid, pos: Position) -> GridElem {
        if pos.x < 0 || pos.x < 0 {
            return GridElem::Void;
        }

        let x = pos.x as usize;
        let y = pos.y as usize;

        if y >= self.data.len() || x >= self.data[y].len() {
            return GridElem::Void;
        }

        return self.data[y][x];
    }
}

fn main() {
    let file = File::open("input/22.txt").unwrap();
    let reader = BufReader::new(file);

    let mut parsing_map = true;

    let mut map = Vec::<Vec<GridElem>>::new();
    let mut instructions = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            parsing_map = false;
            continue;
        }

        if parsing_map {
            let elements = line.chars().map(|c| match c {
                ' ' => GridElem::Void,
                '.' => GridElem::Valid,
                '#' => GridElem::Wall,
                _ => unreachable!(),
            }).collect();
            map.push( elements );
        } else {
            let mut cur_num = String::new();
            for ch in line.chars() {
                match ch {
                    '0'..='9' => {
                        cur_num.push(ch);
                    },
                    'L' | 'R' => {
                        if cur_num.len() > 0 {
                            let num: u32 = cur_num.parse().unwrap();
                           instructions.push( Instruction::Move(num) );
                            cur_num.clear();
                        }

                        if ch == 'L' {
                            instructions.push( Instruction::Left );
                        } else {
                            instructions.push( Instruction::Right );
                        }
                    },
                    _ => unreachable!(),
                }
            }

            if cur_num.len() > 0 {
               let num: u32 = cur_num.parse().unwrap();
                instructions.push( Instruction::Move(num) );
            }
        }
    }

    let mut grid = Grid{ data: map };

    let mut cur_dir = Direction::Right;

    let start_x = grid.data[0].iter().position(|v| *v == GridElem::Valid).unwrap();
    let mut cur_pos = Position{ x: start_x as i32, y: 0 };

    for ins in &instructions {
        cur_dir = change_dir(cur_dir, *ins);

        if let Instruction::Move(dist) = ins {
            for _ in 0..*dist {
                let mut new_pos = cur_pos.clone();
                match cur_dir {
                    Direction::Up => { new_pos.y -= 1 },
                    Direction::Right => { new_pos.x += 1 },
                    Direction::Down => { new_pos.y += 1 },
                    Direction::Left => { new_pos.x -= 1 },
                }

                match grid.get(new_pos) {
                    GridElem::Valid => { cur_pos = new_pos }
                    GridElem::Wall => break,
                    GridElem::Void => {
                        // Needs wrapping
                        let mut scan_pos = cur_pos.clone();
                        match cur_dir {
                            Direction::Down => {
                                while grid.get( Position{ x: scan_pos.x, y: scan_pos.y - 1 } ) != GridElem::Void {
                                    scan_pos.y -= 1;
                                }
                            },
                            Direction::Up => {
                                while grid.get( Position{ x: scan_pos.x, y: scan_pos.y + 1 } ) != GridElem::Void {
                                    scan_pos.y += 1;
                                }
                            },
                            Direction::Left => {
                                while grid.get( Position{ x: scan_pos.x + 1, y: scan_pos.y } ) != GridElem::Void {
                                    scan_pos.x += 1;
                                }
                            },
                            Direction::Right => {
                                while grid.get( Position{ x: scan_pos.x - 1, y: scan_pos.y } ) != GridElem::Void {
                                    scan_pos.x -= 1;
                                }
                            },
                        }

                        if grid.get(scan_pos) == GridElem::Valid {
                            cur_pos = scan_pos;
                        } else { // Wall
                            break;
                        }
                    }
                }
            }
        }
    }

    let mut p1_score = (cur_pos.x + 1)*4 + (cur_pos.y + 1)*1000;
    p1_score += match cur_dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    println!("Part 1: {}", p1_score);
}
