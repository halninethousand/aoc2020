fn main() {
    let mut input: Vec<u64> = include_str!("../../data/day10.txt")
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect();

    // add 0 to start and +3 to end
    input.push(0);
    input.sort();
    input.push(input[input.len() - 1] + 3);

    let mut ones: u16 = 0;
    let mut threes: u16 = 0;
    
    // part 1
    for (i, item) in input.iter().enumerate() {
        if (i + 1) > input.len() - 1 {
            continue;
        } 

        let diff: u64 = input[i + 1] - item;

        if diff == 1 {
            ones += 1;
        } else {
            threes += 1;
        }
    }

    println!("Ones: {} Threes: {} multi: {}", ones, threes, ones * threes);

    let mut slices = vec![];
    let mut current_slice = vec![];

    // generate slices of consecutive 1-diff elements, for example:
    // input:  [0, 1, 4, 5, 6, 9]
    // result: [[0, 1], [4, 5, 6], [9]]
    for window in input.windows(2) {
        match window[1] - window[0] {
            1 => current_slice.push(window[0]),
            3 => {
                current_slice.push(window[0]);
                slices.push(current_slice);
                current_slice = vec![];
            }
            _ => (),
        }
    }

    let prod_slices: u64 = slices
        .iter()
        .map(|slice| match slice.len() {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => panic!("unexpected slice of size N > 5 consecutive 1-diff elements"),
        })
        .product();

    println!("Variations in arrangement: {}", prod_slices);
 
}
