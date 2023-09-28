use std::collections::HashSet;
use std::io::{self, BufRead};
use std::iter::Iterator;

pub fn run() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().lock().read_line(&mut buf)?;
    let mut char_set: HashSet<char> = HashSet::new();
    let mut char_vec: Vec<char> = Vec::new();

    for (index, c) in buf.chars().enumerate() {
        char_vec.push(c);

        if !char_set.insert(c) {
            let position = char_vec.iter().position(|&x| x == c).unwrap();
            for _ in 0..(position + 1) {
                let _ = char_set.remove(&char_vec.remove(0));
            }

            char_set.extend(char_vec.iter());
        }
        if char_vec.len() == 14 {
            println!("Marker at: {}", index + 1);
            break;
        }
    }

    Ok(())
}
