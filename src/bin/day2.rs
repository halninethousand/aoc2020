fn main() {
    let input: Vec<String> = include_str!("../../data/day2.txt").lines()
                                                             .map(|x| x.to_string())
                                                             .collect();

    let mut valid: u32 = 0;

    for item in input.clone() {
        let mut parse: Vec<String> = item.split_whitespace().map(|s| s.to_string()).collect();
        let range: Vec<u32> = parse[0].split('-').map(|n| n.parse().unwrap()).collect();

        let min = range[0];
        let max = range[1];

        parse[1].truncate(1);

        let ch: Vec<char> = parse[1].chars().collect();
        let occ = parse[2].chars().filter(|c| *c == ch[0]).count() as u32;

        if occ >= min && occ <= max {
            valid += 1;
        }
    }

    println!("part 1 {}", valid);


    let mut valid: u32 = 0;

    for item in input.clone() {
        let mut parse: Vec<String> = item.split_whitespace().map(|s| s.to_string()).collect();
        let valid_indices: Vec<u32> = parse[0].split('-').map(|n| n.parse().unwrap()).collect();

        let one = valid_indices[0];
        let two = valid_indices[1];

        parse[1].truncate(1);

        let ch: Vec<char> = parse[1].chars().collect();

        let ans: bool = (parse[2].chars().nth((one as usize) - 1).unwrap() == ch[0]) ^ (parse[2].chars().nth((two as usize) - 1).unwrap() == ch[0]);
        
        if ans {
            println!("Found");
            valid += 1;
        }
    }

    println!("part 2 {}", valid);

}
