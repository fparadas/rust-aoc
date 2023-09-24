use std::io::{self, BufRead};

fn get_priority(first: &str, second: &str, third: &str) -> u8 {
    let mut sum = 0;
    for c in first.chars() {
        if second.contains(c) & third.contains(c) {
            if c.is_uppercase() {
                sum += (c as u8 - 'A' as u8) + 27;
            } else {
                sum += (c as u8 - 'a' as u8) + 1;
            }
            break;
        }
    }

    return sum;
}

fn read_lines() -> io::Result<Vec<Vec<String>>> {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();

    Ok(lines)
}

fn main() -> io::Result<()> {
    let lines = read_lines()?;

    println!(
        "Lines: {:#?}",
        lines
            .iter()
            .map(|x| get_priority(&x[0], &x[1], &x[2]) as u32)
            .sum::<u32>()
    );
    Ok(())
}
