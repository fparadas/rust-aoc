extern crate regex;

use regex::Regex;

use std::io::{self, BufRead};

type HanoiTower = Vec<char>;

#[derive(Debug)]
struct MoveInstruction {
    ammount: usize,
    from: usize,
    to: usize,
}

fn parse_tower(input: &str) -> Vec<HanoiTower> {
    let mut towers: Vec<HanoiTower> = Vec::new();

    let lines: Vec<&str> = input.split("\n").collect();
    lines[..lines.len() - 1].iter().for_each(|line| {
        if towers.is_empty() {
            for _i in (1..=line.len()).step_by(4) {
                towers.push(Vec::new());
            }
        }
        for i in (1..=line.len()).step_by(4) {
            let index = (i - 1) / 4;
            let ch = line.chars().nth(i).unwrap();
            if !ch.is_whitespace() {
                towers[index].push(ch);
            }
        }
    });

    towers
        .iter()
        .map(|tower| {
            let mut tower = tower.clone();
            tower.reverse();
            tower
        })
        .collect()
}

fn parse_intructions(input: &str) -> Vec<MoveInstruction> {
    let re = Regex::new(r"(\b\d+\b)").unwrap();
    input
        .split("\n")
        .map(|line| {
            re.find_iter(line)
                .filter_map(|x| x.as_str().parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .filter_map(|inner_vec| {
            if inner_vec.len() == 3 {
                Some(MoveInstruction {
                    ammount: inner_vec[0],
                    from: inner_vec[1],
                    to: inner_vec[2],
                })
            } else {
                None
            }
        })
        .collect()
}

fn parse_input(input: &str) -> (Vec<MoveInstruction>, Vec<HanoiTower>) {
    let game = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = parse_intructions(game[1]);
    let towers = parse_tower(game[0]);

    (instructions, towers)
}

fn read_lines() -> io::Result<String> {
    let stdin = io::stdin();
    let mut buff = String::new();

    for line in stdin.lock().lines() {
        buff.push_str(&line?);
        buff.push_str("\n");
    }

    Ok(buff)
}

fn play_hanoi(instructions: &Vec<MoveInstruction>, game_towers: &Vec<HanoiTower>) -> String {
    let mut towers = game_towers.clone();
    for instruction in instructions {
        let mut i = instruction.ammount;
        while i > 0 {
            let mut from = towers[instruction.from - 1].clone();
            let mut to = towers[instruction.to - 1].clone();

            let disk = from.pop().unwrap();
            to.push(disk);

            towers[instruction.from - 1] = from;
            towers[instruction.to - 1] = to;

            i -= 1;
        }
    }

    towers
        .iter()
        .filter_map(|tower| tower.last())
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn play_part_two(instructions: &Vec<MoveInstruction>, game_towers: &Vec<HanoiTower>) -> String {
    let mut towers = game_towers.clone();
    for instruction in instructions {
        let mut from = towers[instruction.from - 1].clone();
        let mut to = towers[instruction.to - 1].clone();
        let end = from.len();

        to.extend(from.drain((end - instruction.ammount)..end));

        towers[instruction.from - 1] = from;
        towers[instruction.to - 1] = to;
    }

    towers
        .iter()
        .filter_map(|tower| tower.last())
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn run(part: u8) -> io::Result<()> {
    let buff = read_lines()?;

    let (instructions, towers) = parse_input(&buff);

    let game = match part {
        1 => play_hanoi(&instructions, &towers),
        2 => play_part_two(&instructions, &towers),
        _ => String::from("Error"),
    };

    println!("Game: {:#?}", game);

    Ok(())
}
