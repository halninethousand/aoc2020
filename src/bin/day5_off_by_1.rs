fn main() {
    let input: Vec<String> = include_str!("../../data/day5.txt").split("\r\n").map(|x| x.trim().to_owned()).collect();
    println!("{}", input.len()); 

    let mut highest_id: u32 = 0;

    for item in input {

        if item.is_empty() {
            continue;
        }

        let mut row_start = 0;
        let mut row_end = 127;
        let mut col_start = 0;
        let mut col_end = 7;

        // Calculate row
        for &c in item[..7].as_bytes() {
            let mid = (row_start + row_end) / 2;
            if c == b'F' {
                row_end = mid;
            } else {
                row_start = mid + 1;
            }
        }

        // Calculate column
        for &c in item[7..].as_bytes() {
            let mid = (col_start + col_end) / 2;
            if c == b'L' {
                col_end = mid;
            } else {
                col_start = mid + 1;
            }
        }

        let seat_id = row_start * 8 + col_start;

        if seat_id > highest_id {
            highest_id = seat_id;
        }
        println!("SEAT: {}, Row: {}, Column: {}, ID: {}", item, row_start, col_start, seat_id);
    }

    println!("Part 1: {}", highest_id);
}
