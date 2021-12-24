#![warn( clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day1")?;

    let numbers : Vec<_> = reader.lines().map(aoc::parse_unwrap::<u32>).collect();
    let increases =
        numbers.iter().fold( (None,0), |(prev,ct),&val| 
            (Some(val), prev.map_or(ct, |prev_v| if prev_v < val { ct + 1 } else { ct }))).1;

    let mut prev_sum : u32 = numbers[0..3].iter().sum();
    let mut sum_increased = 0;
    for i in 1 ..= numbers.len()-3 {
        let sum = numbers[i .. i+3].iter().sum();
        if sum > prev_sum {
            sum_increased += 1;
        }
        prev_sum = sum;
    }
    
    println!("{} {}",increases,sum_increased);

    Ok( () )
}
