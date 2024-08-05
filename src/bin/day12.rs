use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = include_str!("../../data/day12.txt")
        .lines()
        .collect();

    #[derive(Debug, Default)] 
    struct Ship<'a> {
        direction: i32,
        east_dist: u32,
        west_dist: u32,
        north_dist: u32,
        south_dist: u32,
        waypoint: Waypoint<'a>,
    }

    #[derive(Debug, Default)] 
    struct Waypoint<'a> {
        east_dist: i32,
        west_dist: i32,
        north_dist: i32,
        south_dist: i32,
        rotation_matrices: HashMap<&'a str, ((i8, i8), (i8, i8))>,
    }
   

    impl Ship<'_> {
        fn perform_instruction(&mut self, instruction: &str) {
            let dir = &instruction[..1];
            let amount: u32 = instruction[1..].parse().unwrap();
            
            if dir == "N" {
                self.north_dist += amount; 
            }

            if dir == "E" {
                self.east_dist += amount; 
            }

            if dir == "W" {
                self.west_dist += amount; 
            }

            if dir == "S" {
                self.south_dist += amount; 
            }

            if dir == "L" {
                self.direction = (self.direction + amount as i32) % 360;
            } else if dir == "R" {
                self.direction = ((self.direction - amount as i32)+ 360) % 360;
            }

            if dir == "F" {
                if self.direction == 90 {
                    self.north_dist += amount;
                }

                if self.direction == 0 {
                    self.east_dist += amount;
                }

                if self.direction == 180 {
                    self.west_dist += amount;
                }

                if self.direction == 270 {
                    self.south_dist += amount;
                }
            }
        }

        fn perform_instruction_2(&mut self, instruction: &str) {
            let dir = &instruction[..1];
            let amount: i32 = instruction[1..].parse().unwrap();

            if dir == "N" {
                self.waypoint.north_dist += amount;
            }

            if dir == "E" {
            // println!("dir: {}, amount: {}", dir, amount);
                self.waypoint.east_dist += amount; 
            }

            if dir == "W" {
                self.waypoint.west_dist += amount; 
            }

            if dir == "S" {
                self.waypoint.south_dist += amount; 
            }

            if dir == "L" || dir == "R" {
                
                // turn to cartesian coordinates
                let x: i32 = self.waypoint.east_dist - self.waypoint.west_dist;
                let y: i32 = self.waypoint.north_dist - self.waypoint.south_dist;

                let new_x: i32;
                let new_y: i32;

                if let Some(&((a, b), (c, d))) = self.waypoint.rotation_matrices.get(instruction) {
                    new_x = (a as i32 * x) + (b as i32 * y);
                    new_y = (c as i32 * x) + (d as i32 * y);
                } else {
                    panic!("Couldn't get rotation, sailor")
                }
               
                // zero out the current waypoint dist and apply newly rotated
                self.waypoint.east_dist = 0;
                self.waypoint.west_dist = 0;
                self.waypoint.south_dist = 0;
                self.waypoint.north_dist = 0;

                // Positive x becomes east_dist, negative x becomes west_dist
                // Positive y becomes north_dist, negative y becomes south_dist
                match new_x {
                    x if x > 0 => self.waypoint.east_dist = new_x.abs(),
                    x if x < 0 => self.waypoint.west_dist = new_x.abs(),
                    _ => println!("No change???"),
                }

                match new_y {
                    y if y > 0 => self.waypoint.north_dist = new_y.abs(),
                    y if y < 0 => self.waypoint.south_dist = new_y.abs(),
                    _ => println!("No change???"),
                }

                // println!("WAYPOINT OF SHIP: E: {}, W: {}, N: {}, S: {}\n",
                //     self.waypoint.east_dist,
                //     self.waypoint.west_dist,
                //     self.waypoint.north_dist,
                //     self.waypoint.south_dist);
            }

            if dir == "F" {
                if self.waypoint.east_dist > 0 {
                    self.east_dist += (self.waypoint.east_dist * amount) as u32;
                }

                if self.waypoint.west_dist > 0 {
                    self.west_dist += (self.waypoint.west_dist * amount) as u32;
                }

                if self.waypoint.north_dist > 0 {
                    self.north_dist += (self.waypoint.north_dist * amount) as u32;
                }

                if self.waypoint.south_dist > 0 {
                    self.south_dist += (self.waypoint.south_dist * amount) as u32;
                }

                // println!("SHIP: {}, {}, {}, {}\n", self.east_dist, self.west_dist, self.north_dist, self.south_dist);
            }
        }

        fn manhattan_distance(&self) -> i32 {
            (self.west_dist as i32 - self.east_dist as i32).abs() + (self.north_dist as i32 - self.south_dist as i32).abs()
        }
    }
    
    let mut ship = Ship::default();
    
    for item in &input {
        ship.perform_instruction(item);
    } 

    println!("Mahnattan distance: {} for ship: {:?}\n", ship.manhattan_distance(), ship);

    // rotation matrices
    let mut rotation_matrices: HashMap<&str, ((i8, i8), (i8, i8))> = HashMap::new(); 
    rotation_matrices.insert("R90", ((0, 1), (-1, 0)));
    rotation_matrices.insert("L90", ((0, -1), (1, 0)));
    rotation_matrices.insert("L180", ((-1, 0), (0, -1)));
    rotation_matrices.insert("R180", ((-1, 0), (0, -1)));
    rotation_matrices.insert("R270", ((0, -1), (1, 0)));
    rotation_matrices.insert("L270", ((0, 1), (-1, 0)));

    let mut ship_2 = Ship {
        waypoint: Waypoint {
            east_dist: 10,
            north_dist: 1,
            rotation_matrices,
            ..Default::default()
        },
        ..Default::default()
    };

    for item in input {
        ship_2.perform_instruction_2(item);
    } 

    println!("Mahnattan distance: {} for ship: {:?}\n", ship_2.manhattan_distance(), ship_2);
}
