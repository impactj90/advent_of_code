use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_digits(line: &String) -> (u32, u32) {
    let mut first: u32 = 0;
    let mut last: u32 = 0;

    let mut number_in_words = "".to_string();

    for (idx, _) in line.chars().enumerate() {
        for (_, char) in line.chars().enumerate().skip(idx).take(idx + 5) {
            if char.is_numeric() {
                if first == 0 {
                    first = char.to_digit(10).unwrap();
                }
                last = char.to_digit(10).unwrap();
            } else {
                number_in_words.push(char);

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
                    if first == 0 {
                        first = map_word_to_number(&number_in_words)
                            .expect("i expect a number word from 1 - 9");

                        number_in_words = "".to_string();
                        break;
                    }
                    last = map_word_to_number(&number_in_words)
                        .expect("i expect a number word from 1 - 9");

                    number_in_words = "".to_string();
                    break;
                } else if number_in_words.len() == 5 {
                    number_in_words = "".to_string();
                }
            }
        }
    }

    println!("first: {}, last: {}", first, last);

    return (first, last);
}

fn map_word_to_number(string: &str) -> Option<u32> {
    println!("current word: {}", string);
    Some(match string {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => return None, // Handle the case where the input is not a valid word
    })
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
