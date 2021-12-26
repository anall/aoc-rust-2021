#![warn( clippy::pedantic )]
use std::io::BufRead;
use std::collections::{HashMap,HashSet};
use adventlib::aoc;
use regex::Regex;

fn get_neighbors<'a>((node,seen) : &(&str, usize), edges : &'a HashMap<String,(usize,HashSet<String>)>) -> Vec<(&'a str,usize)> {
    let (_,neighbors) = edges.get(*node).unwrap();
    if *node == "end" {
        vec![]
    } else {
        neighbors.iter().filter_map(|neighbor| {
            let (neighbor_flag,_) = edges.get(neighbor.as_str()).unwrap();
            if *neighbor == neighbor.to_ascii_uppercase() {
                Some( (neighbor.as_str(),seen | neighbor_flag) )
            } else {
                if seen & neighbor_flag != 0 {
                    None
                } else {
                    Some( (neighbor.as_str(),seen | neighbor_flag) )
                }
            }
        }).collect::<Vec<(&str,usize)>>()
    }
}

fn get_neighbors_twice<'a>((node,twice,seen_a,seen_b) : &(&'a str, Option<&'a str>, usize, usize), edges : &'a HashMap<String,(usize,HashSet<String>)>) -> Vec<(&'a str,Option<&'a str>,usize,usize)> {
    let (_,neighbors) = edges.get(*node).unwrap();
    if *node == "end" {
        vec![]
    } else {
        neighbors.iter().filter_map(|neighbor| {
            let (neighbor_flag,_) = edges.get(neighbor.as_str()).unwrap();
            if *neighbor == neighbor.to_ascii_uppercase() {
                Some( (neighbor.as_str(),twice.clone(),seen_a | neighbor_flag, seen_b | neighbor_flag) )
            } else {
                if seen_a & neighbor_flag != 0 && seen_b & neighbor_flag != 0 {
                    None
                } else if let Some(twice_v) = twice {
                    if twice_v == neighbor {
                        assert!(seen_a & neighbor_flag != 0 && seen_b & neighbor_flag != 0);
                        None
                    } else if seen_a & neighbor_flag == 0 {
                        assert!(seen_b & neighbor_flag == 0);
                        Some( (neighbor.as_str(),Some(*twice_v),seen_a | neighbor_flag, seen_b | neighbor_flag) )
                    } else {
                        None
                    }
                } else {
                    assert!(seen_b & neighbor_flag == 0);
                    if seen_a & neighbor_flag == 0 {
                        Some( (neighbor.as_str(),None,seen_a | neighbor_flag, *seen_b) )
                    } else {
                        Some( (neighbor.as_str(),Some(neighbor.as_str()),*seen_a, seen_b | neighbor_flag) )
                    }
                }
            }
        }).collect::<Vec<(&'a str,Option<&'a str>,usize,usize)>>()
    }
}

fn walk_and_find<'a>(cur : (&'a str,usize), path : &Vec<&'a str>, edges : &'a HashMap<String,(usize,HashSet<String>)>, out : &mut Vec<Vec<&'a str>>) {
    let mut new_path = path.clone();
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

fn walk_and_find_twice<'a>(cur : (&'a str,Option<&'a str>,usize,usize), path : &Vec<&'a str>, edges : &'a HashMap<String,(usize,HashSet<String>)>, out : &mut Vec<Vec<&'a str>>) {
    let mut new_path = path.clone();
    new_path.push(cur.0);

    if cur.0 == "end" {
        out.push(new_path);
    } else {
        let neighbors = get_neighbors_twice(&cur, edges);

        for neighbor in neighbors {
            walk_and_find_twice(neighbor, &new_path, edges, out);
        }
    }
}


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

            out.entry(left.to_string()).or_insert_with(|| { let cbit = bit; bit = bit << 1; (cbit,Default::default()) }).1.insert(right.to_string());
            out.entry(right.to_string()).or_insert_with(|| { let cbit = bit; bit = bit << 1; (cbit,Default::default()) }).1.insert(left.to_string());
        }
        out
    };

    let start_flag = edges.get("start").unwrap().0;
    let mut result_once = Vec::new();
    walk_and_find(("start",start_flag), &vec![], &edges, &mut result_once);

    println!("{:?}",result_once.len());

    let mut result_twice = Vec::new();
    walk_and_find_twice(("start",None,start_flag,start_flag), &vec![], &edges, &mut result_twice);

    println!("{:?}",result_twice.len());


    Ok( () )
}