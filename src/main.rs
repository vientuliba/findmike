use std::fs;
use std::io;
use std::num;
use std::env;

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
    let mut matches: Vec<usize> = Vec::new();

    for (i, window) in data.windows(pattern.len()).enumerate() {
        if window == pattern {
            matches.push(i)
        }
    }

    return matches;
}

fn print_finds(offsets: &[usize], pattern: &str) {
    if !offsets.is_empty() {
        println!("Found matches for pattern: {}", pattern);

        for offset in offsets {
        println!("\t{}", offset);
        }
    } else {
        println!("Found no matches for pattern: {}", pattern);
    }
}

fn main(){
    if env::args().count() == 1 {
        eprintln!("Usage: findmike 'path' 'pattern'")
    }

    let path = env::args().nth(1).expect("No path provided");
    let pattern = env::args().nth(2).expect("No pattern provided");

    let file_bytes: Vec<u8> = read_binary(&path).expect("Failed to read binary");
    let pattern_bytes: Vec<u8> = parse_pattern(&pattern).expect("Failed to parse pattern");

    let offsets = find_pattern(&file_bytes, &pattern_bytes);

    print_finds(&offsets, &pattern);
}
