use itertools::Itertools;

fn main() {
    let input: Vec<u32> = include_str!("../../data/day1.txt").lines().map(|x| x.parse().unwrap()).collect();

    for pair in input.iter().combinations(2) {
        let sum: u32 = pair.clone().into_iter().sum(); 

        if sum == 2020 {
            println!("part 1 {:?}", pair[0] * pair[1]);
        }
    }

    for pair in input.iter().combinations(3) {
        let sum: u32 = pair.clone().into_iter().sum(); 

        if sum == 2020 {
            println!("part 2 {:?}", pair[0] * pair[1] * pair[2]);
        }
    }

    println!("{:?}", input);
}
