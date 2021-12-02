use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

// How many lines are greater than the last
fn part1(filename: &std::string::String){
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut ct: u16 = 0;
    let mut last: u16 = 65535;//max u16
    for line in reader.lines() {
        let line = line.unwrap();
        let cur_measurement = line.parse::<u16>().unwrap();
        if cur_measurement > last {
            ct += 1;
        }
        last = cur_measurement;
    }
    println!("Measurements larger than last: {}", ct);
}

fn part2(filename: &std::string::String){
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut data: Vec<u32> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let cur_measurement = line.parse::<u32>().unwrap();
        data.push(cur_measurement);
    }
    let iter = data.iter();
    let mut prev_sum: u32 = 4294967295;
    let mut ct: u16 = 0;
    for (a,b,c) in iter.tuple_windows(){
        let sum: u32 = a + b + c;
        if sum > prev_sum {
            ct += 1;
        }
        prev_sum = sum;
    }
    println!("three-measurement sliding windows > last: {}", ct);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    part1(filename);
    part2(filename);
}
