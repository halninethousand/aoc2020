use std::collections::HashMap;

fn main() {
    let input: Vec<String> = include_str!("../data/day8.txt").split("\r\n").map(|x| x.to_owned()).collect();

    println!("{:?}", input);
    
    let mut visited_instructions: HashMap<usize, usize> = HashMap::new(); 
    let mut acc: isize = 0;
   

    let mut id = 0;

    'main: while id < input.len() {
        let item = input[id].clone();

        println!("{} at {}", item, id);

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
                }
            }
        } else if item.starts_with("acc") {
            if let Ok(offset) = item[4..].trim().parse::<isize>() {
                acc += offset; 
            }
        }

        id += 1;
    }
}
