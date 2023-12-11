use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let red = 12;
    let blue = 14;
    let green = 13;

    let file = File::open("input.txt").expect("Unable to open the File");
    let buffer = BufReader::new(file);

    for (idx, line) in buffer.lines().enumerate() {
        let line = line.expect("unable to read line");
        // extract to ds

        // "Game 97: 6 red, 1 blue, 7 green; 2 blue, 5 red, 7 green; 8 red, 3 blue, 6 green; 6 green, 1 red, 3 blue; 5 red, 2 blue, 14 green; 3 green, 6 red, 6 blue"
        // let v: Vec<&str> = "2020-11-03 23:59".split(&['-', ' ', ':', '@'][..]).collect();
        let g: Vec<&str> = line.split(&[',', ':', ';'][..]).skip(1).collect();
        println!("idx: [{}] - {:?}", idx + 1, g);
    }
}
