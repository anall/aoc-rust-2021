#![warn( clippy::all, clippy::pedantic )]
use std::io::BufRead;
use std::cmp::{self,Ordering};
use adventlib::aoc;
use regex::Regex;

fn count_intersections(data : &[Vec<u8>]) -> u32 {
    let mut ct = 0;
    for row in data {
        for cell in row {
            if *cell > 1 {
                ct += 1;
            }
        }
    }
    ct
}
fn main() -> aoc::Result<()> {
    let regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let reader = aoc::file("inputs/day5")?;

    let lines : Vec<_> = reader.lines().map(Result::unwrap).map(|line| {
        let cap = regex.captures(&line).unwrap();
        let x1 = cap[1].parse::<i32>().unwrap();
        let y1 = cap[2].parse::<i32>().unwrap();
        let x2 = cap[3].parse::<i32>().unwrap();
        let y2= cap[4].parse::<i32>().unwrap();


        if x1.cmp(&x2).then(y1.cmp(&y2)) == Ordering::Greater {
            ((x2,y2),(x1,y1))
        } else {
            ((x1,y1),(x2,y2))
        }
    }).collect();

    let min_x = lines.iter().map(|((x1,_),(x2,_))| { assert!(x2 >= x1); x1 }).min().unwrap();
    let max_x = lines.iter().map(|((x1,_),(x2,_))| { assert!(x2 >= x1); x2 }).max().unwrap();
    let len_x = (max_x - min_x) as usize;

    let min_y = lines.iter().map(|((_,y1),(_,y2))| cmp::min(y1,y2)).min().unwrap();
    let max_y = lines.iter().map(|((_,y1),(_,y2))| cmp::max(y1,y2)).max().unwrap();
    let len_y = (max_y - min_y) as usize;

    println!("{} {}",min_x,max_x);
    println!("{} {}",min_y,max_y);

    let mut data = vec![vec![0_u8; len_y+1]; len_x+1];

    for ((x1,y1),(x2,y2)) in &lines {
        if x1 == x2 {
            let i = (x1 - min_x) as usize;
            for j in (*y1) ..= (*y2) {
                data[i][(j  - min_y) as usize] += 1;
            }
        } else if y1 == y2 {
            let j = (y1 - min_y) as usize;
            for i in (*x1) ..= (*x2) {
                data[(i - min_x) as usize][j] += 1;
            }
        }
    }

    let pt1 = count_intersections(&data);

    for ((x1,y1),(x2,y2)) in &lines {
        if x1 != x2 && y1 != y2 {
            assert!((x2-x1).abs() == (y2-y1).abs());
            let len = (x2-x1).abs();
            let x_dir = (x2-x1)/len;
            let y_dir = (y2-y1)/len;

            for n in 0 ..= len {
                data[(x1 + n * x_dir - min_x) as usize][(y1 + n * y_dir  - min_y) as usize] += 1;
            }
        }
    }

    
    let pt2 = count_intersections(&data);
    println!("{} {}",pt1,pt2);

    Ok( () )
}
