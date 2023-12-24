use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("Unable to open the File");
    let buffer = BufReader::new(file);

    for line in buffer.lines() {
        let line = line.expect("unable to read line");
        let (_, turns) = line.split_once(": ").unwrap();

        let mut turn_list = Vec::new();
        let turns = turns.split("; ").collect::<Vec<_>>();

        for t in turns {
            let cubes = t.split(", ").collect::<Vec<_>>();
            let mut turn = Turn::default();

            for c in cubes {
                let (amount, color) = c.split_once(" ").unwrap();

                let amount: usize = amount.parse().unwrap();

                match &color[0..1] {
                    "r" => turn.red = amount,
                    "g" => turn.green = amount,
                    "b" => turn.blue = amount,
                    _ => panic!("something is buggy"),
                }
            }
            turn_list.push(turn);
            println!("{:?}", turn_list);
        }
    }
}

#[derive(Debug, Default)]
struct Turn {
    red: usize,
    blue: usize,
    green: usize,
}
