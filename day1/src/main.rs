use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_digits(line: &String) -> (u32, u32) {
    let mut first: u32 = 0;
    let mut last: u32 = 0;

    let mut number_in_words = "".to_string();

    for (idx, _) in line.chars().enumerate() {
        println!();
        //"abcone2threexyz"
        // TODO: take fn have wrong params
        for (_, char) in line.chars().enumerate().skip(idx).take(5) {
            println!("{:?}", line.chars().enumerate().skip(idx).take(5).collect::<Vec<_>>());
            if char.is_numeric() {
                if first == 0 {
                    first = char.to_digit(10).unwrap();
                    println!();
                    println!("added! (first): {}", first);
                    break;
                }
                last = char.to_digit(10).unwrap();
                println!();
                println!("added! (last): {}", last);
                break;

            } else {
                number_in_words.push(char);
                if is_word_a_number(&number_in_words) {
                    if first == 0 {
                        first = map_word_to_number(&number_in_words)
                            .expect("i expect a number word from 1 - 9");

                        println!();
                        println!("added! (first): {}", first);
                        number_in_words = "".to_string();
                        break;
                    }
                    last = map_word_to_number(&number_in_words)
                        .expect("i expect a number word from 1 - 9");

                    println!();
                    println!("added! (last): {}", last);
                    number_in_words = "".to_string();
                    break;
                }

                if number_in_words.len() == 5 {
                    number_in_words = "".to_string();
                    break;
                }
            }
        }
    }

    return (first, last);
}

fn is_word_a_number(number_in_words: &String) -> bool {
    if number_in_words.len() >= 3 && number_in_words.contains("one")
        || number_in_words.contains("two")
        || number_in_words.contains("three")
        || number_in_words.contains("four")
        || number_in_words.contains("five")
        || number_in_words.contains("six")
        || number_in_words.contains("seven")
        || number_in_words.contains("eight")
        || number_in_words.contains("nine")
    {
        return true;
    }

    return false;
}

fn map_word_to_number(string: &str) -> Option<u32> {
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
