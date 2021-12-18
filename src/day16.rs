#![warn( clippy::all, clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;
use aoc2021::{Packet,BitIterator};

use itertools::Itertools;

fn from_hex(nibble : char) -> u8 {
    let nibble = nibble as u8;
    if nibble >= b'0' && nibble <= b'9' {
        nibble - b'0'
    } else if nibble >= b'A' && nibble <= b'F' {
        nibble - b'A' + 10
    } else {
        panic!("invalid hex character {}",nibble)
    }
}

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day16")?;
    let line = reader.lines().next().unwrap().unwrap();
    let bytes : Vec<u8> = line.chars().into_iter().tuples().map(|(h,l)| (from_hex(h) << 4) | from_hex(l) ).collect();

    let mut iter = BitIterator::new(bytes.into_iter());
    let packet = Packet::parse(&mut iter).unwrap();
    println!("{}",packet.sum_versions());

    println!("{}",packet.evaluate());

    Ok( () )
}