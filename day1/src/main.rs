use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_digits(line: &String) -> (u32, u32) {
    let mut first: u32 = 0;
    let mut last: u32 = 0;
    let mut range = 5;

    for i in 0..=line.len() {
        if (i + range) > line.len() {
            if range == 0 {
                break;
            }
            range = line.len() - i;
        }

        for j in 0..=range {
            let substring = &line[i..(i + j)];

            if substring.len() == 1 && substring.chars().any(|c| c.is_digit(10)) {
                println!("i: {}, j: {} | search: {}", i, j, substring);
                if first == 0 {
                    first = substring.parse::<u32>().expect("String to be a number string");
                }
                last = substring.parse::<u32>().expect("String to be a number string")
            }

            if is_word_a_number(substring) {
                println!("i: {}, j: {} | search: {}", i, j, substring);
                if first == 0 {
                    first = parse_word_to_number(substring)
                        .expect("Expexted a word associated Number.");
                }
                last = parse_word_to_number(substring).expect("Expexted a word associated Number.");
            }
        }
    }

    return (first, last);
}

fn is_word_a_number(number_in_words: &str) -> bool {
    if number_in_words.len() >= 3 && number_in_words.eq("one")
        || number_in_words.eq("two")
        || number_in_words.eq("three")
        || number_in_words.eq("four")
        || number_in_words.eq("five")
        || number_in_words.eq("six")
        || number_in_words.eq("seven")
        || number_in_words.eq("eight")
        || number_in_words.eq("nine")
    {
        return true;
    }

    return false;
}

fn parse_word_to_number(string: &str) -> Option<u32> {
    match string {
        s if s.contains("one") => Some(1),
        s if s.contains("two") => Some(2),
        s if s.contains("three") => Some(3),
        s if s.contains("four") => Some(4),
        s if s.contains("five") => Some(5),
        s if s.contains("six") => Some(6),
        s if s.contains("seven") => Some(7),
        s if s.contains("eight") => Some(8),
        s if s.contains("nine") => Some(9),
        _ => None,
    }
}

fn concat_numbers(first: u32, last: u32) -> u32 {
    println!("concat: {}", (first * 10) + last);
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
    }

    let result: u32 = number_list.iter().sum();

    println!("result: {}", result);
}
