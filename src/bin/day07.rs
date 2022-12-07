use std::io::{
    prelude::*,
    BufReader,
};

#[derive(Debug, Clone)]
enum ChildType {
    File(usize),
    Directory(usize)
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent_id: usize,
    child_ids: Vec<ChildType>,
    size: usize,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

fn calc_dir_sizes(dirs: &mut Vec<Directory>, files: &Vec<File>, dir_id: usize) -> usize {
    let mut cur_size = 0;

    for child in dirs[dir_id].child_ids.clone() {
        match child {
            ChildType::Directory(id) => {
                cur_size += calc_dir_sizes(dirs, files, id);
            }
            ChildType::File(id) => {
                cur_size += files[id].size;
            },
        }
    }

    dirs[dir_id].size = cur_size;
    return cur_size;
}

fn main() {
    let file = std::fs::File::open("input/07.txt").unwrap();
    let reader = BufReader::new(file);

    let mut files = Vec::<File>::new();
    let mut directories = Vec::<Directory>::new();

    directories.push( Directory{ name: "/".to_string(), parent_id: 0, child_ids: Vec::new(), size: 0 } );

    let mut cur_directory_id: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with("$") {
            if line.starts_with("$ cd") {
                let new_dir_name = &line[5..];

                if new_dir_name == ".." {
                    cur_directory_id = directories[cur_directory_id].parent_id;
                } else {
                    for child in &directories[cur_directory_id].child_ids {
                        if let ChildType::Directory(id) = child {
                            if directories[*id].name == new_dir_name {
                                cur_directory_id = *id;
                                break;
                            }
                        }
                    }
                }
            }
        } else {
            // Processing ls list
            let split: Vec<&str> = line.split(" ").collect();
            let size = split[0];
            let name = split[1];

            if size == "dir" {
                let new_dir = Directory {
                    name: name.to_string(),
                    parent_id: cur_directory_id, 
                    child_ids: Vec::new(),
                    size: 0,
                };

                directories.push(new_dir);
                let new_id = directories.len() - 1;
                directories[cur_directory_id].child_ids.push(ChildType::Directory(new_id));
            } else {
                let new_file = File{
                    name: name.to_string(),
                    size: size.parse::<usize>().unwrap()
                };

                files.push(new_file);
                let file_id = files.len() - 1;
                directories[cur_directory_id].child_ids.push(ChildType::File(file_id));
            }
        }
    }

    calc_dir_sizes(&mut directories, &files, 0);

    const FILESYSTEM_LIMIT: usize = 70_000_000;
    const UPDATE_SIZE: usize = 30_000_000;
    let cur_space_free = FILESYSTEM_LIMIT - directories[0].size;
    let extra_space_needed = UPDATE_SIZE - cur_space_free;

    let mut p1_answer = 0;

    for dir in &directories {
        if dir.size <= 100_000 {
            p1_answer += dir.size;
        }
    }

    directories.sort_by(|a, b| a.size.cmp(&b.size) );

    let mut p2_answer = 0;

    for dir in &directories {
        if dir.size >= extra_space_needed {
            p2_answer = dir.size;
            break;
        }
    }

    println!("Part 1: {}", p1_answer);
    println!("Part 2: {}", p2_answer);
}
