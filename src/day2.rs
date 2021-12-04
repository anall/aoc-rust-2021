#![warn( clippy::all, clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;
use regex::Regex;

fn main() -> aoc::Result<()> {
    let regex = Regex::new(r"^(.+?) (\d+)$").unwrap();
    let reader = aoc::file("inputs/day2")?;

    let instructions : Vec<_> = reader.lines().map(Result::unwrap).map(|line| {
        let cap = regex.captures(&line).unwrap();
        let distance = cap[2].parse::<i32>().unwrap();
        match &cap[1] {
            "forward" => (distance,0),
            "down" => (0,distance),
            "up" => (0,-distance),
            _ => unimplemented!()
        }
    }).collect();

    let pt1 = instructions.iter().fold( (0,0), |prev,cur| (prev.0+cur.0,prev.1+cur.1) );
    let pt2 = instructions.iter().fold( (0,0,0), |prev,cur| (prev.0+cur.0,prev.1+cur.0*prev.2,prev.2+cur.1) );

    println!("{} {} {:?}",pt1.0*pt1.1,pt2.0*pt2.1,pt2);

    Ok( () )
}