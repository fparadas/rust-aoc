use std::io::{self, BufRead};

enum GameInput {
    Rock,
    Paper,
    Scissors,
}

fn move_from_string(input: &str) -> GameInput {
    match input {
        "A" | "X" => GameInput::Rock,
        "B" | "Y" => GameInput::Paper,
        "C" | "Z" => GameInput::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn shape_score(input: &GameInput) -> i32 {
    match input {
        GameInput::Rock => 1,
        GameInput::Paper => 2,
        GameInput::Scissors => 3,
    }
}

fn game_score(adversary: &GameInput, player: &GameInput) -> i32 {
    shape_score(player)
        + match adversary {
            GameInput::Rock => match player {
                GameInput::Rock => 3,
                GameInput::Paper => 6,
                GameInput::Scissors => 0,
            },
            GameInput::Paper => match player {
                GameInput::Rock => 0,
                GameInput::Paper => 3,
                GameInput::Scissors => 6,
            },
            GameInput::Scissors => match player {
                GameInput::Rock => 6,
                GameInput::Paper => 0,
                GameInput::Scissors => 3,
            },
        }
}

enum Strategy {
    Win,
    Draw,
    Lose,
}

fn strat_from_string(input: &str) -> Strategy {
    match input {
        "X" => Strategy::Lose,
        "Y" => Strategy::Draw,
        "Z" => Strategy::Win,
        _ => panic!("Invalid input"),
    }
}

fn follow_stat(adversary: &GameInput, player: &Strategy) -> i32 {
    match adversary {
        GameInput::Rock => match player {
            Strategy::Win => game_score(adversary, &GameInput::Paper),
            Strategy::Draw => game_score(adversary, &GameInput::Rock),
            Strategy::Lose => game_score(adversary, &GameInput::Scissors),
        },
        GameInput::Paper => match player {
            Strategy::Win => game_score(adversary, &GameInput::Scissors),
            Strategy::Draw => game_score(adversary, &GameInput::Paper),
            Strategy::Lose => game_score(adversary, &GameInput::Rock),
        },
        GameInput::Scissors => match player {
            Strategy::Win => game_score(adversary, &GameInput::Rock),
            Strategy::Draw => game_score(adversary, &GameInput::Scissors),
            Strategy::Lose => game_score(adversary, &GameInput::Paper),
        },
    }
}

fn parse(input: &str) -> Vec<(GameInput, Strategy)> {
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut chars = x.chars();
            let adversary = move_from_string(&chars.next().unwrap().to_string());
            chars.next(); //skip whitespace
            let player = strat_from_string(&chars.next().unwrap().to_string());
            (adversary, player)
        })
        .collect()
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

pub fn run() -> io::Result<()> {
    let buff = read_lines()?;

    let total_score = parse(&buff)
        .iter()
        .map(|x| follow_stat(&x.0, &x.1))
        .sum::<i32>();

    println!("Total Score: {:#?}", total_score);
    Ok(())
}
