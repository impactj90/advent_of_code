use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default, Debug)]
pub struct AoCGame {
    games: Vec<Vec<Turn>>,
}

impl AoCGame {
    pub fn new() -> Self {
        Self::default()
    }
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open the File");
    let buffer = BufReader::new(file);
    let mut game = AoCGame::default();

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
        }
        game.games.push(turn_list);
    }

    let mut valid_game_total = 0;
    'next_game: for (index, game) in game.games.iter().enumerate() {
        for turn in game {
            if !turn.is_valid() {
                continue 'next_game;
            }
        }
        valid_game_total += index + 1;
    }

    // pt 2 starts here
    let mut power_sum = 0;
    for game in game.games.iter() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for turn in game {
            red = red.max(turn.red);
            green = green.max(turn.green);
            blue = blue.max(turn.blue);
        }
        power_sum += red * green * blue;
    }

    println!("{}", power_sum)
}

#[derive(Debug, Default)]
struct Turn {
    red: usize,
    blue: usize,
    green: usize,
}

impl Turn {
    fn is_valid(&self) -> bool {
        return self.red <= 12 && self.green <= 13 && self.blue <= 14;
    }
}
