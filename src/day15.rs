#![warn( clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;
use std::convert::TryFrom;

use pathfinding::prelude::dijkstra;

#[inline]
fn get_extended_score(data : &[Vec<u32>], x : usize, y : usize) -> u32 {
    let m = data.len();
    let n = data[0].len();

    let delta_m = u32::try_from(x / m).unwrap();
    let delta_n = u32::try_from(y / n).unwrap();

    let arr_x = x % m;
    let arr_y = y % n;

    ((data[arr_x][arr_y]+delta_m+delta_n-1) % 9) + 1
}

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day15")?;

    let data : Vec<Vec<u32>> = reader.lines().map(|line| line.unwrap().chars().map(|c| (c as u32) - ('0' as u32)).collect() ).collect();
    let m = data.len();
    let n = data[0].len();

    let route = dijkstra( &(0,0), 
        |(x,y)| {
            aoc::valid_neigbors_no_diagonal((*x,*y),m,n).into_iter().map(|(n_x,n_y)| ((n_x,n_y),data[n_x][n_y]))
        },
         |node| *node == (m-1,n-1) );
    let cost : u32 = route.unwrap().1;

    println!("{}",cost);

    let route_larger = dijkstra( &(0,0), 
        |(x,y)| {
            aoc::valid_neigbors_no_diagonal((*x,*y),m*5,n*5).into_iter().map(|(n_x,n_y)| ((n_x,n_y),get_extended_score(&data,n_x,n_y)))
        },
         |node| *node == ((m*5)-1,(n*5)-1) );
    let cost_larger : u32 = route_larger.unwrap().1;

    println!("{}",cost_larger);

    Ok( () )
}