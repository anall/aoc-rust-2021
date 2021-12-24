#![warn( clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;
use aoc2021::{Packet,BitIterator};

use itertools::Itertools;

fn from_hex(nibble : char) -> u8 {
    let nibble = nibble as u8;
    if (b'0'..=b'9').contains(&nibble) {
        nibble - b'0'
    } else if (b'A'..=b'F').contains(&nibble) {
        nibble - b'A' + 10
    } else {
        panic!("invalid hex character {}",nibble)
    }
}

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day16")?;
    let line = reader.lines().next().unwrap().unwrap();
    let bytes = line.chars().into_iter().tuples().map(|(h,l)| (from_hex(h) << 4) | from_hex(l) );

    let mut iter = BitIterator::new(bytes);
    let packet = Packet::parse(&mut iter).unwrap();
    println!("{}",packet.sum_versions());

    println!("{}",packet.evaluate());

    Ok( () )
}