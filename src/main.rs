extern crate clap;
mod day_five;
mod day_four;
mod day_one;
mod day_three;
mod day_two;

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
        1 => day_one::run()?,
        2 => day_two::run()?,
        3 => day_three::run()?,
        4 => day_four::run()?,
        5 => day_five::run(cli.part.unwrap_or(1))?,
        _ => println!("Day not implemented"),
    }

    Ok(())
}
