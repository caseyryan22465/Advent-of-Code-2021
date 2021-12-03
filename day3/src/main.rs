use std::env;
use std::fs;
use std::io::{BufRead,BufReader};


fn part1(filename: &std::string::String) {
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut bit_ct: [u16; 12] = [0,0,0,0,0,0,0,0,0,0,0,0];
    for line in reader.lines(){
        let line = line.unwrap();
        for (i, b) in line.chars().enumerate(){
            match b {
                '1' => {
                    bit_ct[i] += 1;
                },
                _ => (),//Do nothing
            }
        }
    }
    for i in &mut bit_ct {
        *i = if *i > 500 {1} else {0};
    }
    let mut epsilon: [u16; 12] = [0,0,0,0,0,0,0,0,0,0,0,0];//doing iter.map.collect doesnt work bc array fixed size
    for (i, b) in bit_ct.iter().enumerate() {
        epsilon[i] = if *b == 1 {0} else {1};
    }
    //println!("gamma:  {:?}, dec: {}", bit_ct, bits_to_int(&bit_ct));
    //println!("epsilon:{:?}, dec: {}", epsilon, bits_to_int(&epsilon));
    println!("gamma*epsilon = {}", (bits_to_int(&bit_ct) as u32 * bits_to_int(&epsilon) as u32));
}

fn bits_to_int(bits: &[u16]) -> u16{//fold is better way to do this
    let mut result: u16 = 0;
    for b in bits{
        result <<= 1;
        result ^= b;
    }
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    part1(filename);
}
