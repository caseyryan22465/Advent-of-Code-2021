use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
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
