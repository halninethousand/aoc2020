use std::collections::VecDeque;
use itertools::Itertools;

fn main() {
    let input: Vec<u64> = include_str!("../data/day9.txt")
        .split("\r\n")
        .filter_map(|x| x.parse().ok())
        .collect();

    println!("{:?}", input);

    let window_len: usize = 25;
    let mut window: VecDeque<u64> = input.iter().cloned().take(window_len).collect();
    let mut combinations: Vec<_> = window.iter().cloned().combinations(2).collect();

    println!("initial window: {:?}", window);

    'main: for (c, (i, item)) in input[window_len..].iter().enumerate().enumerate() {
        if i == 0 {

        } else {
            window = input.iter().cloned().skip(c).take(window_len).collect();
            combinations = window.iter().cloned().combinations(2).collect();
            println!("window {:?}, {}", window, item);
        }

        let mut found: Vec<(u64, u64)> = vec![];

        for comb in &combinations {
            if comb[0] + comb[1] != *item {
                // println!("{} + {} != {}", comb[0], comb[1], item);
            } else {
                println!("Found sum: {} + {} == {}", comb[0], comb[1], item);
                found.push((comb[0], comb[1]));
            }
        }
        
        println!("{} combinations found", found.len());
        
        // part1
        if found.is_empty() {
            println!("{} is not the sum of the previous window of {}", item, window_len);
            break 'main; 
        }
    }
    
    // part2
    let target_sum: u64 = 20874512;
    'main: for start_idx in 0..input.len() {
        let mut nums: Vec<u64> = vec![];
        let mut sum: u64 = 0;
        
        for &item in input[start_idx..].iter() {
            nums.push(item);
            sum += item;
            
            if nums.len() >= 2 && sum == target_sum {
                println!("Contiguous sum found idx: {},\n{:?}", start_idx, nums);
                println!("sum: {}", nums[0] + nums[nums.len()-1]);
                break 'main;
            }
            
            if sum > target_sum {
                break;  // Move to next starting point
            }
        }
    }
}

