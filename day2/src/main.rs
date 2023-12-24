use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn find_possible_games(data: Vec<String>) -> Vec<u32> {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    return vec![0];
}

fn backup(l: String) {
    let _game_data: Vec<_> = l
        .split(&[':', ',', ';'][..])
        .skip(1)
        .map(|cube| cube.trim().to_string())
        .into_iter()
        .enumerate()
        .map(|(idx, cube)| {
            println!("{}, {:?}", idx, cube.split(" ").collect::<Vec<_>>());
        })
        .collect();
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open the File");
    let buffer = BufReader::new(file);
    let mut possible_game_number_sum: u32 = 0;

    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    for (idx, line) in buffer.lines().enumerate() {
        let games = line.expect("unable to read line");

        let mut r: Vec<u32> = vec![];
        let mut b: Vec<u32> = vec![];
        let mut g: Vec<u32> = vec![];

        let splitted_game: Vec<_> = games.split(&[',', ';', ':'][..]).skip(1).collect();

        //[" 5 red", " 9 green", " 2 blue", " 9 blue", " 6 green", " 1 red", " 8 blue", " 7 green", " 3 red"]
        println!("game: {:?}", splitted_game);

        for lines in splitted_game.iter() {
            let data = lines.trim_start().split(" ").collect::<Vec<_>>();

            if data.contains(&"red") {
                let number = data[0].parse::<u32>().unwrap();
                r.push(number)
            }
            if data.contains(&"blue") {
                let number = data[0].parse::<u32>().unwrap();
                b.push(number)
            }
            if data.contains(&"green") {
                let number = data[0].parse::<u32>().unwrap();
                g.push(number)
            }

            println!("{:?}", data);
        }

        let sum_r: u32 = r.iter().sum();
        let sum_b: u32 = b.iter().sum();
        let sum_g: u32 = g.iter().sum();

        if sum_r < red_cubes {
            possible_game_number_sum += idx
        }

        println!("{:?}", sum_r);
        println!("{:?}", sum_b);
        println!("{:?}", sum_g);

        //Game 100: 5 red, 9 green, 2 blue; 9 blue, 6 green, 1 red; 8 blue, 7 green, 3 red
    }
}
