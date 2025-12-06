use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = load_puzzle();
    let start: i32 = 50;

    part_1(&file, start);
    part_2(&file, start);
}

fn part_1(file: &Vec<String>, start: i32) {
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

fn part_2(file: &Vec<String>, start: i32) {
    let mut position: i32 = start;
    let mut count: i32 = 0;

    for line in file {
        let direction = match line.chars().next() {
            Some('L') => -1,
            _ => 1,
        };
        let steps = line[1..].parse::<i32>().unwrap();

        let mut steps_until_zero: i32 = if direction == 1 {
            (100 - position) % 100
        } else {
            position % 100
        };

        if steps_until_zero == 0 {
            steps_until_zero = 100;
        }

        if steps_until_zero <= steps {
            let full_rotations = (steps - steps_until_zero) / 100;
            count += 1 + full_rotations;
        }

        position = ((position + (direction * steps)) % 100 + 100) % 100;
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