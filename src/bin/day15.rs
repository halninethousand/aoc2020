use std::collections::HashMap;

// bruteforce as usual, approx ~30 seconds on a 5800X3D, cpu usuage 20~30 %
fn main() {

    let mut input: Vec<usize> = include_str!("../../data/day15.txt")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();
    
    let mut spoken_map: HashMap<usize, usize> = HashMap::new();
    let mut seen_map: HashMap<usize, usize> = HashMap::new();
    
    for (i, num) in input.iter().enumerate() {
        *spoken_map.entry(*num).or_insert(i) = i;
        *seen_map.entry(*num).or_insert(1) += 1;
    }

    
    // comment uncomment part 1/ part 2
    let part1: usize = 2020;
    let part2: usize = 30_000_000; 

    for i in input.len()..part2 {
        let last = input[input.len()-1];
        if let Some(seen_last) = seen_map.get(&last) {
            if *seen_last > 1 {
                let last_seen_idx = spoken_map.get(&last);
                let idx_diff = i - 1 - last_seen_idx.unwrap();
                input.push(idx_diff);
            } else {
                input.push(0);
            }
        } else {
            input.push(0);
        }
        
        *spoken_map.entry(last).or_insert(i) = i-1;
        *seen_map.entry(last).or_insert(1) += 1;
    }

    println!("last element: {}", input.last().unwrap());
}
