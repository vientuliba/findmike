use std::fs;
use std::io;
use std::num;

fn read_binary(path: &str) -> Result<Vec<u8>, io::Error> {
    let bytes = fs::read(path);
    
    return bytes;
}

fn parse_pattern(pattern: &str) -> Result<Vec<u8>, num::ParseIntError> {
    let bytes = pattern
        .split_whitespace()
        .map(|x| u8::from_str_radix(x, 16))
        .collect();
 
    return bytes;
}

fn find_pattern(data: &[u8], pattern: &[u8]) -> Vec<usize> {
    todo!();
}

fn main(){
    let bytes = read_binary("./my.bin").expect("Failed to read binary");
    let pattern = parse_pattern("FF 41 67 3A").expect("Failed to parse pattern");

    for b in &pattern {
        print!("0x{:x} ", b);
    }

    println!();

    for b in &bytes {
        print!("0x{:x} ", b);
    }
}
