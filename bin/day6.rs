use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input: Vec<String> = include_str!("../data/day6.txt").split("\r\n\r").map(|x| x.trim().to_owned()).collect();
    println!("{:?}", input);
    
    let mut count_seen: usize = 0;

    for item in &input {
        let mut group_seen: HashSet<char> = HashSet::new();
        let group: Vec<&str> = item.split("\r\n").collect();
        
        for item in &group {
            let ch = item.chars();
            if ch.clone().count() == 1 {
                group_seen.insert(item.chars().next().expect("NO CHARS?"));
            } else {
                for c in ch {
                    group_seen.insert(c);
                }
            }
        }

        count_seen += group_seen.len();

        println!("{:?}", group_seen);
    }

    println!("Part 1: {}", count_seen);

    let mut all: u32 = 0;

    for item in input {
        let mut group_seen: HashMap<char, u8> = HashMap::new();
        let group: Vec<&str> = item.split("\r\n").collect();
        let count: u8 = group.len() as u8;
        let mut everyone_ans: u8 = 0;

        println!("{:?}", group);

        
        
        for item in &group {
            let chars = item.chars();
            for ch in chars.clone() {
                *group_seen.entry(ch).or_insert(0) += 1;
            }
        }

        for (_k, v) in group_seen.iter() {
            if *v == count {
                everyone_ans += 1;
            } 
        }

        all += everyone_ans as u32;

        println!("Everyone answered: {} for {:?}", everyone_ans, group_seen);
    }

    println!("Part 2: {}", all);
}
