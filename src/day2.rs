#![warn( clippy::all, clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs-sample/day2")?;

    let numbers : Vec<_> = reader.lines();
}