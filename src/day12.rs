#![warn( clippy::pedantic )]
use std::io::BufRead;
use std::collections::{HashMap,HashSet};
use adventlib::aoc;
use regex::Regex;

fn get_neighbors<'a>((node,twice,seen) : &(&'a str, Option<&'a str>, usize), edges : &'a HashMap<String,(usize,bool,HashSet<String>)>) -> Vec<(&'a str,Option<&'a str>,usize)> {
    let (_,_,neighbors) = edges.get(*node).unwrap();
    neighbors.iter().filter_map(|neighbor| {
        let (neighbor_flag,is_uppercase,_) = edges.get(neighbor.as_str()).unwrap();
        if *is_uppercase || seen & neighbor_flag == 0 { // uppercase nodes can be visited as many times as we'd like
            Some( (neighbor.as_str(),*twice,seen | neighbor_flag) )
        } else if twice.is_none() { // we're can visit this one twice
            assert!(seen & neighbor_flag != 0);
            Some( (neighbor.as_str(),Some(neighbor.as_str()),*seen) )
        } else {
            None
        }
    }).collect::<Vec<(&'a str,Option<&'a str>,usize)>>()
}

#[allow(unused)]
fn walk_and_find_internal<'a>(cur : (&'a str,Option<&'a str>,usize), path : &[&'a str], edges : &'a HashMap<String,(usize,bool,HashSet<String>)>, out : &mut Vec<Vec<&'a str>>) {
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

#[allow(unused)]
fn walk_and_find(allow_double_visit : bool, edges : &HashMap<String,(usize,bool,HashSet<String>)>) -> Vec<Vec<&str>> {
    let mut out = Vec::new();
    let start_flag = edges.get("start").unwrap().0;
    walk_and_find_internal( 
        ("start", if allow_double_visit { None } else { Some("PLACEHOLDER") }, start_flag), 
        &[], edges, &mut out);
    out
}


fn walk_and_count_internal(cur : (&str,Option<&str>,usize), edges : &HashMap<String,(usize,bool,HashSet<String>)>, out : &mut usize) {
    if cur.0 == "end" {
        *out += 1;
    } else {
        let neighbors = get_neighbors(&cur, edges);

        for neighbor in neighbors {
            walk_and_count_internal(neighbor, edges, out);
        }
    }
}

fn walk_and_count(allow_double_visit : bool, edges : &HashMap<String,(usize,bool,HashSet<String>)>) -> usize {
    let mut out = 0;
    let start_flag = edges.get("start").unwrap().0;
    walk_and_count_internal( 
        ("start", if allow_double_visit { None } else { Some("PLACEHOLDER") }, start_flag), 
        edges, &mut out);
    out
}


#[allow(clippy::similar_names)]
fn main() -> aoc::Result<()> {
    let regex = Regex::new("^(.+?)-(.+?)$").unwrap();
    let reader = aoc::file("inputs/day12")?;

    let edges = {
        let mut out : HashMap<String,(usize,bool,HashSet<String>)> = HashMap::new();
        let mut bit : usize = 1;
        for line in reader.lines().map(Result::unwrap) {
            let cap = regex.captures(&line).unwrap();
            let left = &cap[1];
            let right = &cap[2];

            // start cannot be reentered, however we should still create the entries no matter the order
            let l_entry = out.entry(left.to_string()).or_insert_with(|| {
                let cbit = bit;
                bit <<= 1;
                (cbit,left == left.to_ascii_uppercase(),HashSet::new()) });
            if right != "start" {
                l_entry.2.insert(right.to_string());
            }

            let r_entry = out.entry(right.to_string()).or_insert_with(|| {
                let cbit = bit;
                bit <<= 1;
                (cbit,right == right.to_ascii_uppercase(),HashSet::new()) });
            if left != "start" {
                r_entry.2.insert(left.to_string());
            }
        }

        // end cannot be exited once entered
        out.get_mut("end").unwrap().2.clear();
        
        out
    };

    println!("{}",walk_and_count(false, &edges));
    println!("{}",walk_and_count(true, &edges));

    Ok( () )
}