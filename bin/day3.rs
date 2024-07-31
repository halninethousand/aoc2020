fn main() {
    let input: Vec<String> = include_str!("../data/day3.txt").lines()
                                                             .map(|x| x.to_string())
                                                             .collect();

    let mut map: Vec<Vec<String>> = vec![];

    for _ in 0..input.len() {
        map.push(input.clone());
    }

    let mut long_map = vec![];
    
    // move from a Vec<Vec<String>> to Vec<String>, probably could be done with fold
    for i in 0..input.len() {
        let mut long_line: String = String::new();
        for _ in 0..input.len() {
            long_line.push_str(&map[i][i].clone());
        }
        long_map.push(long_line.clone());
    }
    
    let long_map: Vec<_> = long_map.iter().map(|x| x.to_owned()).collect();

    let mut part1: u64 = 0;

    let mut horiz: usize= 0;
    let mut vert: usize = 0;

    for _ in 1..long_map.len() {
        horiz += 3;
        vert += 1;

        if long_map[vert].chars().nth(horiz).unwrap() == '#' {
            part1 += 1;
        }
    }

    println!("Part 1: {}", part1);


    let mut part2: u64 = 1;

    horiz = 0;
    vert = 0;
    part1 = 0;

    for _ in 1..long_map.len() {
        horiz += 1;
        vert += 1;

        if long_map[vert].chars().nth(horiz).unwrap() == '#' {
            part1 += 1;
        }
    }
    
    println!("[1, 1: {}]", part1);

    part2 *= part1;

    horiz = 0;
    vert = 0;
    part1 = 0;

    for _ in 1..long_map.len() {
        horiz += 3;
        vert += 1;

        if long_map[vert].chars().nth(horiz).unwrap() == '#' {
            part1 += 1;
        }
    }

    println!("[3, 1: {}]", part1);

    part2 *= part1;

    horiz = 0;
    vert = 0;
    part1 = 0;

    for _ in 1..long_map.len() {
        horiz += 5;
        vert += 1;

        if long_map[vert].chars().nth(horiz).unwrap() == '#' {
            part1 += 1;
        }
    }

    println!("[5, 1: {}]", part1);

    part2 *= part1;

    horiz = 0;
    vert = 0;
    part1 = 0;

    for _ in 1..long_map.len() {
        horiz += 7;
        vert += 1;

        if long_map[vert].chars().nth(horiz).unwrap() == '#' {
            part1 += 1;
        }
    }

    println!("[7, 1: {}]", part1);

    part2 *= part1;

    horiz = 0;
    vert = 0;
    part1 = 0;

    for _ in 1..long_map.len() {
        horiz += 1;
        vert += 2;

        if horiz >= long_map.len() || vert >= long_map.len() {
            break
        }

        if long_map[vert].chars().nth(horiz).unwrap() == '#' {
            part1 += 1;
        }
    }

    part2 *= part1;

    println!("[1, 2: {}]", part1);

    println!("Part 2: {}", part2);

}
