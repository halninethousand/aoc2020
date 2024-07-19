use std::collections::HashMap;

fn main() {
    //let input: Vec<String> = include_str!("../data/current.txt").lines().map(|line| line.to_owned()).collect();

    let input: Vec<&str> = include_str!("../data/day4.txt").split("\r\n\r").map(|x| x.trim()).collect();


    let eyes: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let fieldz: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl" , "pid"];

    let alphabet = (b'a'..=b'f')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    let numabet: Vec<String> = (0..=9).map(|x| x.to_string()).collect();

    println!("NUMABE NUMABE {:?}", numabet); 

    let mut valid_1: u32 = 0;
    let mut valid: u32 = 0;

    'all: for item in input {
        let trimmed = item.replace("\r\n", " ");

        let fields: Vec<&str> = trimmed.split_whitespace().collect();
        
        let mut seen_fields: HashMap<&str, &str> = HashMap::new();

        for field in fields {
            let key_value: Vec<&str> = field.split(':').collect();
            seen_fields.insert(key_value[0], key_value[1]);
        }

   
        // fields exist
        for item in fieldz {
            if !(seen_fields.contains_key(item)) {
                continue 'all;
            }

        }

        valid_1 += 1;


        // check values
        for (key, val) in seen_fields.iter() {
            if *key == "byr" {
                if !(val.chars().count() == 4 && val.parse::<usize>().unwrap() >= 1920 && val.parse::<usize>().unwrap() <= 2002) {
                    println!("ERROR IN KEY: {:?} VAL: {:?}", key, val); 
                    continue 'all;
                }
            }

            if *key == "iyr" {
                if !(val.chars().count() == 4 && val.parse::<usize>().unwrap() >= 2010 && val.parse::<usize>().unwrap() <= 2020) {
                    println!("ERROR IN KEY: {:?} VAL: {:?}", key, val); 
                    continue 'all;
                }
            }

            if *key == "eyr" {
                if !(val.chars().count() == 4 && val.parse::<usize>().unwrap() >= 2020 && val.parse::<usize>().unwrap() <= 2030) {
                    println!("ERROR IN KEY: {:?} VAL: {:?}", key, val); 
                    continue 'all;
                }
            }

            if *key == "hgt" {
                if val.ends_with("cm") {
                    let vval: u32 = val.trim_end_matches("cm").parse().expect("Couldn't parse {:?val}");
                    if !((150..=193).contains(&vval)) {
                        println!("ERROR IN KEY: {:?} VAL: {:?}", key, val); 
                        continue 'all;
                    }
                } else if val.ends_with("in") {
                    let vval: u32 = val.trim_end_matches("in").parse().expect("Couldn't parse {:?val}");
                    if !((59..=76).contains(&vval)) {
                        println!("ERROR IN KEY: {:?} VAL: {:?}", key, val); 
                        continue 'all;
                    }
                } else {
                    continue 'all;
                }
            }
            

            if *key == "hcl" {
                if val.starts_with('#') {
                    let nums = val.trim_start_matches('#');
                    for item in nums.chars() {
                        if alphabet.contains(&item) || numabet.contains(&item.to_string()) {
                            println!("{} IN {}", item, nums); 
                        } else {
                            println!("ERROR IN KEY: {:?} VAL: {:?}", key, val); 
                            continue 'all;
                        }
                    }
                } else {
                    println!("ERROR IN KEY: {:?} VAL: {:?}", key, val); 
                    continue 'all;
                }
                println!("HCL VALS: {}", val); 
            }

            if *key == "ecl" {
                if !(eyes.contains(&val)) {
                    println!("ERROR IN KEY: {:?} VAL: {:?}", key, val); 
                    continue 'all;
                }
            }

            if *key == "pid" {
                if val.chars().count() != 9 {
                    println!("ERROR IN KEY: {:?} VAL: {:?}", key, val); 
                    continue 'all;
                }
            }
   
        }

        valid += 1;

    }

    println!("Part 1: {}", valid_1);
    println!("Part 2: {}", valid);

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.

}
