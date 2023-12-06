use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    for (i, line) in f.lines().enumerate() {
        let line = line.expect("Unable to read line");
        println!("{:?}", line);
        println!("{:?}", i);

        println!("line length: {}", line.len());
        for (j, char) in line.chars().enumerate() {
            if char.is_numeric() {
                print!("{:?}, ", char);
                println!("index of: {:?}", j);
            }
        }
    }
}
