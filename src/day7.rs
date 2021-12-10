#![warn( clippy::all, clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;
use itertools::Itertools;
use itertools::MinMaxResult;

fn cost_move_to(numbers : &[i32], goal : i32) -> i32 {
    numbers.iter().map(|num| (num-goal).abs()).sum()
}

fn cost_move_to_series(numbers : &[i32], goal : i32) -> i32 {
    numbers.iter().map(|num| (1..=(num-goal).abs()).sum::<i32>() ).sum()
}

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day7")?;
    let numbers : Vec<_> = reader.lines().next().unwrap().unwrap().split(",").map(|v| v.parse::<i32>().unwrap() ).collect();

    let (min,max) = match numbers.iter().minmax() {
        MinMaxResult::OneElement(x) => (*x,*x),
        MinMaxResult::MinMax(x,y) => (*x,*y),
        _ => unreachable!()
    };

    let best_goal_pt1 = (min ..= max).min_by_key(|goal| cost_move_to(&numbers,*goal)).unwrap();
    
    println!("{}",cost_move_to(&numbers,best_goal_pt1));

    let best_goal_pt2 = (min ..= max).min_by_key(|goal| cost_move_to_series(&numbers,*goal)).unwrap();
    
    println!("{}",cost_move_to_series(&numbers,best_goal_pt2));

    Ok( () )
}