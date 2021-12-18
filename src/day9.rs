#![warn( clippy::all, clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;
use std::collections::HashSet;

fn walk_basin(data : &[Vec<u32>], i_init : usize, j_init : usize) -> Vec<u32> {
    let m = data.len();
    let n = data[0].len();
    let mut basin : Vec<u32> = Vec::new();

    let mut to_visit : Vec<(usize,usize)> = vec![(i_init,j_init)];
    let mut seen : HashSet<(usize,usize)> = HashSet::new();
    seen.insert( (i_init,j_init) );

    while let Some((i,j)) = to_visit.pop() {
        basin.push(data[i][j]);

        if i > 0 && ! seen.contains( &(i-1,j) ) && data[i-1][j] != 9 {
            seen.insert( (i-1,j) );
            to_visit.push( (i-1,j) );
        }
        if j > 0 && ! seen.contains( &(i,j-1) ) && data[i][j-1] != 9 {
            seen.insert( (i,j-1) );
            to_visit.push( (i,j-1) );
        }

        if i+1 < m && ! seen.contains( &(i+1,j) ) && data[i+1][j] != 9 {
            seen.insert( (i+1,j) );
            to_visit.push( (i+1,j) );
        }
        if j+1 < n && ! seen.contains( &(i,j+1) ) && data[i][j+1] != 9 {
            seen.insert( (i,j+1) );
            to_visit.push( (i,j+1) );
        }
    }

    return basin;
}

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day9")?;
    let data : Vec<Vec<u32>> = reader.lines().map(|line| line.unwrap().chars().map(|ch| (ch as u32) - ('0' as u32)).collect() ).collect();
    
    let m = data.len();
    let n = data[0].len();

    let minima : Vec<(usize,usize,u32)> = (0 .. m).flat_map(|i| (0 .. n).filter_map(|j| {
            let v = data[i][j];
            if (i == 0 || v < data[i-1][j]) && (j == 0 || v < data[i][j-1]) && (i+1 == m || v < data[i+1][j]) && (j+1 == n || v < data[i][j+1]) {
                Some( (i,j,v) )
            } else {
                None
            }
        }).collect::<Vec<_>>()
    ).collect();

    let p1 = minima.iter().map(|v| v.2+1).sum::<u32>();
    println!("{}",p1);

    let mut basins : Vec<usize> = minima.iter().map(|v| walk_basin(&data, v.0, v.1).len() ).collect();
    basins.sort_by_key(|v| std::cmp::Reverse(*v) );
    let p2 = basins[0..3].iter().fold(1,|prev,cur| prev*cur);

    println!("{}",p2);

    Ok( () )
}