#![warn( clippy::all, clippy::pedantic )]
use std::io::BufRead;
use std::collections::HashSet;
use adventlib::aoc;
use regex::Regex;
use std::cmp::max;
use console_bitmap::{self,BraillePatterns};

fn apply_instruction(points : HashSet<(usize,usize)>, (direction,position): (char,usize)) -> HashSet<(usize,usize)> {
    // position-coord+position, except never going negative
    match direction {
        'x' => points.into_iter().filter_map(|(x,y)|
            if x < position {
                Some((x,y))
            } else if x > position {
                Some((2*position-x,y))
            } else {
                None
            }).collect(),
        'y' => points.into_iter().filter_map(|(x,y)|
            if y < position {
                Some((x,y))
            } else if y > position {
                Some((x,2*position-y))
            } else {
                None
            }).collect(),
        _ => unreachable!()
    }
}

fn main() -> aoc::Result<()> {
    let regex = Regex::new(r"^fold along (.)=(\d+)$").unwrap();
    let reader = aoc::file("inputs/day13")?;

    let mut lines= reader.lines().map(Result::unwrap);
    let mut points : HashSet<(usize,usize)> = HashSet::new();
    for line in (&mut lines).take_while(|line| line != "") {
        let mut iter = line.split(",").map(|v| v.parse::<usize>().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        points.insert( (x,y) );
    }

    let mut instructions : Vec<(char,usize)> = Vec::new();
    for line in lines {
        let cap = regex.captures(&line).unwrap();
        let direction = cap[1].chars().next().unwrap();
        let position = cap[2].parse::<usize>().unwrap();
        instructions.push( (direction,position) );
    }

    let mut instruction_iter = instructions.into_iter();
    points = apply_instruction(points,instruction_iter.next().unwrap());
    println!("{}",points.len());

    for instruction in instruction_iter {
        points = apply_instruction(points,instruction);
    }

    let (max_x,max_y) = points.iter().fold((0,0),|(max_x,max_y),(x,y)| (max(max_x,*x),max(max_y,*y)) );
    let mut data = vec![vec![false; max_x+1]; max_y+1];
    for (x,y) in points {
        data[y][x] = true;
    }

    for line in console_bitmap::draw_from_vec::<BraillePatterns>(&data) {
        println!("{}",line);
    }

    Ok( () )
}