#![warn( clippy::all, clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;

fn get_bit_counts(data : &[Vec<u8>]) -> Vec<usize> {
    let mut iter = data.into_iter();
    let first = iter.next().unwrap().iter().map(|v| *v as usize).collect();
    iter.fold(first, |prev,cur| cur.into_iter().zip(prev).map(|(a,b)| (*a as usize)+b).collect() )
}

fn filter_and_get_last(data : &[Vec<u8>], goal_fn: fn(count : usize, count : usize) -> u8) -> usize {
    let mut working : Vec<_> = data.into_iter().cloned().collect();

    let mut idx : usize = 0;
    while working.len() > 1 {
        let counts = get_bit_counts(&working);
        let goal = goal_fn(counts[idx],working.len());
        
        working = working.into_iter().filter(|v| v[idx] == goal ).collect();
        
        idx += 1;
    }

    working[0].iter().rev().enumerate().fold(0, |prev,(idx,&val)| prev + if val == 1 { 1<<idx } else { 0 })
}

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day3")?;

    let data : Vec<_> = reader.lines().map(Result::unwrap).map(|line| {
        line.chars().into_iter().map(|ch| if ch == '1' { 1 } else { 0 } ).collect::<Vec<u8>>()
    }).collect();

    let mid = data.len()/2;
    let bit_counts = get_bit_counts(&data);
    
    let (gamma,epsilon) = bit_counts.iter().rev().enumerate().fold((0,0), |prev,(idx,&val)| 
        (prev.0 + if val < mid { 0 } else { 1<<idx },prev.1 + if val >= mid { 0 } else { 1<<idx }));
    println!("{}",gamma*epsilon);

    let oxygen = filter_and_get_last(&data, |count,total| if count >= (total-count) { 1 } else { 0 } );
    let co2 = filter_and_get_last(&data, |count,total| if count < (total-count) { 1 } else { 0 } );

    println!("{}",oxygen*co2);

    Ok( () )
}