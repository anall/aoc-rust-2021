#![warn( clippy::pedantic )]
use std::io::BufRead;
use std::collections::{HashMap,HashSet};
use adventlib::aoc;
use regex::Regex;

fn get_neighbors<'a>((node,twice,seen) : &(&'a str, Option<&'a str>, usize), edges : &'a HashMap<String,(usize,HashSet<String>)>) -> Vec<(&'a str,Option<&'a str>,usize)> {
    let (_,neighbors) = edges.get(*node).unwrap();
    if *node == "end" { // end can never be left
        vec![]
    } else {
        neighbors.iter().filter_map(|neighbor| {
            let (neighbor_flag,_) = edges.get(neighbor.as_str()).unwrap();
            if *neighbor == neighbor.to_ascii_uppercase()  {
                Some( (neighbor.as_str(),*twice,seen | neighbor_flag) )
            } else if *neighbor == "start" { // Start can never be reentered
                None
            } else if let Some(twice_v) = twice {
                if twice_v == neighbor { // we already visited this one a second time
                    assert!(seen & neighbor_flag != 0);
                    None
                } else if seen & neighbor_flag == 0 {
                    Some( (neighbor.as_str(),Some(*twice_v),seen | neighbor_flag) )
                } else {
                    None
                }
            } else if seen & neighbor_flag == 0 { // visiting for first time
                    Some( (neighbor.as_str(),None,seen | neighbor_flag) )
            } else { // we're visiting this one twice
                Some( (neighbor.as_str(),Some(neighbor.as_str()),*seen) )
            }
        }).collect::<Vec<(&'a str,Option<&'a str>,usize)>>()
    }
}

fn walk_and_find<'a>(cur : (&'a str,Option<&'a str>,usize), path : &[&'a str], edges : &'a HashMap<String,(usize,HashSet<String>)>, out : &mut Vec<Vec<&'a str>>) {
    let mut new_path = path.to_vec();
    new_path.push(cur.0);

    if cur.0 == "end" {
        out.push(new_path);
    } else {
        let neighbors = get_neighbors(&cur, edges);

        for neighbor in neighbors {
            walk_and_find(neighbor, &new_path, edges, out);
        }
    }
}


#[allow(clippy::similar_names)]
fn main() -> aoc::Result<()> {
    let regex = Regex::new("^(.+?)-(.+?)$").unwrap();
    let reader = aoc::file("inputs/day12")?;

    let edges = {
        let mut out : HashMap<String,(usize,HashSet<String>)> = HashMap::new();
        let mut bit : usize = 1;
        for line in reader.lines().map(Result::unwrap) {
            let cap = regex.captures(&line).unwrap();
            let left = &cap[1];
            let right = &cap[2];

            out.entry(left.to_string()).or_insert_with(|| { let cbit = bit; bit <<= 1; (cbit,HashSet::new()) }).1.insert(right.to_string());
            out.entry(right.to_string()).or_insert_with(|| { let cbit = bit; bit <<= 1; (cbit,HashSet::new()) }).1.insert(left.to_string());
        }
        out
    };

    let start_flag = edges.get("start").unwrap().0;
    let mut result_once = Vec::new();
    walk_and_find(("start",Some("PLACEHOLDER"),start_flag), &[], &edges, &mut result_once);

    println!("{:?}",result_once.len());

    let mut result_twice = Vec::new();
    walk_and_find(("start",None,start_flag), &[], &edges, &mut result_twice);

    println!("{:?}",result_twice.len());

    Ok( () )
}