#![warn( clippy::all, clippy::pedantic )]
use std::hash::Hash;
use std::io::BufRead;
use std::collections::HashMap;
use adventlib::aoc;
use itertools::Itertools;
use regex::Regex;

fn run_cycle(polymer : &(HashMap<(char,char),usize>,char), conversions : &HashMap<(char,char),char>) -> (HashMap<(char,char),usize>,char) {
    let mut out : HashMap<(char,char),usize> = HashMap::new();

    for (&(a,b),&ct) in &polymer.0 {
        let replacement = *conversions.get( &(a,b) ).unwrap();
        *out.entry( (a,replacement) ).or_default() += ct;
        *out.entry( (replacement,b) ).or_default() += ct;
    }

    (out,polymer.1)
}

fn frequency_count(polymer : &(HashMap<(char,char),usize>,char)) -> HashMap<char,usize> {
    let mut out : HashMap<char,usize> = HashMap::new();
    for ((a,_),ct) in &polymer.0 {
        // only counting a, to not double-count, the last character is handled below
        *out.entry(*a).or_default() += ct;
    }
    *out.entry( polymer.1 ).or_default() += 1;
    out
}

fn main() -> aoc::Result<()> {
    let regex = Regex::new("^(.+?) -> (.+?)$").unwrap();
    let reader = aoc::file("inputs/day14")?;
    let mut lines = reader.lines().map(Result::unwrap);

    let starting_polymer = lines.next().unwrap();
    
    lines.next(); // eat blank line

    let conversions = { 
        let mut out : HashMap<(char,char),char> = HashMap::new();
        for line in lines {
            let cap= regex.captures(&line).unwrap();
            let mut first = cap[1].chars();
            let a = first.next().unwrap();
            let b = first.next().unwrap();
            assert!( first.next().is_none() );
            let mut second = cap[2].chars();
            out.insert( (a,b), second.next().unwrap() );
            assert!( second.next().is_none() );
        }
        out
    };

    // convert starting polymer into pairs
    let paired_polymer = {
        let mut last_char = ' ';
        let mut out : HashMap<(char,char),usize> = HashMap::new();
        for (a,b) in starting_polymer.chars().tuple_windows() {
            *out.entry( (a,b) ).or_default() += 1;
            last_char = b;
        }
        (out,last_char)
    };

    let mut polymer = paired_polymer.clone();
    for _ in 0 .. 10 {
        polymer = run_cycle(&polymer, &conversions);
    }

    let frequency = frequency_count(&polymer);
    let min_freq = frequency.iter().min_by_key(|(_,freq)| **freq).unwrap();
    let max_freq = frequency.iter().max_by_key(|(_,freq)| **freq).unwrap();

    println!("{}",*max_freq.1-*min_freq.1);

    for _ in 10 .. 40 {
        polymer = run_cycle(&polymer, &conversions);
    }

    let frequency = frequency_count(&polymer);
    let min_freq = frequency.iter().min_by_key(|(_,freq)| **freq).unwrap();
    let max_freq = frequency.iter().max_by_key(|(_,freq)| **freq).unwrap();

    println!("{}",*max_freq.1-*min_freq.1);

    Ok( () )
}