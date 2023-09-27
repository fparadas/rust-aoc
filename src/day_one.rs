use std::io::{self, BufRead};

pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buff = String::new();

    for line in stdin.lock().lines() {
        buff.push_str(&line?);
        buff.push_str("\n");
    }
    let elves = parse(&buff);
    let top = find_max(&elves);

    println!("Top Calories: {:#?}", top);
    println!("Top Three: {:#?}", find_top_three(&elves));
    Ok(())
}

fn parse(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect()
}

fn find_max(input: &Vec<i32>) -> i32 {
    input.iter().max().unwrap().clone()
}

fn find_top_three(input: &Vec<i32>) -> i32 {
    let mut arr = input.clone();
    arr.sort();
    arr.reverse();
    arr.truncate(3);
    arr.iter().sum::<i32>()
}
