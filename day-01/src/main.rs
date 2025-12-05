use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = load_puzzle();

    let start: i32 = 50;
    let mut position: i32 = start;
    let mut count: i32 = 0;

    for line in file {
        let direction = match line.chars().next() {
            Some('L') => -1,
            _ => 1,
        };
        let steps = line[1..].parse::<i32>().unwrap();

        position = ((position + (direction * steps)) % 100 + 100) % 100;

        if position == 0 {
            count += 1;
        }
    }

    println!("Count: {}", count);
}

fn load_puzzle() -> Vec<String> {
    let file = File::open("puzzle.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut list: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        list.push(line);
    }

    list
}