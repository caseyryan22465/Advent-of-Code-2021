use std::env;
use std::fs;
use std::io::{BufRead,BufReader};

fn part1(filename: &std::string::String) {
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut x = 0;
    let mut y = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut it = line.split(' ');
        
        let is_y: bool;
        let is_up: bool;
        match it.next().unwrap() {
            "up" => {
                is_y = true;
                is_up = true;
            },
            "down" => {
                is_y = true;
                is_up = false;
            },
            "forward" => {
                is_y = false;
                is_up = false;//unnecessary but wouldnt compile without
            },
            _ => break
        };

        let dist = it.next().unwrap().parse::<i32>().unwrap();
        match is_y {
            true => {
                y += dist * (if is_up {-1} else {1});
            },
            false => {
                x += dist;
            }
        }
    }
    println!("final position: depth {}, horizontal {}", y, x);
    println!("{} * {} = {}", y, x, y*x);
}

fn part2(filename: &std::string::String) {
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut it = line.split(' ');
        
        let is_y: bool;
        let is_up: bool;
        match it.next().unwrap() {
            "up" => {
                is_y = true;
                is_up = true;
            },
            "down" => {
                is_y = true;
                is_up = false;
            },
            "forward" => {
                is_y = false;
                is_up = false;//unnecessary but wouldnt compile without
            },
            _ => break
        };

        let dist = it.next().unwrap().parse::<i32>().unwrap();
        match is_y {
            true => {
                aim += dist * (if is_up {-1} else {1});
            },
            false => {
                x += dist;
                y += aim * dist;
            }
        }
    }
    println!("final position: depth {}, horizontal {}", y, x);
    println!("{} * {} = {}", y, x, y*x);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    part1(filename);
    part2(filename);
}
