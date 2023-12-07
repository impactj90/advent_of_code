use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_digits(line: &String) -> (u32, u32) {
    let mut first: u32 = 0;
    let mut last: u32 = 0;

    for char in line.chars() {
        if char.is_numeric() {
            if first == 0 {
                first = char.to_digit(10).unwrap();
            }
            last = char.to_digit(10).unwrap();
        }
    }

    println!("first: {}, last: {}", first, last);

    return (first, last);
}

fn concat_numbers(first: u32, last: u32) -> u32 {
    println!("cocnat: {}",(first * 10) + last);
    return (first * 10) + last;
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut number_list: Vec<u32> = vec![];

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        println!("{:?}", line);

        let (first, last) = find_digits(&line);
        let number = concat_numbers(first, last);

        number_list.push(number);


        println!("line length: {}", line.len());
    }

    let result: u32 = number_list.iter().sum();

    println!("result: {}", result);
}
