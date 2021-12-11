#![warn( clippy::all, clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;

use std::collections::{HashMap,HashSet};

#[derive(Debug,Clone)]
struct Line {
    patterns : Vec<String>,
    digits : Vec<Result<(u8,String),String>>
}

fn get_singleton<R,I : IntoIterator<Item=R>>(data : I) -> R {
    let mut iter = data.into_iter();
    let val = iter.next().unwrap();
    assert!( iter.next().is_none() );
    val
}

impl Line {
    fn parse_all<T: BufRead>(reader : T) -> Vec<Self> {
        reader.lines().map(|ln| Self::parse(ln.unwrap())).collect()
    }

    fn decode_seven(s: &str) -> Result<(u8,String),String> {
        let mut chars : Vec<char> = s.chars().collect();
        chars.sort();
        let sorted : String = chars.into_iter().collect();
        let len = sorted.len();
        if len == 2 { // cf is unique length
            Ok( (1,sorted) )
        } else if len == 4 { // bcdf is unique length
            Ok( (4,sorted) )
        } else if len == 3 { // acf is unique length
            Ok( (7,sorted) )
        } else if len == 7 { // abcdef is unique length
            Ok( (8,sorted) )
        } else {
            Err(sorted)
        }
    }

    fn parse(s : String) -> Self {
        let mut split = s.split(" | ");
        let patterns = split.next().unwrap().to_string();
        let digits = split.next().unwrap().to_string();
        assert!( split.next().is_none() );

        Self {
            patterns: patterns.split(" ").map(|v| v.to_string()).collect(),
            digits: digits.split(" ").map(|s| Self::decode_seven(s) ).collect()
        }
    }

    fn infer_segments(&self) -> Line {
        let mut patterns_by_length : HashMap<usize,Vec<HashSet<char>>> = HashMap::new();
        let mut segment_mapping : HashMap<char,char> = HashMap::new();

        // bucket by length
        for pattern in &self.patterns {
            patterns_by_length.entry(pattern.len()).or_default().push( pattern.chars().collect() );
        }

        // Find 1, 4, 7, 8 (these are unique by segment lenght)
        let pattern_1 = get_singleton( patterns_by_length.get(&2).unwrap() );
        let pattern_4 = get_singleton( patterns_by_length.get(&4).unwrap() );
        let pattern_7 = get_singleton( patterns_by_length.get(&3).unwrap() );
        let pattern_8 = get_singleton( patterns_by_length.get(&7).unwrap() );

        // and we can get segment a from 1 and 7
        let segment_a = get_singleton( pattern_7.difference(&pattern_1).copied() );

        assert!(!segment_mapping.contains_key(&segment_a));
        segment_mapping.insert(segment_a,'a');

        // we know a
        // Of the 3 patterns of length 5 (2, 3, 5), we can determine a d g, and remove a (from 7) getting dg
        let length_5 = patterns_by_length.get(&5).unwrap();
        assert!(length_5.len() == 3);
        let segments_dg : HashSet<char> = {
            let mut iter = length_5.iter();
            let mut tmp = iter.next().unwrap().clone();
            for set in iter {
                tmp = tmp.intersection(set).copied().collect()
            }

            tmp.iter().copied().filter(|c| *c != segment_a ).collect()
        };
        
        // we can then use 4 to figure out d (and g)
        let segment_d = get_singleton( segments_dg.intersection(pattern_4).copied() );
        let segment_g = get_singleton( segments_dg.difference(pattern_4).copied() );

        assert!(!segment_mapping.contains_key(&segment_d));
        segment_mapping.insert(segment_d,'d');
        
        assert!(!segment_mapping.contains_key(&segment_g));
        segment_mapping.insert(segment_g,'g');

        // we know adg
        // now we can use 4 to figure out b ( subtract 1, subtract d )
        let segment_b = get_singleton( pattern_4.difference(pattern_1).copied().filter(|c| *c != segment_d) );

        assert!(!segment_mapping.contains_key(&segment_b));
        segment_mapping.insert(segment_b,'b');

        // we know abdg
        // Of the 3 patterns of length 6 (0, 6, 8) we can get abfg, and exclude abg to give f
        let length_6 = patterns_by_length.get(&6).unwrap();
        assert!(length_6.len() == 3);
        let segment_f = {
            let mut iter = length_6.iter();
            let mut tmp = iter.next().unwrap().clone();
            for set in iter {
                tmp = tmp.intersection(set).copied().collect()
            }

            get_singleton( tmp.iter().copied().filter(|c| *c != segment_a && *c != segment_b && *c != segment_g ) )
        };

        assert!(!segment_mapping.contains_key(&segment_f));
        segment_mapping.insert(segment_f,'f');
        
        // we know abdfg
        // now that we have f, we can get c from 1
        let segment_c = get_singleton( pattern_1.iter().copied().filter(|c| *c != segment_f ) ); 

        assert!(!segment_mapping.contains_key(&segment_c));
        segment_mapping.insert(segment_c,'c');

        // we know abcdfg, only missing e
        // I could write out the list but 8 is right there, and this would still work if we ever have other characters besides "abcdefg"
        let known_segments : HashSet<char> = segment_mapping.keys().copied().collect();
        let segment_e = get_singleton( pattern_8.difference(&known_segments).copied() );

        assert!(!segment_mapping.contains_key(&segment_e));
        segment_mapping.insert(segment_e,'e');

        let digits = self.digits.iter().cloned().map(|d| {
            match d { 
                Ok(v) => Ok(v),
                Err(s) => {
                    let mut remapped_vec : Vec<char> = s.chars().map(|v| *segment_mapping.get(&v).unwrap() ).collect();
                    remapped_vec.sort();
                    let remapped : String = remapped_vec.into_iter().collect();
                    match remapped.as_str() {
                        "abcefg" => Ok( (0,s) ),
                        // 1 is already handled
                        "acdeg" => Ok( (2,s) ),
                        "acdfg" => Ok( (3,s) ),
                        // 4 is already handled
                        "abdfg" => Ok( (5,s) ),
                        "abdefg" => Ok( (6,s) ),
                        // 7 is already handled
                        // 8 is already handled
                        "abcdfg" => Ok( (9,s) ),
                        _ => Err(s)
                    }
                }
            }
        }).collect();

        Self {
            patterns: self.patterns.clone(),
            digits: digits
        }
    }
}

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day8")?;
    let lines = Line::parse_all(reader);

    let counts_pt1 : usize = lines.iter().map(|line|
        line.digits.iter().filter_map(|d| d.as_ref().ok() ).filter(|d| d.0 == 1 || d.0 == 4 || d.0 == 7 || d.0 == 8 ).count() ).sum();

    println!("{}",counts_pt1);

    // try and infer other values
    let infered : Vec<_> = lines.iter().map(|line| line.infer_segments()).collect();
    let values : Vec<_> = infered.iter().map(|line| line.digits.iter().fold(0,|prev,v| (prev * 10) + (v.as_ref().unwrap().0 as usize) ) ).collect();
    let pt2 : usize = values.iter().sum();

    println!("{}",pt2);

    /*for line in infered {
        println!("{:?}",line.digits);
    }*/

    Ok( () )
}