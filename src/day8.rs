use std::collections::HashMap;

fn main() {
    let mut input: Vec<String> = include_str!("../data/day8.txt").split("\r\n").map(|x| x.to_owned()).collect();

    println!("{:?}", input);
    
    let mut visited_instructions: HashMap<usize, usize> = HashMap::new(); 
    let mut acc: isize = 0;
   

    let mut id = 0;
    let mut counter = 0;

    'main: while id < input.len() {
        let item = input[id].clone();

        // println!("{} at {}", item, id);

        if let Some(_) = visited_instructions.get(&id) {
            println!("SEEN a second time: instruction id: {}, instruction: {}", id, item);
            println!("{}", acc);
            break 'main;
        }

        visited_instructions.entry(id).or_insert(1);

        if item.starts_with("jmp") {
            if let Ok(offset) = item[4..].trim().parse::<isize>() {
                let new_index = id as isize + offset;
                if new_index >= 0 && new_index < input.len() as isize {
                    id = new_index as usize;
                    continue;
                } else {
                    panic!("Jumped outside bounad")
                }
            }
        } else if item.starts_with("acc") {
            if let Ok(offset) = item[4..].trim().parse::<isize>() {
                acc += offset; 
            }
        }

        id += 1;
    }

    let input_2: Vec<&str> = input.iter().map(|x| x.as_str()).collect();
    part1(&input_2);
    part2(&input_2);
}

fn part2(split_file: &Vec<&str>) {
    // First get lines that contain nop or jmp
    let mut line = 0;
    while line < split_file.len() as i32 {
        if !split_file[line as usize].contains("acc") {
            let mut mutated_file = split_file.clone();
            let mutated_line = match &mutated_file[line as usize][0..3] {
                "jmp" => mutated_file[line as usize].replace("jmp", "nop"),
                "nop" => mutated_file[line as usize].replace("nop", "kmp"),
                _ => break
            };
            let _got = std::mem::replace(&mut mutated_file[line as usize], &mutated_line);
            let success = part1(&mutated_file);
            if success {
                println!("Modifying line {} fixed the bug.", line+1);
                break;
            }
        }
        line += 1
    }
}

fn part1(split_file: &Vec<&str>) -> bool {
    let mut accumulator: i32 = 0;
    let mut line: i32 = 0;
    let mut executed_lines = Vec::<i32>::new();

    while line < (split_file.len() - 1) as i32 {
        if executed_lines.contains(&line) {
            println!("Line hit twice {}. Accumulator: {}", line+1, accumulator);
            return false;
        } else {
            executed_lines.push(line);
        }
        let this_line: Vec<&str> = split_file[line as usize].split(" ").collect();
        let (action, count) = (this_line[0], this_line[1].parse::<i32>().unwrap());
        match action {
            "acc" => {
                accumulator += count;
                line += 1
            },
            "jmp" => line += count,
            "nop" => line += 1,
            _ => ()
        }
    }

    println!("Success! No line hit twice. Accumulator: {}", accumulator);
    return true;
}
