#![warn( clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;

#[allow(clippy::needless_return)]
fn first_mismatch(s : &str) -> Result<char,usize> {
    let mut expected_closer = Vec::new();
    for ch in s.chars() {
        match ch {
            '(' => expected_closer.push(')'),
            '[' => expected_closer.push(']'),
            '{' => expected_closer.push('}'),
            '<' => expected_closer.push('>'),
            ')'|']'|'}'|'>' => {
                let expected = expected_closer.pop().unwrap();
                if  expected != ch {
                    return Ok(ch);
                }
            }
            _ => unreachable!()
        };
    }
    
    return Err( expected_closer.into_iter().rev().map(score_for_autocomplete).fold(0_usize, |prev,next| prev*5 + next) );
}

fn score_for_autocomplete(ch : char) -> usize {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!()
    }
}

fn mismatch_to_score(mismatch : &Result<char,usize>) -> usize {
    match mismatch {
        Ok(')') => 3,
        Ok(']') => 57,
        Ok('}') => 1197,
        Ok('>') => 25137,
        Err(_) => 0,
        _ => unreachable!()
    }
}
fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day10")?;

    let lines : Vec<String> = reader.lines().map(Result::unwrap).collect();

    let data : Vec<_> = lines.iter().map(|line| first_mismatch(line)).collect();
    println!("{}", data.iter().map(mismatch_to_score).sum::<usize>());
    
    let mut autocomplete_scores : Vec<_> = data.iter().filter_map(|s| s.err()).collect();
    autocomplete_scores.sort_unstable();
    println!("{}",autocomplete_scores[autocomplete_scores.len()/2]);

    Ok( () )
}