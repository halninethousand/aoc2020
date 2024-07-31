use std::fmt;
use std::ops::{Deref, DerefMut};

struct CharGrid(Vec<Vec<char>>);

impl CharGrid {
    fn new(vec: Vec<Vec<char>>) -> Self {
        CharGrid(vec)
    }
}

// Implement Deref to allow easy access to Vec methods
impl Deref for CharGrid {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CharGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for CharGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            for item in v {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut input_vec: Vec<Vec<char>> = include_str!("../../data/day11.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let mut input = CharGrid::new(input_vec.clone());
    let mut input_2 = CharGrid::new(input_vec.clone());
    
    println!("{}", input); // Use .0 to access the inner Vec for debug printing
    
    let offsets: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ];
    
    
    let mut empty_to_occupied: Vec<(usize, usize)> = vec![];   
    let mut occupied_to_empty: Vec<(usize, usize)> = vec![];   

    'main: loop {
        for (y, row) in input.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                let mut neighbors: Vec<char> = vec![];

                for (y_offset, x_offset) in offsets.iter() {
                    let adj_y = y as isize + *y_offset;
                    let adj_x = x as isize + *x_offset;
                    if in_bounds(&input, adj_y, adj_x) {
                        let neighbor = input[adj_y as usize][adj_x as usize];
                        neighbors.push(neighbor);
                    }
                }

                // println!("Neighbors at ({}, {}) ({}): {:?}", y, x, item, neighbors);
                
                let hashtags = neighbors
                    .iter()
                    .filter(|&x| *x == '#')
                    .count();

                if *item == 'L' && hashtags == 0 {
                    empty_to_occupied.push((y, x));
                } else if *item == '#' && hashtags >= 4 {
                    occupied_to_empty.push((y, x))
                } 
            } 
        }

        // println!("Empty seats to be occupied:\n{:?}", empty_to_occupied);
        // println!("Occupied seats to be emptied:\n{:?}", occupied_to_empty);
        
        if empty_to_occupied.is_empty() && occupied_to_empty.is_empty() {
            let count = input.iter()
                .flat_map(|row| row.iter())
                .filter(|&&ch| ch == '#')
                .count();
            println!("Seats taken: {}", count);
            break 'main;
        }

        if !empty_to_occupied.is_empty() {
            for (y, x) in empty_to_occupied.iter() {
                input.0[*y][*x] = '#';
            }
        }

        if !occupied_to_empty.is_empty() {
            for (y, x) in occupied_to_empty.iter() {
                input.0[*y][*x] = 'L';
            }
        }

        empty_to_occupied.clear(); 
        occupied_to_empty.clear();  
    }

    // part 2
    'main: loop {
        for (y, row) in input_2.iter().enumerate() {
            'chars: for (x, item) in row.iter().enumerate() {
                let mut seen_line_o_sight: Vec<char> = vec![];

                if *item == '.' {
                    continue 'chars;
                } 

                'offsets: for (y_offset, x_offset) in offsets.iter() {
                    let mut adj_y = y as isize;
                    let mut adj_x = x as isize;

                    'neigh: loop {
                        adj_y += *y_offset; 
                        adj_x += *x_offset; 
                        if in_bounds(&input_2, adj_y, adj_x) {
                            let neighbor = input_2[adj_y as usize][adj_x as usize];
                            if neighbor == '#' || neighbor == 'L' {
                                seen_line_o_sight.push(neighbor);
                                // println!("Chair in sight. y: {y} x: {x}, item: {item}, neighbor: {neighbor}");
                                continue 'offsets;
                            }
                        } else {
                            // println!("At edge, didn't see a chair.");
                            break 'neigh;
                        }
                    }
                }

                // println!("Neighbors at ({}, {}) ({}): {:?}", y, x, item, neighbors);
                
                let hashtags = seen_line_o_sight 
                    .iter()
                    .filter(|&x| *x == '#')
                    .count();

                if *item == 'L' && hashtags == 0 {
                    empty_to_occupied.push((y, x));
                } else if *item == '#' && hashtags >= 5 {
                    occupied_to_empty.push((y, x))
                } 
            } 
        }

        println!("{}", input_2);

        if empty_to_occupied.is_empty() && occupied_to_empty.is_empty() {
            let count = input_2.iter()
                .flat_map(|row| row.iter())
                .filter(|&&ch| ch == '#')
                .count();
            println!("Seats taken: {}", count);
            break 'main;
        }

        if !empty_to_occupied.is_empty() {
            for (y, x) in empty_to_occupied.iter() {
                input_2.0[*y][*x] = '#';
            }
        }

        if !occupied_to_empty.is_empty() {
            for (y, x) in occupied_to_empty.iter() {
                input_2.0[*y][*x] = 'L';
            }
        }

        empty_to_occupied.clear(); 
        occupied_to_empty.clear();  
    }
}

fn in_bounds(input: &CharGrid, y: isize, x: isize) -> bool {
    (0 <= y && y < input.len() as isize) && (0 <= x && x < input[0].len() as isize)
}
