mod days;

extern crate clap;

use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    /// The day to run
    #[clap(short, long)]
    day: u8,

    #[clap(short, long)]
    part: Option<u8>,
}

fn main() -> io::Result<()> {
    let cli = CLI::parse();

    match cli.day {
        1 => days::day_one::run()?,
        2 => days::day_two::run()?,
        3 => days::day_three::run()?,
        4 => days::day_four::run()?,
        5 => days::day_five::run(cli.part.unwrap_or(1))?,
        6 => days::day_six::run()?,
        _ => println!("Day not implemented"),
    }

    Ok(())
}
