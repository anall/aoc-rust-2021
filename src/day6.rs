#![warn( clippy::all, clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;

#[derive(Debug)]
struct State {
    pop : [usize;9]
}
impl State {
    fn new(numbers : &[u8]) -> Self {
        let mut pop = [0;9];
        for n in numbers {
            pop[(*n) as usize] += 1;
        }

        Self { pop }
    }

    fn advance(&self) -> Self {
        Self {
            pop: [
                self.pop[1], // 0
                self.pop[2], // 1
                self.pop[3], // 2
                self.pop[4], // 3
                self.pop[5], // 4
                self.pop[6], // 5
                self.pop[7] + self.pop[0], // 6 (7 and everything reset from 0)
                self.pop[8], // 7
                self.pop[0] // 8 new fish
            ]
        }
    }

    fn count_fish(&self) -> usize {
        self.pop.iter().sum()
    }
}
fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day6")?;
    let numbers : Vec<_> = reader.lines().next().unwrap().unwrap().split(",").map(|v| v.parse::<u8>().unwrap() ).collect();

    let state = State::new(&numbers);

    let result = (0..80).fold(state,|state,_| state.advance());
    println!("{}",result.count_fish());
    
    let result = (80..256).fold(result,|state,_| state.advance());
    println!("{}",result.count_fish());

    Ok( () )
}