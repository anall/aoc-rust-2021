#![warn( clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;

// Make this a (unbounded) iterator. This will give us a bunch for free.
struct OctopusIterator {
    data : Vec<Vec<u8>>,
}

impl OctopusIterator {
    fn new_from_data(data : &[Vec<u8>]) -> Self {
        let len = data[0].len();
        assert!(data.iter().all(|v| v.len() == len));
        Self { data: data.to_vec() }
    }
}

impl Iterator for OctopusIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // increment every cell by 1
        for row in &mut self.data {
            for cell in row {
                *cell += 1;
            }
        }

        let mut flashed = true;
        let mut n_flashes = 0;
        let m = self.data.len();
        let n = self.data[0].len();
        while flashed {
            flashed = false;
            for i in 0 .. m {
                for j in 0 .. n {
                    if self.data[i][j] > 9 {
                        flashed = true;
                        n_flashes += 1;
                        self.data[i][j] = 0;
                        if i > 0 {
                            if self.data[i-1][j] != 0 {
                                self.data[i-1][j] += 1;
                            }
                            if j > 0 && self.data[i-1][j-1] != 0 {
                                self.data[i-1][j-1] += 1;
                            }
                            if j+1 < n && self.data[i-1][j+1] != 0 {
                                self.data[i-1][j+1] += 1;
                            }
                        }
                        if i+1 < m {
                            if j > 0 && self.data[i+1][j-1] != 0 {
                                self.data[i+1][j-1] += 1;
                            }
                            if j+1 < n && self.data[i+1][j+1] != 0 {
                                self.data[i+1][j+1] += 1;
                            }
                            if self.data[i+1][j] != 0 {
                                self.data[i+1][j] += 1;
                            }
                        }
                        if j > 0 && self.data[i][j-1] != 0 {
                            self.data[i][j-1] += 1;
                        }
                        if j+1 < n && self.data[i][j+1] != 0 {
                            self.data[i][j+1] += 1;
                        }
                    }
                }
            }
        }

        Some( n_flashes )
    }
}

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day11")?;

    let data : Vec<Vec<u8>> = reader.lines().map(|ln| ln.unwrap().chars().map(|ch| (ch as u8) - b'0').collect() ).collect();
    let flash_count_100 : usize = OctopusIterator::new_from_data(&data).take(100).sum();
    println!("{:?}",flash_count_100);

    let count = data.len() * data[0].len();

    let all_flash = OctopusIterator::new_from_data(&data).position(|flashed| flashed == count).unwrap() + 1;
    println!("{:?}",all_flash);

    Ok( () )
}