use regex::Regex;
use ux::*;

const ONE: ux::u36 = u36::new(1);
const ZERO: ux::u36 = u36::new(0);
const RAM_SIZE: usize = 70000;

fn main() {
    let input: &str = include_str!("../../data/day14.txt");
    let input = split_input(input);

    let mut memory = vec![ZERO; RAM_SIZE];

    for instruction in input {
        println!("{:?}", instruction);


        for mem in instruction.1 {
            let re = Regex::new(r"\[(\d+)\]").unwrap();
            
            if let Some(mem_slot) = re.captures(&mem) {
                let mem_value = mem.split('=').nth(1).unwrap().trim().parse::<u64>().unwrap();
                let utsix = u36::new(mem_value); 
                let mut bits: Vec<ux::u36> = (0..36).map(|n| (utsix >> n) & ONE).collect();
                let mask_chars = instruction.0.chars().rev().enumerate();

                // println!("Bits len: {}", bits.len());
                // println!("binary: {}, bits: {:?}", utsix, bits);

                for (i, ch) in mask_chars {
                    if ch == 'X' {
                        continue;
                    } else if ch == '1' {
                        if bits[i] == ONE {

                        } else if bits[i] == ZERO {
                            bits[i] = ONE;
                        }

                    } else if ch == '0' {
                        if bits[i] == ZERO {

                        } else if bits[i] == ONE {
                            bits[i] = ZERO;
                        }
                    }
                } 

                let mut masked: String = String::new();

                for digit in bits.iter().rev() {
                    masked.push_str(&digit.to_string());
                }

                let u36_num: ux::u36 = bits.iter().take(36).enumerate().fold(ux::u36::new(0), |acc, (i, &x)| {
                    acc | (x & ux::u36::new(1)) << i
                }); 

                let idx: usize = mem_slot.get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();

                memory[idx] = u36_num;


                // println!("MASKED: {}", masked);
                // println!("mem_slot: {:?}, mem_value: {:?}", &mem_slot[1], mem_value);
            } else {
                panic!("couldn't extract memory");
            }

        }
    }
    
    let mut part1_sum: u64 = 0;
    
    for item in memory{
        if item != ZERO {
            part1_sum += u64::from(item);
        }
    }
    
    println!("Part1: sum of memory values: {}", part1_sum);
}

fn split_input(input: &str) -> Vec<(String, Vec<String>)> {
    let mut result = Vec::new();
    let mut current_mask = String::new();
    let mut current_mem_lines = Vec::new();

    for line in input.lines() {
        if line.starts_with("mask") {
            if !current_mask.is_empty() {
                result.push((current_mask, current_mem_lines));
                current_mem_lines = Vec::new();
            }
            current_mask = line.split('=').nth(1).unwrap().trim().to_string();
        } else if line.starts_with("mem") {
            current_mem_lines.push(line.to_string());
        }
    }

    if !current_mask.is_empty() {
        result.push((current_mask, current_mem_lines));
    }

    result
}
