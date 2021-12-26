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
            } else if seen & neighbor_flag == 0 { // visiting for first time
                Some( (neighbor.as_str(),*twice,seen | neighbor_flag) )
            } else if twice.is_none() { // we're can visit this one twice
                assert!(seen & neighbor_flag != 0);
                Some( (neighbor.as_str(),Some(neighbor.as_str()),*seen) )
            } else {
                None
            }
        }).collect::<Vec<(&'a str,Option<&'a str>,usize)>>()
    }
}

fn walk_and_find_internal<'a>(cur : (&'a str,Option<&'a str>,usize), path : &[&'a str], edges : &'a HashMap<String,(usize,HashSet<String>)>, out : &mut Vec<Vec<&'a str>>) {
    let mut new_path = path.to_vec();
    new_path.push(cur.0);

    if cur.0 == "end" {
        out.push(new_path);
    } else {
        let neighbors = get_neighbors(&cur, edges);

        for neighbor in neighbors {
            walk_and_find_internal(neighbor, &new_path, edges, out);
        }
    }
}

fn walk_and_find(allow_double_visit : bool, edges : &HashMap<String,(usize,HashSet<String>)>) -> Vec<Vec<&str>> {
    let mut out = Vec::new();
    let start_flag = edges.get("start").unwrap().0;
    walk_and_find_internal( 
        ("start", if allow_double_visit { None } else { Some("PLACEHOLDER") }, start_flag), 
        &[], edges, &mut out);
    out
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

    let result_once = walk_and_find(false, &edges);

    println!("{:?}",result_once.len());

    let result_twice =  walk_and_find(true, &edges);

    println!("{:?}",result_twice.len());

    Ok( () )
}