use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = include_str!("../../data/day13.txt")
        .lines()
        .collect();

    let timestamp: u32 = input[0].parse().unwrap();
    let buses: Vec<u32> = input[1]
        .split(',')
        .filter(|&s| is_integer(s))
        .filter_map(|s| s.parse().ok())
        .collect();

    println!("Bus numbers: {:?}", buses);
    println!("Earlist timestamp: {}", timestamp);
    
    let mut comparer: HashMap<u32, u32> = HashMap::new();

    for bus in buses {
        let mut counter: u32 = 0;
        let mut accumulator: u32 = 0;
        loop {
            if accumulator >= timestamp {
                break;
            }
            accumulator += bus;
            counter += 1;
        }
        
        comparer.insert(bus, accumulator);

        println!("counter: {}, accumulator: {} for bus: {}", counter, accumulator, bus);
    }
    
    let min: u32 = *comparer.values().min().unwrap();

    match find_key_by_value(&comparer, min) {
        Some(key) => println!("Key for value {:?}: {}. (earliest - timestamp) * ID: {}", min, key, (min-timestamp) * key),
        None => println!("No key found for value {:?}", min),
    } 
}

fn is_integer(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

fn find_key_by_value(map: &HashMap<u32, u32>, value: u32) -> Option<u32> {
    map.iter()
        .find_map(|(&key, &val)| if val == value { Some(key) } else { None })
}
