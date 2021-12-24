#![warn( clippy::pedantic )]
#![allow( clippy::cast_sign_loss )]
use std::io::BufRead;
use std::cmp;
use adventlib::aoc;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point(i32,i32);

struct PointIterator {
    start : Point,
    n : i32, len : i32,
    x_dir : i32,
    y_dir : i32
}
impl PointIterator {
    fn new(line : &Line) -> Self {
        let Point(x1,y1) = line.0;
        let Point(x2,y2) = line.1;

        if x1==x2 {
            assert!(y1 < y2);
            Self {
                start: line.0,
                n: 0,
                len: y2-y1,
                x_dir: 0,
                y_dir: 1,
            }
        } else if y1==y2 {
            assert!(x1 < x2);
            Self {
                start: line.0,
                n: 0,
                len: x2-x1,
                x_dir: 1,
                y_dir: 0,
            }
        } else {
            assert!( (x2-x1).abs() == (y2-y1).abs() );
            let len = (x2-x1).abs();

            Self {
                start: line.0,
                n: 0,
                len,
                x_dir: (x2-x1)/len,
                y_dir: (y2-y1)/len,
            }
        }
    }
}
impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == self.len+1 {
            None
        } else {
            let pt = Point(self.start.0 + self.n * self.x_dir,self.start.1 + self.n * self.y_dir);
            self.n += 1;
            Some(pt)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line(Point,Point);
impl Line {
    fn is_axial(&self) -> bool {
        self.0.0 == self.1.0 || self.0.1 ==  self.1.1
    }

    fn points(&self) -> PointIterator {
        PointIterator::new(self)
    }

    fn apply_count_to_data(&self, data : &mut[Vec<u8>], base_x : i32, base_y : i32) {
        for pt in self.points() {
            data[(pt.0 - base_x) as usize][(pt.1 - base_y) as usize] += 1;
        }
    }
}

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
        let pt1 = Point( cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap() );
        let pt2 =  Point(cap[3].parse::<i32>().unwrap(), cap[4].parse::<i32>().unwrap() );

        if pt2 < pt1 {
            Line(pt2,pt1)
        } else {
            Line(pt1,pt2)
        }
    }).collect();

    let min_x = lines.iter().map(|Line(pt1,pt2)| { assert!(pt2.0 >= pt1.0); pt1.0 }).min().unwrap();
    let max_x = lines.iter().map(|Line(pt1,pt2)| { assert!(pt2.0 >= pt1.0); pt2.0 }).max().unwrap();
    let len_x = (max_x - min_x) as usize;

    let min_y = lines.iter().map(|Line(pt1,pt2)| cmp::min(pt1.1,pt2.1)).min().unwrap();
    let max_y = lines.iter().map(|Line(pt1,pt2)| cmp::max(pt1.1,pt2.1)).max().unwrap();
    let len_y = (max_y - min_y) as usize;

    let mut data = vec![vec![0_u8; len_y+1]; len_x+1];

    for line in &lines {
        if line.is_axial() {
            line.apply_count_to_data(&mut data, min_x, min_y);
        }
    }

    let pt1 = count_intersections(&data);

    for line in &lines {
        if ! line.is_axial() {
            line.apply_count_to_data(&mut data, min_x, min_y);
        }
    }

    let pt2 = count_intersections(&data);
    println!("{} {}",pt1,pt2);

    Ok( () )
}
