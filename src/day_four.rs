use std::io::{self, BufRead};

struct Elf {
    start: i32,
    end: i32,
}

fn make_elf_pair(input: &str) -> Vec<Elf> {
    input
        .split(',')
        .map(|range| {
            let arr = range
                .split('-')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Elf {
                start: arr[0],
                end: arr[1],
            }
        })
        .collect()
}

// fn fully_contain(elf_one: &Elf, elf_two: &Elf) -> bool {
//     (elf_one.start <= elf_two.start && elf_one.end >= elf_two.end)
//         || (elf_two.start <= elf_one.start && elf_two.end >= elf_one.end)
// }

fn overlap(elf_one: &Elf, elf_two: &Elf) -> bool {
    (elf_one.start <= elf_two.start && elf_one.end >= elf_two.start)
        || (elf_two.start <= elf_one.start && elf_two.end >= elf_one.start)
}

fn read_lines() -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>();

    Ok(lines)
}

pub fn run() -> io::Result<()> {
    let buff = read_lines()?;

    let contained = buff
        .iter()
        .map(|x| make_elf_pair(&x))
        .map(|x| overlap(&x[0], &x[1]) as i32)
        .sum::<i32>();

    println!("Total Score: {}", contained);
    Ok(())
}
